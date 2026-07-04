use crate::schema::{FALSE, Schema, TRUE};
use crate::{Link, data::erased::ErasableData};

pub mod erased;
mod impls;
pub mod visitor;

pub use erased::{ErasableRequest, ErasedRequest, SelfReceiver};

pub use visitor::Visitor;

/// The `Request` trait defines the interface for receiving data during a query.
///
/// When a `Data` object is queried, it provides its internal structure (values and links)
/// to the given `Request`. The `Request` uses a `Schema` to filter incoming data, and a
/// `Visitor` to process accepted primitive values.
///
/// A common use case is using `Option<T>` as a request to extract a single primitive value.
///
/// # Examples
///
/// Extracting a string from a `Data` object:
/// ```rust
/// use datalink::{Data, Request};
///
/// let my_data = "Hello, world!";
/// let mut result = Option::<String>::None;
/// 
/// my_data.query(result.by_ref());
/// assert_eq!(result.unwrap(), "Hello, world!");
/// ```
pub trait Request {
    #[inline]
    fn query(&self) -> impl crate::Data {}

    #[inline]
    fn schema(&self) -> impl Schema {
        TRUE
    }

    fn visitor(&mut self) -> impl Visitor;

    #[inline]
    fn visitor_for<T: 'static>(&mut self) -> impl Visitor {
        self.visitor()
    }

    #[inline]
    fn provide_value_unchecked<T: 'static>(&mut self, value: T) {
        use visitor::VisitorExt;
        self.visitor_for::<T>().visit(value);
    }

    #[inline]
    fn provide_ref_unchecked<T: 'static>(&mut self, value: &T) {
        use visitor::VisitorExt;
        self.visitor_for::<&T>().visit_ref(value);
    }

    #[inline]
    fn provide_link_unchecked<L: Link>(&mut self, link: L) {
        debug_assert!(
            false,
            "Got link ({}) for request {} which doesn't take any",
            core::any::type_name_of_val(&link),
            core::any::type_name::<Self>(),
        );
    }

    #[inline]
    fn provide_value<T: 'static>(&mut self, value: T) {
        if self.schema().accepts_value(&value) {
            self.provide_value_unchecked(value);
        }
    }

    #[inline]
    fn provide_value_with<T: 'static>(&mut self, f: impl FnOnce() -> T) {
        if self.schema().accepts_value_of::<T>() {
            self.provide_value(f());
        }
    }

    #[inline]
    fn try_provide_value_with<T: 'static>(&mut self, f: impl FnOnce() -> Option<T>) {
        if self.schema().accepts_value_of::<T>()
            && let Some(val) = f()
        {
            self.provide_value(val);
        }
    }

    #[inline]
    fn provide_link<L: Link>(&mut self, link: L) {
        if self.would_accept_link(&link) {
            self.provide_link_unchecked(link);
        }
    }

    #[inline]
    fn provide_link_with<L: Link>(&mut self, f: impl FnOnce() -> L) {
        // dbg!("provide_link_with",core::any::type_name_of_val(&self.schema()), core::any::type_name::<L>());
        if !self.would_accept_link_type::<L>() {
            return;
        }
        self.provide_link(f());
    }

    #[inline]
    fn try_provide_link_with<L: Link>(&mut self, f: impl FnOnce() -> Option<L>) {
        if !self.would_accept_link_type::<L>() {
            return;
        }
        if let Some(link) = f() {
            self.provide_link(link);
        }
    }

    #[cfg(feature = "unique")]
    #[inline]
    fn provide_id(&mut self, _id: impl Into<crate::id::ID>) {
        // self.provide_with(|| id.into());
    }

    #[inline]
    fn provide_discriminant<T: 'static>(&mut self, _v: &T) {
        // use core::mem::discriminant;

        // self.provide_with(|| (IsPrimitive, discriminant(v)));
    }

    #[inline]
    fn by_ref(&mut self) -> impl Request {
        self
    }

    fn as_erased(&mut self) -> impl ErasableRequest;

    #[inline]
    fn provide_str(&mut self, value: &str) {
        // if self.would_accept_value_of::<&str>() {
        self.visitor_for::<&str>().visit_str(value);
        // }
    }

    #[inline]
    fn provide_str_with<'a>(&mut self, f: impl FnOnce() -> &'a str) {
        if self.would_accept_value_of::<&str>() {
            self.visitor_for::<&str>().visit_str(f());
        }
    }

    #[inline]
    fn try_provide_str_with<'a>(&mut self, f: impl FnOnce() -> Option<&'a str>) {
        if self.would_accept_value_of::<&str>()
            && let Some(val) = f()
        {
            self.visitor_for::<&str>().visit_str(val);
        }
    }

    #[inline]
    fn provide_bytes(&mut self, value: &[u8]) {
        if self.would_accept_value_of::<&[u8]>() {
            self.visitor_for::<&[u8]>().visit_bytes(value);
        }
    }

    #[inline]
    fn provide_bytes_with<'a>(&mut self, f: impl FnOnce() -> &'a [u8]) {
        if self.would_accept_value_of::<&[u8]>() {
            self.visitor_for::<&[u8]>().visit_bytes(f());
        }
    }

    #[inline]
    fn try_provide_bytes_with<'a>(&mut self, f: impl FnOnce() -> Option<&'a [u8]>) {
        if self.would_accept_value_of::<&[u8]>()
            && let Some(val) = f()
        {
            self.visitor_for::<&[u8]>().visit_bytes(val);
        }
    }
}

pub trait RequestExt: Request {
    #[inline]
    fn would_accept_value<T: 'static>(&self, value: &T) -> bool {
        self.schema().accepts_value(value)
    }

    #[inline]
    fn would_accept_value_of<T: 'static>(&self) -> bool {
        self.schema().accepts_value_of::<T>()
    }

    #[inline]
    fn would_accept_link<L: Link>(&self, link: &L) -> bool {
        // dbg!(core::any::type_name::<Self>(),core::any::type_name_of_val(&self.schema()), core::any::type_name_of_val(link));
        self.schema().accepts_link(link)
    }

    #[inline]
    fn would_accept_link_type<L: Link>(&self) -> bool {
        self.schema().accepts_link_type::<L>()
    }

    #[inline]
    fn provide_cow_with<'a, T: ToOwned + 'static>(
        &mut self,
        f: impl FnOnce() -> std::borrow::Cow<'a, T>,
    ) {
        use std::borrow::Cow;
        use visitor::VisitorExt;
        let schema = self.schema();

        let accepts_ref = schema.accepts_value_of::<&T>();
        let accepts_owned = schema.accepts_value_of::<T::Owned>();
        drop(schema);

        if accepts_ref || accepts_owned {
            match f() {
                // TODO: validate &str by value, not just type
                Cow::Borrowed(v) if accepts_ref => {
                    self.provide_ref_unchecked(v);
                    self.visitor_for::<&T>().visit_ref(v);
                }
                Cow::Owned(v) if accepts_owned => {
                    self.visitor_for::<T>().visit(v);
                }
                _ => {}
            }
        }
    }
}
impl<R: Request + ?Sized> RequestExt for R {}

#[deny(clippy::missing_trait_methods)]
impl<R: Request + ?Sized> Request for &mut R {
    #[inline]
    fn query(&self) -> impl crate::Data {
        (**self).query()
    }

    #[inline]
    fn schema(&self) -> impl Schema {
        (**self).schema()
    }

    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        (**self).visitor()
    }

    #[inline]
    fn visitor_for<T: 'static>(&mut self) -> impl Visitor {
        (**self).visitor_for::<T>()
    }

    #[inline]
    fn provide_link_unchecked<L: Link>(&mut self, link: L) {
        (**self).provide_link_unchecked(link);
    }

    #[inline]
    fn provide_link<L: Link>(&mut self, link: L) {
        (**self).provide_link(link);
    }

    #[inline]
    fn provide_discriminant<T: 'static>(&mut self, v: &T) {
        (**self).provide_discriminant(v);
    }

    #[cfg(feature = "unique")]
    #[inline]
    fn provide_id(&mut self, id: impl Into<crate::id::ID>) {
        (**self).provide_id(id);
    }

    #[inline]
    fn provide_link_with<L: Link>(&mut self, f: impl FnOnce() -> L) {
        (**self).provide_link_with(f);
    }

    #[inline]
    fn try_provide_link_with<L: Link>(&mut self, f: impl FnOnce() -> Option<L>) {
        (**self).try_provide_link_with(f);
    }

    #[inline]
    fn by_ref(&mut self) -> impl Request {
        (**self).by_ref()
    }

    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {
        (**self).as_erased()
    }

    #[inline]
    fn provide_value_unchecked<T: 'static>(&mut self, value: T) {
        (**self).provide_value_unchecked(value);
    }

    #[inline]
    fn provide_ref_unchecked<T: 'static>(&mut self, value: &T) {
        (**self).provide_ref_unchecked(value);
    }

    #[inline]
    fn provide_value<T: 'static>(&mut self, value: T) {
        (**self).provide_value(value);
    }

    #[inline]
    fn provide_value_with<T: 'static>(&mut self, f: impl FnOnce() -> T) {
        (**self).provide_value_with(f);
    }

    #[inline]
    fn try_provide_value_with<T: 'static>(&mut self, f: impl FnOnce() -> Option<T>) {
        (**self).try_provide_value_with(f);
    }

    #[inline]
    fn provide_str(&mut self, value: &str) {
        (**self).provide_str(value);
    }

    #[inline]
    fn provide_str_with<'a>(&mut self, f: impl FnOnce() -> &'a str) {
        (**self).provide_str_with(f);
    }

    #[inline]
    fn try_provide_str_with<'a>(&mut self, f: impl FnOnce() -> Option<&'a str>) {
        (**self).try_provide_str_with(f);
    }

    #[inline]
    fn provide_bytes(&mut self, value: &[u8]) {
        (**self).provide_bytes(value);
    }

    #[inline]
    fn provide_bytes_with<'a>(&mut self, f: impl FnOnce() -> &'a [u8]) {
        (**self).provide_bytes_with(f);
    }

    #[inline]
    fn try_provide_bytes_with<'a>(&mut self, f: impl FnOnce() -> Option<&'a [u8]>) {
        (**self).try_provide_bytes_with(f);
    }
}

impl Request for Option<Box<dyn ErasableData>> {
    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        // todo
    }

    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {
        self
    }
}

impl ErasableRequest for Option<Box<dyn ErasableData>> {
    #[inline]
    fn erased_visitor(&mut self) -> erased::ErasedVisitor {
        None
    }
    #[inline]
    fn erased_self_receiver(&mut self) -> Option<SelfReceiver> {
        if self.is_some() {
            return None;
        }

        Some(Box::new(move |data| {
            self.get_or_insert(data);
        }))
    }
}

impl Request for () {
    #[inline]
    fn schema(&self) -> impl Schema {
        FALSE
    }

    #[inline]
    fn visitor(&mut self) -> impl Visitor {}

    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {}

    #[inline]
    fn by_ref(&mut self) -> impl Request {}
}

impl ErasableRequest for () {
    #[inline]
    fn erased_visitor(&mut self) -> erased::ErasedVisitor {
        None
    }
    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        crate::schema::SimpleSchema::EMPTY
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, Request};

    #[test]
    fn providable() {
        let mut req = Option::<u8>::None;
        42u8.query(req.by_ref());
        assert_eq!(req, Some(42));

        let mut req = Option::<u8>::None;
        let r = &42u8;
        r.query(req.by_ref());
        assert_eq!(req, Some(42));

        let mut req = Option::<String>::None;
        "Hello world".query(req.by_ref());
        assert_eq!(req.unwrap(), "Hello world");
    }
}


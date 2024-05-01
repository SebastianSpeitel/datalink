use crate::data::Data;
use crate::links::{LinkError, Links};
use crate::rr::{meta, prelude::*};

impl Data for () {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        self.provide_requested(&mut request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        request.provide_owned(meta::IsUnit);
    }
}

macro_rules! impl_copy_data {
    ($ty:ty,$fn:ident) => {
        impl Data for $ty {
            #[inline]
            fn provide_value(&self, mut request: Request) {
                self.provide_requested(&mut request).debug_assert_provided();
            }
            #[inline]
            fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
                request.$fn(*self);
            }
        }
    };
}

impl_copy_data!(bool, provide_bool);
impl_copy_data!(u8, provide_u8);
impl_copy_data!(i8, provide_i8);
impl_copy_data!(u16, provide_u16);
impl_copy_data!(i16, provide_i16);
impl_copy_data!(u32, provide_u32);
impl_copy_data!(i32, provide_i32);
impl_copy_data!(u64, provide_u64);
impl_copy_data!(i64, provide_i64);
impl_copy_data!(u128, provide_u128);
impl_copy_data!(i128, provide_i128);
impl_copy_data!(f32, provide_f32);
impl_copy_data!(f64, provide_f64);
impl_copy_data!(char, provide_char);

impl Data for str {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        request.provide_str(self);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        request.provide_str(self);
    }
}

impl Data for &str {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        request.provide_str(self);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        request.provide_str(self);
    }
}

impl Data for [u8] {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        request.provide_bytes(self);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        request.provide_bytes(self);
    }
}

impl Data for &[u8] {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        request.provide_bytes(self);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        request.provide_bytes(self);
    }
}

impl Data for usize {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        self.provide_requested(&mut request).debug_assert_provided();
    }

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        if usize::BITS >= u128::BITS {
            request.provide_u128(*self as u128);
        }
        if usize::BITS >= u64::BITS {
            request.provide_u64(*self as u64);
        }
        if usize::BITS >= u32::BITS {
            request.provide_u32(*self as u32);
        }
        if usize::BITS >= u16::BITS {
            request.provide_u16(*self as u16);
        }
        if usize::BITS >= u8::BITS {
            request.provide_u8(*self as u8);
        }
    }
}

impl Data for isize {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        self.provide_requested(&mut request).debug_assert_provided();
    }

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        if isize::BITS >= i128::BITS {
            request.provide_i128(*self as i128);
        }
        if isize::BITS >= i64::BITS {
            request.provide_i64(*self as i64);
        }
        if isize::BITS >= i32::BITS {
            request.provide_i32(*self as i32);
        }
        if isize::BITS >= i16::BITS {
            request.provide_i16(*self as i16);
        }
        if isize::BITS >= i8::BITS {
            request.provide_i8(*self as i8);
        }
    }
}

macro_rules! impl_nonzero_data {
    ($ty:ty) => {
        impl Data for $ty {
            #[inline]
            fn provide_value(&self, mut request: Request) {
                if !self.provide_requested(&mut request).was_provided() {
                    self.get().provide_value(request);
                }
            }
            #[inline]
            fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
                self.get().provide_requested(request).was_provided()
            }
        }
    };
}

impl_nonzero_data!(core::num::NonZeroU8);
impl_nonzero_data!(core::num::NonZeroI8);
impl_nonzero_data!(core::num::NonZeroU16);
impl_nonzero_data!(core::num::NonZeroI16);
impl_nonzero_data!(core::num::NonZeroU32);
impl_nonzero_data!(core::num::NonZeroI32);
impl_nonzero_data!(core::num::NonZeroU64);
impl_nonzero_data!(core::num::NonZeroI64);
impl_nonzero_data!(core::num::NonZeroU128);
impl_nonzero_data!(core::num::NonZeroI128);
impl_nonzero_data!(core::num::NonZeroUsize);
impl_nonzero_data!(core::num::NonZeroIsize);

#[warn(clippy::missing_trait_methods)]
impl<D: Data> Data for Option<D> {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        if !self.provide_requested(&mut request).was_provided() {
            if let Some(d) = self {
                d.provide_value(request);
            }
        }
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        if let Some(d) = self {
            request.provide_owned(meta::IsSome);
            d.provide_requested(request).was_provided()
        } else {
            request.provide_owned(meta::IsNone);
            true
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        if let Some(d) = self {
            d.provide_links(links)?;
        }

        Ok(())
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        if let Some(d) = self {
            d.query_links(links, query)?;
        }

        Ok(())
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.as_ref().and_then(Data::get_id)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: ?Sized> Data for std::borrow::Cow<'_, D>
where
    for<'a> &'a D: Data,
    D: ToOwned,
    D::Owned: Data,
{
    #[inline]
    fn provide_value(&self, mut request: Request) {
        if !self.provide_requested(&mut request).was_provided() {
            match self {
                Self::Borrowed(data) => data.provide_value(request),
                Self::Owned(data) => data.provide_value(request),
            }
        }
    }

    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        match self {
            Self::Borrowed(data) => {
                request.provide_owned(meta::IsBorrowed);
                data.provide_requested(request).was_provided()
            }
            Self::Owned(data) => {
                request.provide_owned(meta::IsOwned);
                data.provide_requested(request).was_provided()
            }
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Self::Borrowed(data) => data.provide_links(links),
            Self::Owned(data) => data.provide_links(links),
        }
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        match self {
            Self::Borrowed(data) => data.query_links(links, query),
            Self::Owned(data) => data.query_links(links, query),
        }
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        match self {
            Self::Borrowed(data) => data.get_id(),
            Self::Owned(data) => data.get_id(),
        }
    }
}

#[cfg(feature = "unique")]
#[warn(clippy::missing_trait_methods)]
impl<D: crate::data::unique::Unique + ?Sized> crate::data::unique::Unique
    for std::borrow::Cow<'_, D>
where
    D: ToOwned,
    D::Owned: crate::data::unique::Unique,
{
    #[inline]
    fn id(&self) -> crate::id::ID {
        match self {
            Self::Borrowed(data) => data.id(),
            Self::Owned(data) => data.id(),
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data> Data for core::cell::OnceCell<D> {
    #[inline]
    fn provide_value(&self, request: Request) {
        self.get().provide_value(request);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        self.get().provide_requested(request).was_provided()
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        self.get().provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        self.get().query_links(links, query)
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.get().get_id()
    }
}

impl<D: Data> Data for std::sync::OnceLock<D> {
    #[inline]
    fn provide_value(&self, request: Request) {
        self.get().provide_value(request);
    }
    #[inline]
    fn provide_requested<R: Req>(&self, request: &mut Request<R>) -> impl Provided {
        self.get().provide_requested(request).was_provided()
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        self.get().provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        self.get().query_links(links, query)
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.get().get_id()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;

    #[test]
    fn once_cell() {
        let cell = core::cell::OnceCell::new();

        let values = cell.all_values();
        dbg!(&values);
        assert!(values.iter().find(|v| v.as_i32().is_some()).is_none());
        assert!(values.as_i32().is_none());

        cell.get_or_init(|| 100);
        let values = cell.all_values();
        dbg!(&values);

        assert!(values.iter().find(|v| v.as_i32().is_some()).is_some());
        assert!(values.as_i32().is_some());
    }
}

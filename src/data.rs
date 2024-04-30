use std::fmt::Debug;

use crate::{
    links::{LinkError, Links, LinksExt},
    value::{Req, ValueRequest},
};

#[cfg(feature = "unique")]
pub mod constant;
mod ext;
pub mod format;
mod impls;
#[cfg(feature = "unique")]
pub mod unique;

pub use ext::DataExt;
pub type BoxedData = Box<dyn Data>;

/// The core trait of this crate.
///
/// This trait is object-safe, so it can be used as a trait object.
/// ```rust
/// use datalink::prelude::*;
///
/// let heterogeneous: &[&dyn Data] = &[&1, &true];
///
/// for data in heterogeneous {
///    println!("{:?}", data);
/// }
/// ```
pub trait Data {
    #[allow(unused_variables)]
    #[inline]
    fn provide_value(&self, request: ValueRequest) {}

    #[allow(unused_variables)]
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        Ok(())
    }

    #[allow(unused_variables)]
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        use crate::query::LinkFilter;
        match query.filter() {
            LinkFilter::None => Ok(()),
            LinkFilter::Any => self.provide_links(links),
            filter => self.provide_links(&mut links.filter(filter)),
        }
    }

    #[cfg_attr(not(feature = "unique"), doc(hidden))]
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        None
    }

    #[inline]
    #[must_use]
    #[allow(unused_variables)]
    fn provide_requested<R: Req>(&self, request: &mut ValueRequest<R>) -> impl Provided
    where
        Self: Sized,
    {
        internal::DefaultImpl
    }
}

#[cfg(feature = "unique")]
impl<D: Data + ?Sized> PartialEq<D> for dyn Data {
    #[inline]
    fn eq(&self, other: &D) -> bool {
        match (self.get_id(), other.get_id()) {
            (Some(self_id), Some(other_id)) => self_id == other_id,
            _ => false,
        }
    }
}

impl Debug for dyn Data {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}
impl Debug for dyn Data + Sync {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}
impl Debug for dyn Data + Send {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}
impl Debug for dyn Data + Sync + Send {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}

mod internal {
    pub(super) struct DefaultImpl;
    impl super::Provided for DefaultImpl {
        #[inline]
        fn was_provided(&self) -> bool {
            false
        }
    }
}

pub trait Provided {
    #[inline]
    fn was_provided(&self) -> bool {
        true
    }

    #[inline]
    #[track_caller]
    fn assert_provided(&self) {
        assert!(self.was_provided());
    }

    #[inline]
    #[track_caller]
    fn debug_assert_provided(&self) {
        debug_assert!(self.was_provided());
    }
}

impl Provided for () {}

impl Provided for bool {
    #[inline]
    fn was_provided(&self) -> bool {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_d: &dyn Data) {}
    }
}

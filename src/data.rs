use std::fmt::Debug;

use crate::{
    links::{LinkError, Links, LinksExt},
    value::{Provided, ValueQuery, ValueRequest},
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
    fn provide_value(&self, request: &mut ValueRequest) {}

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
    fn provide_requested<Q: ValueQuery>(&self, request: &mut ValueRequest<Q>) -> impl Provided
    where
        Self: Sized,
    {
        crate::rr::provided::DefaultImpl
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_d: &dyn Data) {}
    }
}

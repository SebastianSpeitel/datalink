use std::fmt::Debug;

use crate::{
    link_builder::{LinkBuilder, LinkBuilderError},
    value::ValueBuiler,
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
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {}

    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) -> Result<(), LinkBuilderError> {
        builder.end()
    }

    #[allow(unused_variables)]
    #[inline]
    fn query_links(
        &self,
        builder: &mut dyn LinkBuilder,
        query: &crate::query::Query,
    ) -> Result<(), LinkBuilderError> {
        use crate::query::LinkSelector;
        match query.selector() {
            LinkSelector::None => builder.end(),
            LinkSelector::Any => self.provide_links(builder),
            _ => Err(LinkBuilderError::UnsupportedQuery),
        }
    }

    #[cfg_attr(not(feature = "unique"), doc(hidden))]
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        None
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

/// Marker trait to automatically implement `Data` for containers of this type
pub trait Primitive: Data {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_d: &dyn Data) {}
    }
}

use crate::data::unique::Unique;
use crate::data::{format, Data, DataExt, Primitive};
use crate::id::ID;
use crate::link_builder::{LinkBuilder, LinkBuilderError as LBE};
use crate::value::ValueBuiler;

/// Wrapper for data with compile-time constant ID
///
/// Useful in situations where you know the ID of a datum at compile-time so you don't have to store it
///
/// ```rust
/// use std::mem::size_of_val;
/// use datalink::data::{Data, constant::Const};
///
/// pub const ROOT: Const<1234> = Const::empty();
///
/// assert!(ROOT.get_id().is_some());
/// assert_eq!(size_of_val(&ROOT), 0);
/// ```
///
/// # Safety
/// The given `ID` must be non-zero to be able to use it as `NonZeroU128`
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Const<const ID: u128, D: Data + ?Sized = ()>(D);

impl<const I: u128, D: Data + ?Sized> Const<I, D> {
    #[inline]
    pub const fn new(data: D) -> Self
    where
        D: Sized,
    {
        Self(data)
    }
}

impl<const I: u128, D: Data> Default for Const<I, D>
where
    D: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<const I: u128, D: Data + ?Sized> std::fmt::Debug for Const<I, D> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}

impl<const I: u128, D: Data> From<D> for Const<I, D> {
    #[inline]
    fn from(value: D) -> Self {
        Self::new(value)
    }
}

impl<const I: u128, S: Data + ?Sized, O: Data + ?Sized> PartialEq<O> for Const<I, S> {
    #[inline]
    fn eq(&self, other: &O) -> bool {
        other.get_id().is_some_and(|id| id == self.id())
    }
}
impl<const I: u128, D: Data + ?Sized> Eq for Const<I, D> {}

#[warn(clippy::missing_trait_methods)]
impl<const I: u128, D: Data + ?Sized> Data for Const<I, D> {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        self.0.provide_value(builder)
    }

    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) -> Result<(), LBE> {
        self.0.provide_links(builder)
    }

    #[inline]
    fn query_links(
        &self,
        builder: &mut dyn LinkBuilder,
        query: &crate::query::Query,
    ) -> Result<(), LBE> {
        self.0.query_links(builder, query)
    }

    #[inline(always)]
    fn get_id(&self) -> Option<ID> {
        std::num::NonZeroU128::new(I).map(ID::new)
    }
}
impl<const I: u128, D: Data + ?Sized> Primitive for Const<I, D> where D: Primitive {}
impl<const I: u128, D: Data + ?Sized> Unique for Const<I, D> {
    #[inline]
    fn id(&self) -> ID {
        ID::new(unsafe { std::num::NonZeroU128::new_unchecked(I) })
    }
}

impl<const I: u128> Const<I, ()> {
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_id() {
        let empty = Const::<123>::empty();

        assert!(empty.get_id().is_some());
    }

    #[test]
    fn transparent_size() {
        use std::mem::size_of_val;
        let data = vec![1, 2, 3];
        let wrapped = Const::<345, _>::new(data.clone());

        assert_eq!(size_of_val(&data), size_of_val(&wrapped));
    }

    #[test]
    fn empty_is_zst() {
        use std::mem::size_of_val;
        let empty = Const::<123>::empty();

        assert_eq!(size_of_val(&empty), 0);
    }

    #[test]
    fn usable_as_const() {
        const DATA: Const<123> = Const::empty();

        assert_eq!(DATA.id(), "123".parse().unwrap())
    }
}

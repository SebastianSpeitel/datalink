use crate::data::unique::Unique;
use crate::data::{format, Data, DataExt};
use crate::id::ID;
use crate::links::{LinkError, Links};
use crate::rr::Req;
use crate::value::ValueRequest;

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
/// assert!(datalink::Data::<datalink::value::Unknown>::get_id(&ROOT).is_some());
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
impl<const I: u128, R: Req, D: Data + ?Sized> Data<R> for Const<I, D> {
    #[inline]
    fn provide_value<'d>(&self, mut request: ValueRequest<'d, R>) {
        let request = ValueRequest::new(&mut request.0 as &mut dyn crate::rr::Receiver);
        self.0.provide_value(request)
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        self.0.provide_links(links)
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        self.0.query_links(links, query)
    }

    #[inline(always)]
    fn get_id(&self) -> Option<ID> {
        ID::try_new(I).ok()
    }
}
impl<const I: u128, D: Data + ?Sized> Unique for Const<I, D> {
    #[inline]
    fn id(&self) -> ID {
        debug_assert_ne!(I, 0, "ID must be non-zero");
        unsafe { ID::new_unchecked(I) }
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

        assert!(Data::<crate::rr::Unknown>::get_id(&empty).is_some());
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

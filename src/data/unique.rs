use std::borrow::Borrow;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::data::Data;
use crate::id::ID;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing ID")]
    MissingID,
}

/// Trait for marking something as uniquely identifyable by it's ID
pub trait Unique {
    fn id(&self) -> ID;
}

/// Wrapper for Data with or without an `ID` to make it always `Unique`
pub struct Fixed<D: Data + ?Sized, T: Borrow<D> = D> {
    data: T,
    id: ID,
    pd: PhantomData<D>,
}

impl<D: Data + ?Sized, T: Borrow<D>> Fixed<D, T> {
    /// Try to construct a Fixed by trying to get the ID from the data
    #[inline]
    pub fn try_new(data: T) -> Result<Self, Error> {
        let id = data.borrow().get_id().ok_or(Error::MissingID)?;
        Ok(Self {
            data,
            id,
            pd: PhantomData,
        })
    }

    /// Construct an `Fixed` with the given fallback ID if the data doesn't provide one
    #[inline]
    #[must_use]
    pub fn new(data: T, id: ID) -> Self {
        let id = data.borrow().get_id().unwrap_or(id);
        Self {
            data,
            id,
            pd: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn new_with(data: T, f: impl FnOnce() -> ID) -> Self {
        let id = data.borrow().get_id().unwrap_or_else(f);
        Self {
            data,
            id,
            pd: PhantomData,
        }
    }

    #[inline]
    #[cfg(feature = "random")]
    pub fn new_random(data: T) -> Self {
        Self::new_with(data, rand::random)
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> std::fmt::Debug for Fixed<D, T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = super::format::DataFormatter::<_>::new(f);
        self.data.borrow().query(&mut formatter);
        formatter.finish()
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> AsRef<D> for Fixed<D, T> {
    /// Returns a reference to the underlying data
    #[inline]
    fn as_ref(&self) -> &D {
        self.data.borrow()
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data + ?Sized, T: Borrow<D>> Data for Fixed<D, T> {
    #[inline]
    fn query(&self, request: &mut impl crate::Request) {
        request.provide_id(self.id);
        self.data.borrow().query(request);
    }

    #[inline]
    fn query_owned(self, request: &mut impl crate::Request)
    where
        Self: Sized,
    {
        request.provide_id(self.id);
        self.data.borrow().query(request);
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        Some(self.id)
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> Unique for Fixed<D, T> {
    #[inline]
    fn id(&self) -> ID {
        #[cfg(debug_assertions)]
        if let Some(id) = self.as_ref().get_id() {
            debug_assert_eq!(id, self.id);
        }

        self.id
    }
}

impl<D: Data + ?Sized, T: Borrow<D>, O: Data + ?Sized> PartialEq<O> for Fixed<D, T> {
    #[inline]
    fn eq(&self, other: &O) -> bool {
        other.get_id().is_some_and(|id| id == self.id)
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> Eq for Fixed<D, T> {}

impl<D: Data + ?Sized, T: Borrow<D>> Hash for Fixed<D, T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub trait MaybeUnique: Data + Sized {
    #[inline]
    fn try_into_unique(self) -> Result<Fixed<Self, Self>, Error> {
        Fixed::try_new(self)
    }

    #[inline]
    #[must_use]
    fn into_unique(self, id: ID) -> Fixed<Self, Self> {
        Fixed::new(self, id)
    }

    #[inline]
    #[must_use]
    fn into_unique_with(self, f: impl FnOnce() -> ID) -> Fixed<Self, Self> {
        Fixed::new_with(self, f)
    }

    #[cfg(feature = "random")]
    #[inline]
    #[must_use]
    fn into_unique_random(self) -> Fixed<Self, Self> {
        Fixed::new_random(self)
    }
}

/// Extension trait for `Data` to add methods for creating `Unique` data
impl<D: Data> MaybeUnique for D {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_ref() {
        let data = true;
        let res = Fixed::<bool, _>::try_new(&data);
        assert!(res.is_err());
    }
}

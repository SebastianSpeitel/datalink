use std::borrow::Borrow;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::data::{format, Data, DataExt};
use crate::id::ID;
use crate::links::{LinkError, Links};
use crate::rr::Request;

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
        self.data.borrow().format::<format::DEBUG>().fmt(f)
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
    fn provide_value(&self, request: Request) {
        self.as_ref().provide_value(request);
    }
    #[inline]
    fn provide_requested<'d, R: crate::rr::Req>(
        &self,
        _request: &mut Request<'d, R>,
    ) -> impl super::Provided
    where
        Self: Sized,
    {
        super::internal::DefaultImpl
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        self.as_ref().provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        self.as_ref().query_links(links, query)
    }
    #[inline]
    fn get_id(&self) -> Option<ID> {
        #[cfg(debug_assertions)]
        if let Some(id) = self.as_ref().get_id() {
            debug_assert_eq!(id, self.id);
        }

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

impl<D: Data + ?Sized, T: Borrow<D>> PartialEq for Fixed<D, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

// #[derive(Debug)]
// pub struct UniqueBuilder<D: Data, const SET: bool> {
//     data: D,
//     id_provided: bool,
//     id: Option<ID>,
// }

// impl<D: Data, const S: bool> UniqueBuilder<D, S> {
//     #[inline]
//     pub fn new(data: D) -> UniqueBuilder<D, false> {
//         let id_provided = data.id().is_some();
//         UniqueBuilder {
//             data,
//             id_provided,
//             id: None,
//         }
//     }
// }

// impl<D: Data> UniqueBuilder<D, false> {
//     #[inline]
//     pub fn or(self, id: ID) -> UniqueBuilder<D, true> {
//         if self.id_provided {
//             UniqueBuilder {
//                 data: self.data,
//                 id_provided: true,
//                 id: None,
//             }
//         } else {
//             UniqueBuilder {
//                 data: self.data,
//                 id_provided: false,
//                 id: Some(id),
//             }
//         }
//     }

//     #[inline]
//     pub fn or_else(self, f: impl FnOnce() -> ID) -> UniqueBuilder<D, true> {
//         if self.id_provided {
//             UniqueBuilder {
//                 data: self.data,
//                 id_provided: true,
//                 id: None,
//             }
//         } else {
//             UniqueBuilder {
//                 data: self.data,
//                 id_provided: false,
//                 id: Some(f()),
//             }
//         }
//     }

//     // todo create own resulttype with methods like
//     // as_dyn() -> Option<BoxedData>
//     // enum {
//     //    Err(E)
//     //    Provided(D)
//     //    WithId(WithID<D>)
//     // }
//     // Deref<dyn Data>??
//     // Result<MaybeUnique,E>
//     // MaybeUnique = Deref<dyn Data>/ToOwned<BoxedData>
//     #[inline]
//     pub fn build(self) -> Result<BoxedData, UniqueError>
//     where
//         D: 'static,
//     {
//         let data = if self.id_provided {
//             Box::new(self.data) as BoxedData
//         } else {
//             let id = self.id.ok_or(UniqueError::MissingID)?;
//             Box::new(WithID {
//                 data: self.data,
//                 id,
//             })
//         };
//         Ok(data)
//     }
// }

// struct WithID<D: Data> {
//     data: D,
//     id: ID,
// }

// impl<D: Data> Data for WithID<D> {
//     #[inline]
//     fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
//         self.data.provide_value(builder)
//     }
//     #[inline]
//     fn provide_links(&self, builder: &mut dyn LinkBuilder) {
//         self.data.provide_links(builder)
//     }
//     #[inline]
//     fn id(&self) -> Option<ID> {
//         Some(self.id)
//     }
// }
// impl<D: PrimitiveData> PrimitiveData for WithID<D> {}

// /// Wrapper for unique data with fixed ID.
// ///
// /// ```rust
// /// use datalink::prelude::*;
// /// use std::num::NonZeroU128;
// ///
// /// let data = "Hello, world!";
// /// let unique = Unique::new(&data, NonZeroU128::MIN);
// ///
// /// assert!(Data::id(&unique).is_some());
// /// ```
// #[derive(Clone, Debug)]
// pub struct Unique<'d, D: Data + ?Sized> {
//     id: ID,
//     data: &'d D,
// }

// impl<'d, D: Data + ?Sized> Unique<'d, D> {
//     /// Creates a new `Unique` by first checking if the data has an ID, and if not, using the provided ID.
//     ///
//     /// ```rust
//     /// use datalink::prelude::*;
//     /// use std::num::NonZeroU128;
//     ///
//     /// let data = "Hello, world!";
//     /// let unique = Unique::new(&data, NonZeroU128::MIN);
//     ///
//     /// assert_eq!(Data::id(&unique), Some(NonZeroU128::MIN.into()));
//     /// ```
//     #[inline]
//     pub fn new(data: &'d D, id: impl Into<ID>) -> Self {
//         let id = data.id().unwrap_or_else(|| id.into());
//         Self { id, data }
//     }

//     #[inline]
//     pub fn new_with(data: &'d D, f: impl FnOnce() -> ID) -> Self {
//         let id = data.id().unwrap_or_else(f);
//         Self { id, data }
//     }

//     /// Creates a new `Unique` by first checking if the data has an ID, and if not, generating a random ID.
//     ///
//     /// ```rust
//     /// use datalink::prelude::*;
//     ///
//     /// let data = "Hello, world!";
//     /// let unique = Unique::new_random(&data);
//     ///
//     /// assert!(Data::id(&unique).is_some());
//     /// ```
//     #[inline]
//     #[cfg(feature = "random")]
//     pub fn new_random(data: &'d D) -> Self {
//         let id = data.id().unwrap_or_else(rand::random);
//         Self { id, data }
//     }

//     /// Tries to create a new `Unique` by first checking if the data has an ID, and if not, returning an error.
//     ///
//     /// ```rust
//     /// use datalink::prelude::*;
//     ///
//     /// // &str does not provide an ID
//     /// let data = "Hello, world!";
//     ///
//     /// assert!(Unique::try_from(&data).is_err());
//     /// ```
//     #[inline]
//     pub fn try_from(data: &'d D) -> Result<Self, UniqueError> {
//         let id = data.id().ok_or(UniqueError::MissingID)?;
//         Ok(Self { id, data })
//     }

//     /// Creates a new `Unique` with the given ID.
//     ///
//     /// Note:
//     /// Ignores the ID provided by the data.
//     ///
//     /// ```rust
//     /// use datalink::prelude::*;
//     /// use std::num::NonZeroU128;
//     ///
//     /// const HELLO_ID: ID = ID::new(NonZeroU128::MIN);
//     /// const HELLO: Unique<str> = Unique::const_new("Hello, world!", HELLO_ID);
//     ///
//     /// assert_eq!(Data::id(&HELLO), Some(NonZeroU128::MIN.into()));
//     /// ```
//     #[inline]
//     pub const fn const_new(data: &'d D, id: ID) -> Self {
//         Self { id, data }
//     }

//     #[inline]
//     #[must_use]
//     pub const fn id(&self) -> ID {
//         self.id
//     }

//     #[inline]
//     pub const fn into_inner(self) -> &'d D {
//         self.data
//     }
// }

// #[warn(clippy::missing_trait_methods)]
// impl<D: Data + ?Sized> Data for Unique<'_, D> {
//     #[inline(always)]
//     fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
//         self.data.provide_value(builder)
//     }
//     #[inline(always)]
//     fn provide_links(&self, builder: &mut dyn LinkBuilder) {
//         self.data.provide_links(builder)
//     }
//     #[inline]
//     fn id(&self) -> Option<ID> {
//         Some(self.id)
//     }
// }

// impl<D: Data + ?Sized> PrimitiveData for Unique<'_, D> where D: PrimitiveData {}

// impl<D: Data + ?Sized> AsRef<D> for Unique<'_, D> {
//     #[inline]
//     fn as_ref(&self) -> &D {
//         self.data
//     }
// }

// impl<S: Data + ?Sized, O: Data + ?Sized> PartialEq<O> for Unique<'_, S> {
//     #[inline]
//     fn eq(&self, other: &O) -> bool {
//         match other.id() {
//             Some(oid) => oid == self.id,
//             _ => false,
//         }
//     }
// }

// impl<D: Data + ?Sized> Eq for Unique<'_, D> {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn small_id() {
//         use std::mem::size_of;
//         assert!(size_of::<ID>() <= 16);
//     }

//     #[test]
//     fn small_opt_id() {
//         use std::mem::size_of;
//         assert!(size_of::<Option<ID>>() <= 16);
//     }

//     #[test]
//     #[cfg(feature = "random")]
//     fn new_random() {
//         let s1 = "Hello, world!";
//         let u = Unique::new_random(&s1);
//         assert_eq!(u.as_ref(), &s1);

//         let u2 = Unique::new_random(&s1);
//         assert_eq!(u2.as_ref(), &s1);

//         assert_ne!(u.id(), u2.id());
//     }
// }

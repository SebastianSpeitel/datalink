use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::data::{format, Data, DataExt};
use crate::id::ID;
use crate::links::{LinkError, Links};
use crate::value::ValueBuiler;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing ID")]
    MissingID,
}

/// Trait for marking `Data` as uniquely identifyable by it's ID
pub trait Unique: Data {
    fn id(&self) -> ID;
}

/// Wrapper for Data with or without an `ID` to make it always `Unique`
pub enum AlwaysUnique<D: Data + ?Sized, T: Borrow<D>> {
    Implicit { data: T, phantom: PhantomData<D> },
    Explicit { data: T, id: ID },
}

impl<D: Data + ?Sized, T: Borrow<D>> AlwaysUnique<D, T> {
    /// Try to construct an implicit unique data by checking if the data provides an ID
    #[inline]
    pub fn try_new_implicit(data: T) -> Result<Self, Error> {
        if data.borrow().get_id().is_some() {
            Ok(Self::Implicit {
                data,
                phantom: PhantomData,
            })
        } else {
            Err(Error::MissingID)
        }
    }

    /// Construct an `AlwaysUnique` with the given fallback ID if the data doesn't provide one
    #[inline]
    #[must_use]
    pub fn new(data: T, id: ID) -> Self {
        if data.borrow().get_id().is_some() {
            Self::Implicit {
                data,
                phantom: PhantomData,
            }
        } else {
            Self::Explicit { data, id }
        }
    }

    #[inline]
    #[must_use]
    pub fn new_with(data: T, f: impl FnOnce() -> ID) -> Self {
        if data.borrow().get_id().is_some() {
            Self::Implicit {
                data,
                phantom: PhantomData,
            }
        } else {
            Self::Explicit { data, id: f() }
        }
    }

    #[inline]
    #[cfg(feature = "random")]
    pub fn new_random(data: T) -> Self {
        Self::new_with(data, rand::random)
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> std::fmt::Debug for AlwaysUnique<D, T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format::<format::DEBUG>().fmt(f)
    }
}

impl<D: Data + ?Sized, T: Borrow<D>> AsRef<D> for AlwaysUnique<D, T> {
    /// Returns a reference to the underlying data
    #[inline]
    fn as_ref(&self) -> &D {
        match self {
            Self::Implicit { data, .. } | Self::Explicit { data, .. } => data.borrow(),
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data + ?Sized, T: Borrow<D>> Data for AlwaysUnique<D, T> {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        self.as_ref().provide_value(builder)
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
        self.id().into()
    }
}
impl<D: Data + ?Sized, T: Borrow<D>> Unique for AlwaysUnique<D, T> {
    #[inline]
    fn id(&self) -> ID {
        match self {
            AlwaysUnique::Implicit { data, .. } => {
                let id = data.borrow().get_id();
                match id {
                    Some(id) => id,
                    None => unreachable!(),
                }
            }
            AlwaysUnique::Explicit { id, .. } => *id,
        }
    }
}

impl<D: Data + ?Sized, T: Borrow<D>, O: Data> PartialEq<O> for AlwaysUnique<D, T> {
    #[inline]
    fn eq(&self, other: &O) -> bool {
        other.get_id().is_some_and(|id| id == self.id())
    }
}
impl<D: Data + ?Sized, T: Borrow<D>> Eq for AlwaysUnique<D, T> {}

pub trait MaybeUnique: Data + Sized {
    #[inline]
    fn try_into_unique(self) -> Result<AlwaysUnique<Self, Self>, Error> {
        if self.get_id().is_some() {
            Ok(AlwaysUnique::Implicit {
                data: self,
                phantom: PhantomData,
            })
        } else {
            Err(Error::MissingID)
        }
    }

    #[inline]
    #[must_use]
    fn into_unique(self, id: ID) -> AlwaysUnique<Self, Self> {
        AlwaysUnique::new(self, id)
    }

    #[inline]
    #[must_use]
    fn into_unique_with(self, f: impl FnOnce() -> ID) -> AlwaysUnique<Self, Self> {
        AlwaysUnique::new_with(self, f)
    }

    #[cfg(feature = "random")]
    #[inline]
    #[must_use]
    fn into_unique_random(self) -> AlwaysUnique<Self, Self> {
        AlwaysUnique::new_random(self)
    }
}
impl<D: Data> MaybeUnique for D {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_ref() {
        let data = true;
        let res = AlwaysUnique::<bool, _>::try_new_implicit(&data);
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

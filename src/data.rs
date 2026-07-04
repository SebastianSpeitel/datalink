//! This module defines the core `Data` trait, which is the fundamental building block for representing and querying data within the Datalink library.
//!
//! The `Data` trait allows any type to expose its internal structure as a collection of values and links, enabling a unified approach to data access and validation.

use crate::Request;

#[cfg(feature = "unique")]
pub mod constant;
pub mod erased;
pub mod ext;
#[cfg(feature = "foreign")]
pub mod foreign;
pub mod format;
pub mod impls;
#[cfg(feature = "unique")]
pub mod unique;

/// The core trait of this crate.
///
/// Anything can be Data.
/// Data is represented as a collection of links,
/// where the key and target are Data themselves.
///
/// Implementors of this trait define how their internal data is exposed to a `Request`.
/// This allows for a flexible and extensible way to query and validate diverse data structures.
///
/// # Examples
///
/// Implementing `Data` for a custom struct:
///
/// ```rust
/// use datalink::{Data, Request};
///
/// struct MyStruct {
///     name: String,
///     age: u32,
/// }
///
/// impl Data for MyStruct {
///     fn query(&self, mut request: impl Request) {
///         // Provide links representing the struct's fields
///         request.provide_link(("name", self.name.clone()));
///         request.provide_link(("age", self.age));
///     }
/// }
///
/// let my_data = MyStruct { name: "Alice".into(), age: 30 };
/// // Here we use an Option as a Request to query the data if needed.
/// ```
pub trait Data {
    /// Queries the data, providing its internal structure to the given `request`.
    ///
    /// Implementors should call the appropriate `request.provide_...` methods
    /// to expose their values and links.
    fn query(&self, request: impl Request);

    #[inline]
    /// Queries the data, consuming `self`.
    ///
    /// This is a convenience method that calls `query`.
    fn query_owned(self, request: impl Request)
    where
        Self: Sized,
    {
        self.query(request);
    }

    #[inline]
    #[deprecated]
    /// Attempts to retrieve the unique ID of this data object.
    ///
    /// This method queries the data with a specific request designed to capture an `ID`,
    /// if one is provided by the data.
    fn get_id(&self) -> Option<crate::id::ID> {
        let mut req = None;
        self.query(req.by_ref());
        req
    }
}

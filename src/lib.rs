//! # Datalink Library
//!
//! Datalink is a Rust library designed for flexible and extensible data querying, validation, and schema definition. It provides a powerful mechanism to interact with data regardless of its underlying structure, focusing on a "query-based" approach rather than direct data access.
//!
//! ## Core Concepts
//!
//! At its heart, Datalink revolves around a few fundamental traits and concepts:
//!
//! 1.  **`Data` Trait:** The most central trait. Any type that implements `Data` can be queried. Data is conceptualized as a collection of values and links.
//! 2.  **`Request` Trait:** Used to extract specific information from `Data` objects. When a `Data` object is queried, it "provides" its internal data to a `Request` object. `Request`s can filter, collect, or validate this provided data.
//! 3.  **`Visitor` Trait:** A sub-component of `Request` that defines how different types of primitive values (integers, strings, booleans, etc.) are "visited" or processed during a query.
//! 4.  **Schema System:** A robust and composable system for defining and validating the structure and content of `Data` objects. Schemas are themselves `DataValidator`s.
//!
//! ## Data Representation and Querying
//!
//! ### `Data` Trait
//!
//! The `Data` trait is implemented by various types, allowing them to expose their internal structure in a uniform way.
//!
//! ```rust
//! use datalink::Request;
//! pub trait Data {
//!     fn query(&self, request: impl Request);
//!     // ... other methods like query_owned, get_id
//! }
//! ```
//!
//! When `query` is called, the `Data` implementation iterates over its internal values and links, calling corresponding `provide_` methods on the `Request` object.
//!
//! **Examples of `Data` Implementations:**
//! *   **Primitives:** `u32`, `String`, `bool`, etc., provide themselves as values.
//! *   **Collections:** `Vec<T>`, `HashMap<K, V>`, `[T; N]`, etc., provide their elements as values or links (e.g., `Vec` provides indexed links `(index, value)`).
//! *   **`ErasedData`:** A type-erased container for any `Data` object, useful for heterogeneous collections.
//! *   **`Link`:** Represents a directed relationship between two `Data` objects (a key and a target).
//!
//! ### `Request` and `Visitor`
//!
//! The `Request` trait defines the interface for receiving data during a query. It delegates the actual processing of individual data items to an associated `Visitor`.
//!
//! A common pattern is to use `Option<T>` as a `Request` to extract a single value of type `T`, or `Vec<Link>` to collect all links.
//!
//! ## Schema Definition and Validation
//!
//! Datalink's schema system is highly composable, allowing complex validation rules to be built from simpler ones using logical `and`, `or`, and `not` operations.
//!
//! ### Schema Types
//!
//! There are four primary schema types, each with its own `Validator` trait and `IntoSchema` trait:
//!
//! *   **`DataSchema` (`DataValidator`):** Validates an entire `Data` object, encompassing both its value and its links.
//! *   **`LinkSchema` (`LinkValidator`):** Validates a `Link` object, checking its key and target.
//! *   **`ValueSchema` (`ValueValidator`):** Validates a single value, checking its type and optionally a constant value.
//! *   **`TypeSchema` (`TypeValidator`):** Validates a `TypeId` or a type itself.
//!
//! ### Schema Composition (`Composition` Enum)
//!
//! The `Composition` enum is central to combining schemas. It allows you to express logical relationships:
//!
//! *   `Composition::Intersection(I)`: All schemas in `I` must be satisfied (logical AND).
//! *   `Composition::Union(U)`: At least one schema in `U` must be satisfied (logical OR).
//! *   `Composition::Not(N)`: The schema `N` must *not* be satisfied (logical NOT).
//! *   `Composition::Simple(S)`: A basic, uncomposed schema.
//!
//! ### Schema Builder Pattern
//!
//! The `src/schema/builder.rs` module provides a convenient builder pattern for constructing complex schemas. These builders implement the `SchemaExt` trait, offering methods like `and()`, `or()`, `has_value()`, `has_links()`, `has_type()`, `has_key()`, `has_target()`, and `equals()`.
//!
//! **Key Builder Structs:**
//!
//! *   **`HasValue<T>`:** Requires the data to have a value that satisfies schema `T`.
//! *   **`HasLinks<L>`:** Requires the data to have links that satisfy schema `L`. `L` is typically a `LinkSchema` or a tuple of `LinkSchema`s.
//! *   **`HasType<T>`:** Requires the data's value to be of a specific type `T`.
//! *   **`Equals<T>`:** Requires the data's value to be equal to a specific constant `T`.
//! *   **`HasKey<T>`:** Used within `LinkSchema` to validate the key of a link.
//! *   **`HasTarget<T>`:** Used within `LinkSchema` to validate the target of a link.
//!
//! ## Utilities
//!
//! *   **`Snek` Trait:** A custom iterator-like trait used internally for processing heterogeneous lists of schemas (e.g., in `Composition::Intersection`).
//! *   **`IntoTrait`:** A helper trait for converting types into their corresponding schema markers (e.g., `DataMarker`, `LinkMarker`).
//! *   **`TRUE` and `FALSE`:** Constants representing universal (always true) and impossible (always false) schemas, respectively.
//!
//! This documentation provides a high-level overview. For detailed usage and specific API calls, refer to the inline documentation within the source code.
#![warn(
    missing_debug_implementations,
    unreachable_pub,
    clippy::unwrap_used,
    clippy::missing_inline_in_public_items,
    clippy::std_instead_of_core,
    // clippy::std_instead_of_alloc
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::wildcard_imports,
    mismatched_lifetime_syntaxes
)] // remove for prod

pub mod adapter;
pub mod data;
pub mod link;
pub mod meta;
pub mod request;
pub mod schema;
pub mod r#type;
pub mod util;
pub mod value;
#[cfg(feature = "well_known")]
pub mod well_known;
#[cfg(feature = "derive")]
pub use datalink_derive::Data;

#[cfg_attr(not(feature = "unique"), doc(hidden))]
pub mod id;

#[cfg(feature = "foreign")]
pub use data::foreign::{Foreign, ForeignData, QueryFn};
pub use data::{
    Data,
    erased::{ErasableData, ErasedData},
    ext::DataExt,
};
pub use link::{Link, LinkBuilder};
pub use request::{Request, RequestExt, Visitor, visitor::VisitorExt};
pub use r#type::{Type, TypeOf};

pub mod prelude {
    #[cfg(feature = "unique")]
    pub use crate::data::unique::{MaybeUnique, Unique};
    pub use crate::data::{Data, erased::ErasedData};
    #[cfg(feature = "unique")]
    pub use crate::id::ID;
    pub use crate::link::{Link, LinkBuilder};
    pub use crate::request::{Request, RequestExt, Visitor};
    #[cfg(feature = "foreign")]
    pub use crate::{Foreign, ForeignData, QueryFn};
    #[cfg(feature = "derive")]
    pub use datalink_derive::Data;
}

// Show which crate feature enables conditionally compiled APIs in documentation.
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    unreachable_pub,
    clippy::unwrap_used,
    clippy::missing_inline_in_public_items
)]
#![allow(clippy::module_name_repetitions)]

pub mod data;
pub mod link_builder;
pub mod query;
pub mod value;
#[cfg(feature = "well_known")]
pub mod well_known;
#[cfg(feature = "derive")]
pub use datalink_derive::Data;

#[cfg_attr(not(feature = "unique"), doc(hidden))]
pub mod id;

pub use data::{BoxedData, Data};

pub mod prelude {
    #[cfg(feature = "unique")]
    pub use crate::data::unique::{MaybeUnique, Unique};
    pub use crate::data::BoxedData;
    pub use crate::data::Data;
    #[cfg(feature = "unique")]
    pub use crate::id::ID;
}

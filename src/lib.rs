#![warn(
    missing_debug_implementations,
    unreachable_pub,
    clippy::unwrap_used,
    clippy::missing_inline_in_public_items
)]
#![allow(clippy::module_name_repetitions, clippy::default_trait_access)]
#![allow(clippy::missing_errors_doc, clippy::use_self)] // remove for prod

pub mod data;
pub mod link;
pub mod meta;
pub mod query;
pub mod request;
pub mod value;
#[cfg(feature = "well_known")]
pub mod well_known;
#[cfg(feature = "derive")]
pub use datalink_derive::Data;

#[cfg_attr(not(feature = "unique"), doc(hidden))]
pub mod id;

pub use data::{erased::ErasedData, Data};
pub use query::{filter, DataQuery, LinkQuery, Receiver, TypeFilter};
pub use request::Request;

pub mod prelude {
    #[cfg(feature = "unique")]
    pub use crate::data::unique::{MaybeUnique, Unique};
    pub use crate::data::Data;
    #[cfg(feature = "unique")]
    pub use crate::id::ID;
    #[cfg(feature = "derive")]
    pub use datalink_derive::Data;
}

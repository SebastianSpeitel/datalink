//! # Legacy / Unused Macros
//!
//! > [!WARNING]
//! > The macros in this module are part of an older, deprecated design of the datalink crate
//! > and are currently **unregistered** and **not maintained**.
//! > Use `#[derive(Data)]` instead of `impl_data!`.
//!
//! This file is kept for historical reference of the old pull/link-based design.

#[cfg(feature = "unique")]
#[macro_export]
#[deprecated(since = "0.4.0", note = "Use #[derive(Data)] instead")]
macro_rules! impl_unique {
    ($name:ident, $id:expr) => {
        impl $crate::data::unique::Unique for $name {
            #[inline]
            fn id(&self) -> $crate::id::ID {
                $crate::id::ID::try_new($id).unwrap()
            }
        }
    };
}

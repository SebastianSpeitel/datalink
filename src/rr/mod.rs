pub mod erased;
pub mod meta;
pub mod provided;
pub mod query;
pub mod receiver;
pub mod request;
pub mod typeset;

pub use query::Query;
pub use receiver::Receiver;
pub use request::Request;
pub use typeset::TypeSet;

pub mod prelude {
    pub use super::provided::Provided;
    pub use super::query::Query;
    pub use super::receiver::Receiver;
    pub use super::request::Request;
}

#[macro_export]
macro_rules! type_eq {
    ($ty1:ty, $ty2:ty) => {
        core::any::TypeId::of::<$ty1>() == core::any::TypeId::of::<$ty2>()
    };
}

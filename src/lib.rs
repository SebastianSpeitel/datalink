// Show which crate feature enables conditionally compiled APIs in documentation.
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    unreachable_pub,
    clippy::unwrap_used,
    clippy::missing_inline_in_public_items
)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)] // remove for prod

pub mod data;
pub mod links;
mod macros;
pub mod query;
pub mod value;
#[cfg(feature = "well_known")]
pub mod well_known;
#[cfg(feature = "derive")]
pub use datalink_derive::Data;
// pub mod specific;

pub mod rr;

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
    pub use crate::impl_data;
    #[cfg(feature = "derive")]
    pub use datalink_derive::Data;
}

// mod _temp {
//     /// Has to be object safe for Unknown::Receiver = &mut dyn Receiver
//     pub trait Receiver {
//         fn bool(&mut self, value: bool);
//         fn other_ref(&mut self, value: &dyn core::any::Any) {
//             if let Some(v) = value.downcast_ref::<bool>() {
//                 self.bool(*v);
//             }
//         }
//     }

//     pub trait RequestType: 'static {
//         type Receiver<'d>: Receiver;

//         fn requests<T: core::any::Any>() -> bool;
//     }

//     #[derive(Debug)]
//     pub struct Request<'d, T: RequestType + ?Sized>(T::Receiver<'d>);

//     impl<'d, T: RequestType + ?Sized> Request<'d, T> {
//         pub const fn new(receiver: T::Receiver<'d>) -> Self {
//             Self(receiver)
//         }

//         // pub fn receiver_mut(&mut self) -> &mut T::Receiver<'d> {
//         //     &mut self.0
//         // }

//         // pub fn into_receiver(self) -> T::Receiver<'d> {
//         //     self.0
//         // }
//     }

//     impl<'d, T: RequestType + ?Sized> Default for Request<'d, T>
//     where
//         T::Receiver<'d>: Default,
//     {
//         fn default() -> Self {
//             Self(Default::default())
//         }
//     }

//     impl<'d, T: RequestType + ?Sized> core::ops::Deref for Request<'d, T> {
//         type Target = T::Receiver<'d>;

//         fn deref(&self) -> &Self::Target {
//             &self.0
//         }
//     }

//     impl<'d, T: RequestType + ?Sized> core::ops::DerefMut for Request<'d, T> {
//         fn deref_mut(&mut self) -> &mut Self::Target {
//             &mut self.0
//         }
//     }

//     // impl deref receiver for request

//     // #[warn(clippy::missing_trait_methods)]
//     // impl<'d, T: RequestType + ?Sized> Receiver<'d> for Request<'d, T> {
//     //     #[inline]
//     //     fn bool(&mut self, value: bool) {
//     //         self.0.bool(value);
//     //     }
//     // }

//     pub trait Provider<R: RequestType + ?Sized = Unknown> {
//         fn provide<'d>(&'d self, request: Request<'d, R>);
//     }

//     #[derive(Debug)]
//     pub struct Unknown;
//     impl RequestType for Unknown {
//         type Receiver<'d> = &'d mut dyn Receiver;

//         fn requests<T: core::any::Any>() -> bool {
//             true
//         }
//     }

//     #[warn(clippy::missing_trait_methods)]
//     impl Receiver for &mut dyn Receiver {
//         fn bool(&mut self, value: bool) {
//             (**self).bool(value);
//         }
//     }

//     impl<T: RequestType + ?Sized> Provider<T> for bool {
//         fn provide<'d>(&'d self, mut request: Request<'d, T>) {
//             // if (T::Receiver::bool == noop) this will compile to a no-op
//             // -> this can be inlined at least to T::Receiver::bool
//             request.bool(*self);
//         }
//     }

//     impl Receiver for Option<bool> {
//         fn bool(&mut self, value: bool) {
//             *self = Some(value);
//         }
//     }

//     #[warn(clippy::missing_trait_methods)]
//     impl<T: Receiver> Receiver for &mut T {
//         #[inline]
//         fn bool(&mut self, value: bool) {
//             (**self).bool(value);
//         }
//         #[inline]
//         fn other_ref(&mut self, value: &dyn core::any::Any) {
//             (**self).other_ref(value);
//         }
//     }

//     struct OptionRef<T>(core::marker::PhantomData<T>);

//     impl<T: 'static> RequestType for OptionRef<T>
//     where
//         for<'a> &'a mut Option<T>: Receiver,
//     {
//         type Receiver<'d> = &'d mut Option<T>;

//         fn requests<T2: core::any::Any>() -> bool {
//             use core::any::TypeId;
//             TypeId::of::<T>() == TypeId::of::<T2>()
//         }
//     }

//     #[derive(Debug, Default)]
//     struct Debugging;

//     impl RequestType for Debugging {
//         type Receiver<'d> = Self;

//         fn requests<T: core::any::Any>() -> bool {
//             true
//         }
//     }

//     impl Receiver for Debugging {
//         fn bool(&mut self, value: bool) {
//             dbg!("bool:", value);
//         }
//     }

//     impl<T: RequestType + ?Sized> Provider<T> for &dyn Provider {
//         fn provide<'d>(&'d self, mut request: Request<'d, T>) {
//             // I have no idea how the lifetimes work here
//             let request = Request::new(&mut request.0 as &mut dyn Receiver);
//             (*self).provide(request);
//         }
//     }

//     // trait ProviderExt<T: RequestType + ?Sized> {
//     //     // fn as_bool<R:RequestType>(&self) -> Option<bool>
//     //     // where
//     //     //     R: for<'a> RequestType<Receiver<'a> = &'a mut Option<bool>>,
//     //     // {
//     //     //     let mut receiver = Option::<bool>::None;
//     //     //     let request = Request::new(&mut receiver);
//     //     //     self.provide(request);
//     //     //     receiver
//     //     // }
//     //     fn as_<O, R: RequestType + ?Sized>(&self) -> Option<O>;
//     // }

//     // impl<T, T0: RequestType + ?Sized> ProviderExt<T0> for T
//     // where
//     //     T: Provider,
//     // {
//     //     fn as_<'d, O, R>(&'d self) -> Option<O>
//     //     where
//     //         R: RequestType + ?Sized,
//     //         R: for<'a> RequestType<Receiver<'a> = &'a mut Option<O>>,
//     //     {
//     //         let mut ret = None;
//     //         let request = Request::new(&mut ret);
//     //         Provider::<R>::provide(self, request);
//     //         ret
//     //     }
//     // }

//     fn provide_is_obj_safe(o: &dyn Provider) {}

//     fn test() {
//         let request: Request<'_, Debugging> = Request::default();
//         true.provide(request);

//         let request: Request<'_, Debugging> = Request::default();
//         let p = &true as &dyn Provider;
//         // Provider::<Debugging>::provide(&p, Default::default());
//         (&p).provide(request);

//         let mut bool = None;
//         (&p).provide(Request::<OptionRef<bool>>::new(&mut bool));

//         // dbg!(p.as_::<bool>());
//     }
// }

#![allow(unused_variables)]
use std::any::Any;

// #[derive(Debug, Clone, Default)]
// pub enum RequestHint {
//     #[default]
//     Any,
//     Fullfilled,
//     TypeId(std::any::TypeId),
//     TypeName(&'static str),
//     String,
//     Bytes,
//     Fast,
//     Precise,
//     All,
// }

// impl RequestHint {
//     #[inline]
//     #[must_use]
//     pub fn type_id_of<T: 'static + ?Sized>() -> Self {
//         Self::TypeId(std::any::TypeId::of::<T>())
//     }

//     #[inline]
//     #[must_use]
//     pub fn type_name_of<T: ?Sized>() -> Self {
//         Self::TypeName(std::any::type_name::<T>())
//     }

//     #[inline]
//     #[must_use]
//     pub fn is_fullfilled(&self) -> bool {
//         matches!(self, Self::Fullfilled)
//     }

//     #[inline]
//     #[must_use]
//     pub fn type_id(&self) -> Option<std::any::TypeId> {
//         match self {
//             Self::TypeId(id) => Some(*id),
//             Self::String => Some(std::any::TypeId::of::<String>()),
//             Self::Bytes => Some(std::any::TypeId::of::<Vec<u8>>()),
//             _ => None,
//         }
//     }
// }

// macro_rules! type_eq {
//     ($ty1:ty, $ty2:ty) => {
//         core::any::TypeId::of::<$ty1>() == core::any::TypeId::of::<$ty2>()
//     };
// }

/// # Safety
///
/// The caller must ensure that the type of `value` is `T`.
// #[inline(always)]
// unsafe fn downcast_ref_unchecked<T: Any>(any: &dyn Any) -> &T {
//     debug_assert_eq!(any.type_id(), core::any::TypeId::of::<T>());
//     any.downcast_ref::<T>().unwrap_unchecked()
// }
use crate::data::Provided;
/// # Safety
///
/// The caller must ensure that the type of `value` is `T`.
// #[inline(always)]
// unsafe fn downcast_unchecked<T: Any>(any: Box<dyn Any>) -> Box<T> {
//     debug_assert_eq!(any.type_id(), core::any::TypeId::of::<T>());
//     any.downcast::<T>().unwrap_unchecked()
// }
pub use crate::rr::Receiver as ValueReceiver;
pub use crate::rr::Req;
pub use crate::rr::Request as ValueRequest;
pub use crate::rr::Unknown;

// mod old {
//     use super::*;

//     pub trait ValueRequest<'d, T: RequestType + ?Sized = dyn Any> {
//         #[inline]
//         fn provide_requested_ref(&mut self, value: &T)
//         where
//             T: 'static + Sized,
//         {
//             // # Safety
//             // The type is always checked before calling `downcast_ref_unchecked`.
//             unsafe {
//                 if type_eq!(T, bool) {
//                     self.bool(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, u8) {
//                     self.u8(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, i8) {
//                     self.i8(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, u16) {
//                     self.u16(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, i16) {
//                     self.i16(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, u32) {
//                     self.u32(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, i32) {
//                     self.i32(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, u64) {
//                     self.u64(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, i64) {
//                     self.i64(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, u128) {
//                     self.u128(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, i128) {
//                     self.i128(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, f32) {
//                     self.f32(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, f64) {
//                     self.f64(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, char) {
//                     self.char(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, &str) {
//                     self.str(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, String) {
//                     self.str_owned(String::to_owned(downcast_ref_unchecked(value)));
//                 }
//                 if type_eq!(T, Cow<str>) {
//                     self.str_maybe(Cow::clone(downcast_ref_unchecked(value)));
//                 }
//                 if type_eq!(T, &[u8]) {
//                     self.bytes(*downcast_ref_unchecked(value));
//                 }
//                 if type_eq!(T, Vec<u8>) {
//                     self.bytes_owned(Vec::to_owned(downcast_ref_unchecked(value)));
//                 }
//                 if type_eq!(T, Cow<[u8]>) {
//                     self.bytes_maybe(Cow::clone(downcast_ref_unchecked(value)));
//                 }
//             }
//         }

//         #[inline]
//         fn provide_requested_owned(&mut self, value: T)
//         where
//             T: 'static + Sized,
//         {
//             unsafe {
//                 if type_eq!(T, bool) {
//                     self.bool(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, u8) {
//                     self.u8(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, i8) {
//                     self.i8(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, u16) {
//                     self.u16(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, i16) {
//                     self.i16(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, u32) {
//                     self.u32(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, i32) {
//                     self.i32(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, u64) {
//                     self.u64(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, i64) {
//                     self.i64(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, u128) {
//                     self.u128(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, i128) {
//                     self.i128(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, f32) {
//                     self.f32(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, f64) {
//                     self.f64(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, char) {
//                     self.char(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, &str) {
//                     self.str(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, String) {
//                     self.str_owned(String::to_owned(downcast_ref_unchecked(&value)));
//                 }
//                 if type_eq!(T, Cow<str>) {
//                     self.str_maybe(Cow::clone(downcast_ref_unchecked(&value)));
//                 }
//                 if type_eq!(T, &[u8]) {
//                     self.bytes(*downcast_ref_unchecked(&value));
//                 }
//                 if type_eq!(T, Vec<u8>) {
//                     self.bytes_owned(Vec::to_owned(downcast_ref_unchecked(&value)));
//                 }
//                 if type_eq!(T, Cow<[u8]>) {
//                     self.bytes_maybe(Cow::clone(downcast_ref_unchecked(&value)));
//                 }
//             }
//         }

//         #[inline]
//         fn any_ref(&mut self, value: &dyn Any) {
//             use std::any::TypeId;
//             match value.type_id() {
//                 id if id == TypeId::of::<bool>() => unsafe {
//                     self.bool(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u8>() => unsafe {
//                     self.u8(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i8>() => unsafe {
//                     self.i8(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u16>() => unsafe {
//                     self.u16(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i16>() => unsafe {
//                     self.i16(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u32>() => unsafe {
//                     self.u32(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i32>() => unsafe {
//                     self.i32(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u64>() => unsafe {
//                     self.u64(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i64>() => unsafe {
//                     self.i64(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u128>() => unsafe {
//                     self.u128(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i128>() => unsafe {
//                     self.i128(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<f32>() => unsafe {
//                     self.f32(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<f64>() => unsafe {
//                     self.f64(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<char>() => unsafe {
//                     self.char(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<&str>() => unsafe {
//                     self.str(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<String>() => unsafe {
//                     self.str_owned(String::to_owned(downcast_ref_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<Cow<str>>() => unsafe {
//                     self.str_maybe(Cow::clone(downcast_ref_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<&[u8]>() => unsafe {
//                     self.bytes(*downcast_ref_unchecked(value));
//                 },
//                 id if id == TypeId::of::<Vec<u8>>() => unsafe {
//                     self.bytes_owned(Vec::to_owned(downcast_ref_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<Cow<[u8]>>() => unsafe {
//                     self.bytes_maybe(Cow::clone(downcast_ref_unchecked(value)));
//                 },
//                 _ => {}
//             }
//         }

//         #[inline]
//         fn any_owned(&mut self, value: Box<dyn Any>) {
//             use std::any::TypeId;
//             match (*value).type_id() {
//                 id if id == TypeId::of::<bool>() => unsafe {
//                     self.bool(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u8>() => unsafe {
//                     self.u8(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i8>() => unsafe {
//                     self.i8(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u16>() => unsafe {
//                     self.u16(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i16>() => unsafe {
//                     self.i16(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u32>() => unsafe {
//                     self.u32(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i32>() => unsafe {
//                     self.i32(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u64>() => unsafe {
//                     self.u64(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i64>() => unsafe {
//                     self.i64(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<u128>() => unsafe {
//                     self.u128(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<i128>() => unsafe {
//                     self.i128(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<f32>() => unsafe {
//                     self.f32(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<f64>() => unsafe {
//                     self.f64(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<char>() => unsafe {
//                     self.char(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<&str>() => unsafe {
//                     self.str(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<String>() => unsafe {
//                     self.str_owned(String::to_owned(*downcast_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<Cow<str>>() => unsafe {
//                     self.str_maybe(Cow::clone(*downcast_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<&[u8]>() => unsafe {
//                     self.bytes(*downcast_unchecked(value));
//                 },
//                 id if id == TypeId::of::<Vec<u8>>() => unsafe {
//                     self.bytes_owned(Vec::to_owned(*downcast_unchecked(value)));
//                 },
//                 id if id == TypeId::of::<Cow<[u8]>>() => unsafe {
//                     self.bytes_maybe(Cow::clone(*downcast_unchecked(value)));
//                 },
//                 _ => {}
//             }
//         }

//         #[inline]
//         fn r#true(&mut self) {
//             self.bool(true);
//         }

//         #[inline]
//         fn r#false(&mut self) {
//             self.bool(false);
//         }

//         #[inline]
//         fn bool(&mut self, value: bool) {}

//         #[inline]
//         fn u8(&mut self, value: u8) {}

//         #[inline]
//         fn i8(&mut self, value: i8) {}

//         #[inline]
//         fn u16(&mut self, value: u16) {}

//         #[inline]
//         fn i16(&mut self, value: i16) {}

//         #[inline]
//         fn u32(&mut self, value: u32) {}

//         #[inline]
//         fn i32(&mut self, value: i32) {}

//         #[inline]
//         fn u64(&mut self, value: u64) {}

//         #[inline]
//         fn i64(&mut self, value: i64) {}

//         #[inline]
//         fn u128(&mut self, value: u128) {}

//         #[inline]
//         fn i128(&mut self, value: i128) {}

//         #[inline]
//         fn f32(&mut self, value: f32) {}

//         #[inline]
//         fn f64(&mut self, value: f64) {}

//         #[inline]
//         fn char(&mut self, value: char) {}

//         #[inline]
//         fn str(&mut self, value: &str) {}

//         #[inline]
//         fn str_borrowed(&mut self, value: &'d str) {
//             self.str(value);
//         }

//         #[inline]
//         fn str_maybe(&mut self, value: Cow<'d, str>) {
//             match value {
//                 Cow::Borrowed(v) => self.str_borrowed(v),
//                 Cow::Owned(v) => self.str_owned(v),
//             };
//         }

//         #[inline]
//         fn str_owned(&mut self, value: String) {
//             self.str(value.as_str());
//         }

//         #[inline]
//         fn bytes(&mut self, value: &[u8]) {}

//         #[inline]
//         fn bytes_borrowed(&mut self, value: &'d [u8]) {
//             self.bytes(value);
//         }

//         #[inline]
//         fn bytes_maybe(&mut self, value: Cow<'d, [u8]>) {
//             match value {
//                 Cow::Borrowed(v) => self.bytes_borrowed(v),
//                 Cow::Owned(v) => self.bytes_owned(v),
//             }
//         }

//         #[inline]
//         fn bytes_owned(&mut self, value: Vec<u8>) {
//             self.bytes(value.as_slice());
//         }
//     }

//     impl<T: RequestType + ?Sized> dyn ValueRequest<'_, T> + '_ {
//         #[inline]
//         pub fn requests_all(&self) -> bool {
//             if T::requests_all() {
//                 return true;
//             }
//             false
//         }

//         #[inline]
//         pub fn requests<U: Any>(&self) -> bool {
//             T::requests::<U>()
//         }

//         #[inline]
//         pub fn provide_ref<V: Any>(&mut self, value: &V) {
//             T::provide_ref_to(self, value);
//         }
//     }

//     pub trait RequestType {
//         fn requests_all() -> bool;

//         fn requests<T: Any>() -> bool;

//         fn provide_ref_to<'d, T: Any>(request: &mut dyn ValueRequest<'d, Self>, value: &T);
//     }
//     // impl<T: Any + ?Sized> RequestType for T {
//     //     type Value = Self;

//     //     #[inline]
//     //     fn accepts_any() -> bool {
//     //         use core::any::TypeId;
//     //         TypeId::of::<Self>() == TypeId::of::<dyn Any>()
//     //     }

//     //     #[inline]
//     //     fn accepts<T2: Any>() -> bool {
//     //         use core::any::TypeId;
//     //         if Self::accepts_any() {
//     //             return true;
//     //         }
//     //         if TypeId::of::<Self>() == TypeId::of::<T2>() {
//     //             return true;
//     //         }

//     //         return false;
//     //     }
//     // }

//     impl RequestType for dyn Any {
//         #[inline]
//         fn requests_all() -> bool {
//             true
//         }

//         #[inline]
//         fn requests<T: Any>() -> bool {
//             true
//         }

//         #[inline]
//         fn provide_ref_to<'d, T: Any>(request: &mut dyn ValueRequest<'d, Self>, value: &T) {
//             request.any_ref(value);
//             // todo: use specific methods if known
//         }
//     }

//     impl RequestType for bool {
//         #[inline]
//         fn requests_all() -> bool {
//             false
//         }

//         fn requests<T: Any>() -> bool {
//             use core::any::TypeId;
//             TypeId::of::<bool>() == TypeId::of::<T>()
//         }

//         fn provide_ref_to<'d, T: Any>(request: &mut dyn ValueRequest<'d, Self>, value: &T) {
//             if type_eq!(T, bool) {
//                 unsafe {
//                     request.bool(*downcast_ref_unchecked(value));
//                 }
//             }
//         }
//     }

//     impl RequestType for u64 {
//         #[inline]
//         fn requests_all() -> bool {
//             false
//         }

//         fn requests<T: Any>() -> bool {
//             use core::any::TypeId;
//             TypeId::of::<u64>() == TypeId::of::<T>()
//         }

//         fn provide_ref_to<'d, T: Any>(request: &mut dyn ValueRequest<'d, Self>, value: &T) {
//             if type_eq!(T, u64) {
//                 unsafe {
//                     request.u64(*downcast_ref_unchecked(value));
//                 }
//             }
//         }
//     }

//     macro_rules! impl_request {
//         ($ty:ty, $method:ident) => {
//             impl<T: $crate::value::RequestType + ?Sized> $crate::value::ValueRequest<'_, T>
//                 for Option<$ty>
//             {
//                 #[inline]
//                 fn any_ref(&mut self, value: &dyn core::any::Any) {
//                     if let Some(value) = value.downcast_ref::<$ty>() {
//                         self.replace(*value);
//                     }
//                 }

//                 #[inline]
//                 fn any_owned(&mut self, value: Box<dyn core::any::Any>) {
//                     if let Ok(value) = value.downcast::<$ty>() {
//                         self.replace(*value);
//                     }
//                 }

//                 #[inline]
//                 fn $method(&mut self, value: $ty) {
//                     self.replace(value);
//                 }
//             }
//         };
//     }

//     impl_request!(bool, bool);
//     impl_request!(u8, u8);
//     impl_request!(i8, i8);
//     impl_request!(u16, u16);
//     impl_request!(i16, i16);
//     impl_request!(u32, u32);
//     impl_request!(i32, i32);
//     impl_request!(u64, u64);
//     impl_request!(i64, i64);
//     impl_request!(u128, u128);
//     impl_request!(i128, i128);
//     impl_request!(f32, f32);
//     impl_request!(f64, f64);
//     impl_request!(char, char);

//     impl ValueRequest<'_> for Option<String> {
//         #[inline]
//         fn str(&mut self, value: &str) {
//             self.replace(value.to_owned());
//         }
//         #[inline]
//         fn str_maybe(&mut self, value: Cow<'_, str>) {
//             self.replace(value.into_owned());
//         }
//         #[inline]
//         fn str_owned(&mut self, value: String) {
//             self.replace(value);
//         }
//     }

//     impl<'a> ValueRequest<'a> for Option<Cow<'a, str>> {
//         #[inline]
//         fn str(&mut self, value: &str) {
//             self.replace(Cow::Owned(value.to_owned()));
//         }
//         #[inline]
//         fn str_owned(&mut self, value: String) {
//             self.replace(Cow::Owned(value));
//         }
//         #[inline]
//         fn str_borrowed(&mut self, value: &'a str) {
//             self.replace(Cow::Borrowed(value));
//         }
//     }

//     impl ValueRequest<'_> for Option<Vec<u8>> {
//         #[inline]
//         fn bytes(&mut self, value: &[u8]) {
//             self.replace(value.to_owned());
//         }
//         #[inline]
//         fn bytes_owned(&mut self, value: Vec<u8>) {
//             self.replace(value);
//         }
//     }

//     impl<'a> ValueRequest<'a> for Option<Cow<'a, [u8]>> {
//         #[inline]
//         fn bytes(&mut self, value: &[u8]) {
//             self.replace(Cow::Owned(value.to_owned()));
//         }
//         #[inline]
//         fn bytes_borrowed(&mut self, value: &'a [u8]) {
//             self.replace(Cow::Borrowed(value));
//         }
//         #[inline]
//         fn bytes_owned(&mut self, value: Vec<u8>) {
//             self.replace(Cow::Owned(value));
//         }
//     }

//     impl From<bool> for Value<'_> {
//         #[inline]
//         fn from(value: bool) -> Self {
//             Self::Bool(value)
//         }
//     }

//     impl From<u8> for Value<'_> {
//         #[inline]
//         fn from(value: u8) -> Self {
//             Self::U8(value)
//         }
//     }

//     impl From<i8> for Value<'_> {
//         #[inline]
//         fn from(value: i8) -> Self {
//             Self::I8(value)
//         }
//     }

//     impl From<u16> for Value<'_> {
//         #[inline]
//         fn from(value: u16) -> Self {
//             Self::U16(value)
//         }
//     }

//     impl From<i16> for Value<'_> {
//         #[inline]
//         fn from(value: i16) -> Self {
//             Self::I16(value)
//         }
//     }

//     impl From<u32> for Value<'_> {
//         #[inline]
//         fn from(value: u32) -> Self {
//             Self::U32(value)
//         }
//     }

//     impl From<i32> for Value<'_> {
//         #[inline]
//         fn from(value: i32) -> Self {
//             Self::I32(value)
//         }
//     }

//     impl From<u64> for Value<'_> {
//         #[inline]
//         fn from(value: u64) -> Self {
//             Self::U64(value)
//         }
//     }

//     impl From<i64> for Value<'_> {
//         #[inline]
//         fn from(value: i64) -> Self {
//             Self::I64(value)
//         }
//     }

//     impl From<u128> for Value<'_> {
//         #[inline]
//         fn from(value: u128) -> Self {
//             Self::U128(value)
//         }
//     }

//     impl From<i128> for Value<'_> {
//         #[inline]
//         fn from(value: i128) -> Self {
//             Self::I128(value)
//         }
//     }

//     impl From<f32> for Value<'_> {
//         #[inline]
//         fn from(value: f32) -> Self {
//             Self::F32(value)
//         }
//     }

//     impl From<f64> for Value<'_> {
//         #[inline]
//         fn from(value: f64) -> Self {
//             Self::F64(value)
//         }
//     }

//     impl From<char> for Value<'_> {
//         #[inline]
//         fn from(value: char) -> Self {
//             Self::Char(value)
//         }
//     }

//     impl<'v> From<&'v str> for Value<'v> {
//         #[inline]
//         fn from(value: &'v str) -> Self {
//             Self::Str(value)
//         }
//     }

//     impl From<String> for Value<'_> {
//         #[inline]
//         fn from(value: String) -> Self {
//             Self::StrOwned(value)
//         }
//     }

//     impl<'v> From<&'v [u8]> for Value<'v> {
//         #[inline]
//         fn from(value: &'v [u8]) -> Self {
//             Self::Bytes(value)
//         }
//     }

//     impl From<Vec<u8>> for Value<'_> {
//         #[inline]
//         fn from(value: Vec<u8>) -> Self {
//             Self::BytesOwned(value)
//         }
//     }

//     impl From<Box<dyn Any>> for Value<'_> {
//         #[inline]
//         fn from(value: Box<dyn Any>) -> Self {
//             Self::Any(value)
//         }
//     }
// }

#[derive(Debug)]
pub enum Value {
    True,
    False,
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    Other(Box<dyn Any>),
}

impl ValueReceiver for Value {
    #[inline]
    fn bool(&mut self, value: bool) {
        *self = Self::Bool(value);
    }

    #[inline]
    fn u8(&mut self, value: u8) {
        *self = Self::U8(value);
    }

    #[inline]
    fn i8(&mut self, value: i8) {
        *self = Self::I8(value);
    }

    #[inline]
    fn u16(&mut self, value: u16) {
        *self = Self::U16(value);
    }

    #[inline]
    fn i16(&mut self, value: i16) {
        *self = Self::I16(value);
    }

    #[inline]
    fn u32(&mut self, value: u32) {
        *self = Self::U32(value);
    }

    #[inline]
    fn i32(&mut self, value: i32) {
        *self = Self::I32(value);
    }

    #[inline]
    fn u64(&mut self, value: u64) {
        *self = Self::U64(value);
    }

    #[inline]
    fn i64(&mut self, value: i64) {
        *self = Self::I64(value);
    }

    #[inline]
    fn u128(&mut self, value: u128) {
        *self = Self::U128(value);
    }

    #[inline]
    fn i128(&mut self, value: i128) {
        *self = Self::I128(value);
    }

    #[inline]
    fn f32(&mut self, value: f32) {
        *self = Self::F32(value);
    }

    #[inline]
    fn f64(&mut self, value: f64) {
        *self = Self::F64(value);
    }

    #[inline]
    fn char(&mut self, value: char) {
        *self = Self::Char(value);
    }

    #[inline]
    fn str(&mut self, value: &str) {
        *self = Self::String(value.to_owned());
    }

    #[inline]
    fn str_owned(&mut self, value: String) {
        *self = Self::String(value);
    }

    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        *self = Self::Bytes(value.to_owned());
    }

    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
        *self = Self::Bytes(value);
    }

    #[inline]
    fn accepts<T: 'static + ?Sized>() -> bool {
        // Todo: check if the type is accepted
        true
    }
}

#[inline]
fn provide_value<R: Req>(value: &Value, request: &mut ValueRequest<R>) {
    match value {
        Value::Bool(v) => request.provide_ref(v),
        Value::U8(v) => request.provide_ref(v),
        Value::I8(v) => request.provide_ref(v),
        Value::U16(v) => request.provide_ref(v),
        Value::I16(v) => request.provide_ref(v),
        Value::U32(v) => request.provide_ref(v),
        Value::I32(v) => request.provide_ref(v),
        Value::U64(v) => request.provide_ref(v),
        Value::I64(v) => request.provide_ref(v),
        Value::U128(v) => request.provide_ref(v),
        Value::I128(v) => request.provide_ref(v),
        Value::F32(v) => request.provide_ref(v),
        Value::F64(v) => request.provide_ref(v),
        Value::Char(v) => request.provide_ref(v),
        Value::String(v) => request.provide_str(v),
        Value::Bytes(v) => request.provide_bytes(v),
        Value::Other(v) => request.provide_ref(v),
        Value::True => request.provide_bool(true),
        Value::False => request.provide_bool(false),
    }
}

impl crate::Data for Value {
    #[inline]
    fn provide_value(&self, mut request: ValueRequest<'_>) {
        provide_value(self, &mut request);
    }
    #[inline]
    fn provide_requested<'d, R: Req>(&self, request: &mut ValueRequest<'d, R>) -> impl Provided {
        provide_value(self, request);
    }
}

impl std::fmt::Display for Value {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::True => f.write_str("true"),
            Value::False => f.write_str("false"),
            Value::Bool(v) => {
                if *v {
                    f.write_str("true")
                } else {
                    f.write_str("false")
                }
            }
            Value::U8(v) => f.write_fmt(format_args!("{v}u8")),
            Value::I8(v) => f.write_fmt(format_args!("{v}i8")),
            Value::U16(v) => f.write_fmt(format_args!("{v}u16")),
            Value::I16(v) => f.write_fmt(format_args!("{v}i16")),
            Value::U32(v) => f.write_fmt(format_args!("{v}u32")),
            Value::I32(v) => f.write_fmt(format_args!("{v}i32")),
            Value::U64(v) => f.write_fmt(format_args!("{v}u64")),
            Value::I64(v) => f.write_fmt(format_args!("{v}i64")),
            Value::U128(v) => f.write_fmt(format_args!("{v}u128")),
            Value::I128(v) => f.write_fmt(format_args!("{v}i128")),
            Value::F32(v) => f.write_fmt(format_args!("{v}f32")),
            Value::F64(v) => f.write_fmt(format_args!("{v}f64")),
            Value::Char(v) => f.write_fmt(format_args!("'{v}'")),
            Value::String(v) => f.write_fmt(format_args!("\"{v}\"")),
            Value::Bytes(v) => {
                if let Ok(s) = std::str::from_utf8(v) {
                    f.write_fmt(format_args!("b\"{s}\""))
                } else {
                    f.write_fmt(format_args!("{v:?}"))
                }
            }
            Value::Other(v) => f.write_fmt(format_args!("{v:?}")),
        }
    }
}
#[derive(Debug, Default)]
pub struct AllValues(Vec<Value>);

#[warn(clippy::missing_trait_methods)]
impl ValueReceiver for AllValues {
    #[inline]
    fn bool(&mut self, value: bool) {
        self.0.push(Value::Bool(value));
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        self.0.push(Value::Bytes(value.to_owned()));
    }
    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
        self.0.push(Value::Bytes(value));
    }
    #[inline]
    fn char(&mut self, value: char) {
        self.0.push(Value::Char(value));
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        self.0.push(Value::F32(value));
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        self.0.push(Value::F64(value));
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        self.0.push(Value::I128(value));
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        self.0.push(Value::I16(value));
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        self.0.push(Value::I32(value));
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        self.0.push(Value::I64(value));
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        self.0.push(Value::I8(value));
    }
    #[inline]
    fn str(&mut self, value: &str) {
        self.0.push(Value::String(value.to_owned()));
    }
    #[inline]
    fn str_owned(&mut self, value: String) {
        self.0.push(Value::String(value));
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        self.0.push(Value::U128(value));
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        self.0.push(Value::U16(value));
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        self.0.push(Value::U32(value));
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        self.0.push(Value::U64(value));
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        self.0.push(Value::U8(value));
    }
    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        self.0.push(Value::Other(value));
    }
    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        // Can't be stored as Value
    }
    #[inline]
    fn accepts<T: 'static + ?Sized>() -> bool {
        Value::accepts::<T>()
    }
}

impl crate::Data for AllValues {
    #[inline]
    fn provide_value(&self, mut request: ValueRequest<'_>) {
        for value in &self.0 {
            provide_value(value, &mut request);
        }
    }
    #[inline]
    fn provide_requested<'d, R: Req>(&self, request: &mut ValueRequest<'d, R>) -> impl Provided {
        for value in &self.0 {
            provide_value(value, request);
        }
    }
}

impl AllValues {
    #[inline]
    #[must_use]
    pub fn single(&self) -> Option<&Value> {
        if self.0.len() == 1 {
            self.0.first()
        } else {
            None
        }
    }
}

impl Req for AllValues {
    type Receiver<'d> = &'d mut AllValues;
}

impl core::ops::Deref for AllValues {
    type Target = Vec<Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_dyn_any_size() {
        use std::mem::size_of_val;
        let value: Box<dyn std::any::Any> = Box::new(42u8);

        assert_eq!(size_of_val(&value), 16);
    }
}

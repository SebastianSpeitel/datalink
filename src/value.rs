#![allow(unused_variables)]
use std::borrow::Cow;

pub trait ValueBuiler<'d> {
    #[inline]
    fn bool(&mut self, value: bool) {}

    #[inline]
    fn u8(&mut self, value: u8) {}

    #[inline]
    fn i8(&mut self, value: i8) {}

    #[inline]
    fn u16(&mut self, value: u16) {}

    #[inline]
    fn i16(&mut self, value: i16) {}

    #[inline]
    fn u32(&mut self, value: u32) {}

    #[inline]
    fn i32(&mut self, value: i32) {}

    #[inline]
    fn u64(&mut self, value: u64) {}

    #[inline]
    fn i64(&mut self, value: i64) {}

    #[inline]
    fn u128(&mut self, value: u128) {}

    #[inline]
    fn i128(&mut self, value: i128) {}

    #[inline]
    fn f32(&mut self, value: f32) {}

    #[inline]
    fn f64(&mut self, value: f64) {}

    #[inline]
    fn str(&mut self, value: Cow<'d, str>) {}

    #[inline]
    fn bytes(&mut self, value: Cow<'d, [u8]>) {}
}

macro_rules! impl_demand {
    ($ty:ty, $method:ident) => {
        impl ValueBuiler<'_> for Option<$ty> {
            #[inline]
            fn $method(&mut self, value: $ty) {
                self.replace(value);
            }
        }
    };
}

impl_demand!(bool, bool);
impl_demand!(u8, u8);
impl_demand!(i8, i8);
impl_demand!(u16, u16);
impl_demand!(i16, i16);
impl_demand!(u32, u32);
impl_demand!(i32, i32);
impl_demand!(u64, u64);
impl_demand!(i64, i64);
impl_demand!(u128, u128);
impl_demand!(i128, i128);
impl_demand!(f32, f32);
impl_demand!(f64, f64);

impl ValueBuiler<'_> for Option<String> {
    #[inline]
    fn str(&mut self, value: Cow<'_, str>) {
        self.replace(value.into_owned());
    }
}

impl<'a> ValueBuiler<'a> for Option<Cow<'a, str>> {
    #[inline]
    fn str(&mut self, value: Cow<'a, str>) {
        self.replace(value);
    }
}

impl<'a> ValueBuiler<'a> for Option<Cow<'a, [u8]>> {
    #[inline]
    fn bytes(&mut self, value: Cow<'a, [u8]>) {
        self.replace(value);
    }
}

#[derive(Debug)]
pub enum SingleValue {
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
    Str(Box<str>),
    Bytes(Box<[u8]>),
}

impl std::fmt::Display for SingleValue {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SingleValue as E;
        match self {
            E::Bool(true) => f.write_str("true"),
            E::Bool(false) => f.write_str("false"),
            E::U8(v) => f.write_fmt(format_args!("{v}u8")),
            E::I8(v) => f.write_fmt(format_args!("{v}i8")),
            E::U16(v) => f.write_fmt(format_args!("{v}u16")),
            E::I16(v) => f.write_fmt(format_args!("{v}i16")),
            E::U32(v) => f.write_fmt(format_args!("{v}u32")),
            E::I32(v) => f.write_fmt(format_args!("{v}i32")),
            E::U64(v) => f.write_fmt(format_args!("{v}u64")),
            E::I64(v) => f.write_fmt(format_args!("{v}i64")),
            E::U128(v) => f.write_fmt(format_args!("{v}u128")),
            E::I128(v) => f.write_fmt(format_args!("{v}i128")),
            E::F32(v) => f.write_fmt(format_args!("{v}f32")),
            E::F64(v) => f.write_fmt(format_args!("{v}f64")),
            E::Str(v) => f.write_fmt(format_args!("\"{v}\"")),
            E::Bytes(v) => f.write_fmt(format_args!("b\"{}\"", v.escape_ascii())),
        }
    }
}

// impl Value {
//     #[inline]
//     pub fn from_data<D: crate::Data + ?Sized>(data: &D) -> Self {
//         let mut value = Self::default();
//         data.provide_value(&mut value);
//         value
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_enum(&self) -> Option<Option<SingleValue>> {
//         use Option::{None as N, Some as S};
//         use SingleValue as E;
//         let val = match (
//             self.bool,
//             self.u8,
//             self.i8,
//             self.u16,
//             self.i16,
//             self.u32,
//             self.i32,
//             self.u64,
//             self.i64,
//             self.u128,
//             self.i128,
//             self.f32,
//             self.f64,
//             self.str.as_deref(),
//             self.bytes.as_deref(),
//         ) {
//             (N, N, N, N, N, N, N, N, N, N, N, N, N, N, N) => return Some(None),
//             (S(v), N, N, N, N, N, N, N, N, N, N, N, N, N, N) => E::Bool(v),
//             (N, S(v), N, N, N, N, N, N, N, N, N, N, N, N, N) => E::U8(v),
//             (N, N, S(v), N, N, N, N, N, N, N, N, N, N, N, N) => E::I8(v),
//             (N, N, N, S(v), N, N, N, N, N, N, N, N, N, N, N) => E::U16(v),
//             (N, N, N, N, S(v), N, N, N, N, N, N, N, N, N, N) => E::I16(v),
//             (N, N, N, N, N, S(v), N, N, N, N, N, N, N, N, N) => E::U32(v),
//             (N, N, N, N, N, N, S(v), N, N, N, N, N, N, N, N) => E::I32(v),
//             (N, N, N, N, N, N, N, S(v), N, N, N, N, N, N, N) => E::U64(v),
//             (N, N, N, N, N, N, N, N, S(v), N, N, N, N, N, N) => E::I64(v),
//             (N, N, N, N, N, N, N, N, N, S(v), N, N, N, N, N) => E::U128(v),
//             (N, N, N, N, N, N, N, N, N, N, S(v), N, N, N, N) => E::I128(v),
//             (N, N, N, N, N, N, N, N, N, N, N, S(v), N, N, N) => E::F32(v),
//             (N, N, N, N, N, N, N, N, N, N, N, N, S(v), N, N) => E::F64(v),
//             (N, N, N, N, N, N, N, N, N, N, N, N, N, S(v), N) => E::Str(v.into()),
//             (N, N, N, N, N, N, N, N, N, N, N, N, N, N, S(v)) => E::Bytes(v.into()),
//             _ => return None,
//         };
//         Some(Some(val))
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_bool(&self) -> Option<bool> {
//         self.bool
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_u8(&self) -> Option<u8> {
//         self.u8
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_i8(&self) -> Option<i8> {
//         self.i8
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_u16(&self) -> Option<u16> {
//         self.u16
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_i16(&self) -> Option<i16> {
//         self.i16
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_u32(&self) -> Option<u32> {
//         self.u32
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_i32(&self) -> Option<i32> {
//         self.i32
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_u64(&self) -> Option<u64> {
//         self.u64
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_i64(&self) -> Option<i64> {
//         self.i64
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_u128(&self) -> Option<u128> {
//         self.u128
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_i128(&self) -> Option<i128> {
//         self.i128
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_f32(&self) -> Option<f32> {
//         self.f32
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_f64(&self) -> Option<f64> {
//         self.f64
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_str(&self) -> Option<&str> {
//         self.str.as_deref()
//     }

//     #[inline]
//     #[must_use]
//     pub fn as_bytes(&self) -> Option<&[u8]> {
//         self.bytes.as_deref()
//     }
// }

// impl ValueBuiler<'_> for Value {
//     #[inline]
//     fn bool(&mut self, value: bool) {
//         self.bool.replace(value);
//     }

//     #[inline]
//     fn u8(&mut self, value: u8) {
//         self.u8.replace(value);
//     }

//     #[inline]
//     fn i8(&mut self, value: i8) {
//         self.i8.replace(value);
//     }

//     #[inline]
//     fn u16(&mut self, value: u16) {
//         self.u16.replace(value);
//     }

//     #[inline]
//     fn i16(&mut self, value: i16) {
//         self.i16.replace(value);
//     }

//     #[inline]
//     fn u32(&mut self, value: u32) {
//         self.u32.replace(value);
//     }

//     #[inline]
//     fn i32(&mut self, value: i32) {
//         self.i32.replace(value);
//     }

//     #[inline]
//     fn u64(&mut self, value: u64) {
//         self.u64.replace(value);
//     }

//     #[inline]
//     fn i64(&mut self, value: i64) {
//         self.i64.replace(value);
//     }

//     #[inline]
//     fn u128(&mut self, value: u128) {
//         self.u128.replace(value);
//     }

//     #[inline]
//     fn i128(&mut self, value: i128) {
//         self.i128.replace(value);
//     }

//     #[inline]
//     fn f32(&mut self, value: f32) {
//         self.f32.replace(value);
//     }

//     #[inline]
//     fn f64(&mut self, value: f64) {
//         self.f64.replace(value);
//     }

//     #[inline]
//     fn str(&mut self, value: Cow<str>) {
//         match value {
//             Cow::Borrowed(v) => self.str.replace(v.into()),
//             Cow::Owned(v) => self.str.replace(v.into_boxed_str()),
//         };
//     }

//     #[inline]
//     fn bytes(&mut self, value: Cow<[u8]>) {
//         match value {
//             Cow::Borrowed(v) => self.bytes.replace(v.into()),
//             Cow::Owned(v) => self.bytes.replace(v.into_boxed_slice()),
//         };
//     }
//}

#[derive(Clone, Debug, Default)]
pub struct Value<'a> {
    bool: Option<bool>,
    u8: Option<u8>,
    i8: Option<i8>,
    u16: Option<u16>,
    i16: Option<i16>,
    u32: Option<u32>,
    i32: Option<i32>,
    u64: Option<u64>,
    i64: Option<i64>,
    u128: Option<u128>,
    i128: Option<i128>,
    f32: Option<f32>,
    f64: Option<f64>,
    str: Option<Cow<'a, str>>,
    bytes: Option<Cow<'a, [u8]>>,
}

impl<'a> Value<'a> {
    pub fn from_data<D: crate::Data + ?Sized>(data: &'a D) -> Self {
        let mut value = Self::default();
        data.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    pub fn as_enum(&self) -> Option<Option<SingleValue>> {
        use Option::{None as N, Some as S};
        use SingleValue as E;
        let val = match (
            self.bool,
            self.u8,
            self.i8,
            self.u16,
            self.i16,
            self.u32,
            self.i32,
            self.u64,
            self.i64,
            self.u128,
            self.i128,
            self.f32,
            self.f64,
            self.str.as_deref(),
            self.bytes.as_deref(),
        ) {
            (N, N, N, N, N, N, N, N, N, N, N, N, N, N, N) => return Some(None),
            (S(v), N, N, N, N, N, N, N, N, N, N, N, N, N, N) => E::Bool(v),
            (N, S(v), N, N, N, N, N, N, N, N, N, N, N, N, N) => E::U8(v),
            (N, N, S(v), N, N, N, N, N, N, N, N, N, N, N, N) => E::I8(v),
            (N, N, N, S(v), N, N, N, N, N, N, N, N, N, N, N) => E::U16(v),
            (N, N, N, N, S(v), N, N, N, N, N, N, N, N, N, N) => E::I16(v),
            (N, N, N, N, N, S(v), N, N, N, N, N, N, N, N, N) => E::U32(v),
            (N, N, N, N, N, N, S(v), N, N, N, N, N, N, N, N) => E::I32(v),
            (N, N, N, N, N, N, N, S(v), N, N, N, N, N, N, N) => E::U64(v),
            (N, N, N, N, N, N, N, N, S(v), N, N, N, N, N, N) => E::I64(v),
            (N, N, N, N, N, N, N, N, N, S(v), N, N, N, N, N) => E::U128(v),
            (N, N, N, N, N, N, N, N, N, N, S(v), N, N, N, N) => E::I128(v),
            (N, N, N, N, N, N, N, N, N, N, N, S(v), N, N, N) => E::F32(v),
            (N, N, N, N, N, N, N, N, N, N, N, N, S(v), N, N) => E::F64(v),
            (N, N, N, N, N, N, N, N, N, N, N, N, N, S(v), N) => E::Str(v.into()),
            (N, N, N, N, N, N, N, N, N, N, N, N, N, N, S(v)) => E::Bytes(v.into()),
            _ => return None,
        };
        Some(Some(val))
    }

    #[inline]
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        self.bool
    }

    #[inline]
    #[must_use]
    pub fn as_u8(&self) -> Option<u8> {
        self.u8
    }

    #[inline]
    #[must_use]
    pub fn as_i8(&self) -> Option<i8> {
        self.i8
    }

    #[inline]
    #[must_use]
    pub fn as_u16(&self) -> Option<u16> {
        self.u16
    }

    #[inline]
    #[must_use]
    pub fn as_i16(&self) -> Option<i16> {
        self.i16
    }

    #[inline]
    #[must_use]
    pub fn as_u32(&self) -> Option<u32> {
        self.u32
    }

    #[inline]
    #[must_use]
    pub fn as_i32(&self) -> Option<i32> {
        self.i32
    }

    #[inline]
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        self.u64
    }

    #[inline]
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        self.i64
    }

    #[inline]
    #[must_use]
    pub fn as_u128(&self) -> Option<u128> {
        self.u128
    }

    #[inline]
    #[must_use]
    pub fn as_i128(&self) -> Option<i128> {
        self.i128
    }

    #[inline]
    #[must_use]
    pub fn as_f32(&self) -> Option<f32> {
        self.f32
    }

    #[inline]
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        self.f64
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        self.str.as_deref()
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        self.bytes.as_deref()
    }
}

impl crate::Data for Value<'_> {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        if let Some(v) = self.bool {
            value.bool(v);
        }
        if let Some(v) = self.u8 {
            value.u8(v);
        }
        if let Some(v) = self.i8 {
            value.i8(v);
        }
        if let Some(v) = self.u16 {
            value.u16(v);
        }
        if let Some(v) = self.i16 {
            value.i16(v);
        }
        if let Some(v) = self.u32 {
            value.u32(v);
        }
        if let Some(v) = self.i32 {
            value.i32(v);
        }
        if let Some(v) = self.u64 {
            value.u64(v);
        }
        if let Some(v) = self.i64 {
            value.i64(v);
        }
        if let Some(v) = self.u128 {
            value.u128(v);
        }
        if let Some(v) = self.i128 {
            value.i128(v);
        }
        if let Some(v) = self.f32 {
            value.f32(v);
        }
        if let Some(v) = self.f64 {
            value.f64(v);
        }
        if let Some(ref v) = self.str {
            value.str(Cow::Borrowed(v));
        }
        if let Some(ref v) = self.bytes {
            value.bytes(Cow::Borrowed(v));
        }
    }
}

impl<'a> ValueBuiler<'a> for Value<'a> {
    #[inline]
    fn bool(&mut self, value: bool) {
        self.bool.replace(value);
    }

    #[inline]
    fn u8(&mut self, value: u8) {
        self.u8.replace(value);
    }

    #[inline]
    fn i8(&mut self, value: i8) {
        self.i8.replace(value);
    }

    #[inline]
    fn u16(&mut self, value: u16) {
        self.u16.replace(value);
    }

    #[inline]
    fn i16(&mut self, value: i16) {
        self.i16.replace(value);
    }

    #[inline]
    fn u32(&mut self, value: u32) {
        self.u32.replace(value);
    }

    #[inline]
    fn i32(&mut self, value: i32) {
        self.i32.replace(value);
    }

    #[inline]
    fn u64(&mut self, value: u64) {
        self.u64.replace(value);
    }

    #[inline]
    fn i64(&mut self, value: i64) {
        self.i64.replace(value);
    }

    #[inline]
    fn u128(&mut self, value: u128) {
        self.u128.replace(value);
    }

    #[inline]
    fn i128(&mut self, value: i128) {
        self.i128.replace(value);
    }

    #[inline]
    fn f32(&mut self, value: f32) {
        self.f32.replace(value);
    }

    #[inline]
    fn f64(&mut self, value: f64) {
        self.f64.replace(value);
    }

    #[inline]
    fn str(&mut self, value: Cow<'a, str>) {
        self.str.replace(value);
    }

    #[inline]
    fn bytes(&mut self, value: Cow<'a, [u8]>) {
        self.bytes.replace(value);
    }
}

pub trait ValueType {
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>);
}

impl ValueType for bool {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.bool(*self);
    }
}

impl ValueType for u8 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.u8(*self);
    }
}

impl ValueType for i8 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.i8(*self);
    }
}

impl ValueType for u16 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.u16(*self);
    }
}

impl ValueType for i16 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.i16(*self);
    }
}

impl ValueType for u32 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.u32(*self);
    }
}

impl ValueType for i32 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.i32(*self);
    }
}

impl ValueType for u64 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.u64(*self);
    }
}

impl ValueType for i64 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.i64(*self);
    }
}

impl ValueType for u128 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.u128(*self);
    }
}

impl ValueType for i128 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.i128(*self);
    }
}

impl ValueType for f32 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.f32(*self);
    }
}

impl ValueType for f64 {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.f64(*self);
    }
}

impl ValueType for str {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.str(self.to_owned().into());
    }
}

impl ValueType for String {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.str(self.clone().into());
    }
}

impl ValueType for [u8] {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.bytes(self.to_owned().into());
    }
}

impl ValueType for Vec<u8> {
    #[inline]
    fn provide_to(&self, builder: &mut dyn ValueBuiler<'_>) {
        builder.bytes(self.clone().into());
    }
}

use std::any::Any;

pub use crate::rr::prelude::{
    Provided, Query as ValueQuery, Receiver as ValueReceiver, Request as ValueRequest,
};

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

impl Value {
    #[inline]
    pub fn as_number(&self) -> Option<isize> {
        match self {
            Value::U8(v) => (*v).try_into().ok(),
            Value::I8(v) => (*v).try_into().ok(),
            Value::U16(v) => (*v).try_into().ok(),
            Value::I16(v) => (*v).try_into().ok(),
            Value::U32(v) => (*v).try_into().ok(),
            Value::I32(v) => (*v).try_into().ok(),
            Value::U64(v) => (*v).try_into().ok(),
            Value::I64(v) => (*v).try_into().ok(),
            Value::U128(v) => (*v).try_into().ok(),
            Value::I128(v) => (*v).try_into().ok(),
            Value::F32(v) => (*v as isize).try_into().ok(),
            Value::F64(v) => (*v as isize).try_into().ok(),
            _ => None,
        }
    }
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
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        *self = Self::Other(value);
    }

    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        // todo: warn
    }

    #[inline]
    fn accepting() -> impl crate::rr::typeset::TypeSet + 'static {
        // Todo: check if the type is accepted
        crate::rr::typeset::All
    }
}

#[inline]
fn provide_value<R: ValueQuery>(value: &Value, request: &mut ValueRequest<R>) {
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
    fn provide_value(&self, request: &mut ValueRequest) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: ValueQuery>(&self, request: &mut ValueRequest<Q>) -> impl Provided {
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
            Value::Char(v) => f.write_fmt(format_args!("'{}'", v.escape_default())),
            Value::String(v) => f.write_fmt(format_args!("{v:?}")),
            Value::Bytes(v) => f.write_fmt(format_args!("b{}", v.escape_ascii())),
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
    #[allow(unused_variables)]
    fn other_ref(&mut self, value: &dyn Any) {
        // Can't be stored as Value
    }
    #[inline]
    fn accepting() -> impl crate::rr::typeset::TypeSet + 'static {
        Value::accepting()
    }
}

impl crate::Data for AllValues {
    #[inline]
    fn provide_value(&self, request: &mut ValueRequest) {
        for val in &self.0 {
            val.provide_value(request);
        }
    }
    #[inline]
    fn provide_requested<Q: ValueQuery>(&self, request: &mut ValueRequest<Q>) -> impl Provided {
        for val in &self.0 {
            val.provide_requested(request).debug_assert_provided();
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

impl core::ops::Deref for AllValues {
    type Target = Vec<Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for AllValues {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for AllValues {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

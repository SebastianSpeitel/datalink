use core::any::Any;

use crate::{Data, DataQuery, Receiver, Request, TypeFilter};

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
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    #[must_use]
    pub fn as_number(&self) -> Option<isize> {
        match *self {
            Value::True => Some(1),
            Value::False => Some(0),
            Value::Bool(v) => Some(v.into()),
            Value::U8(v) => Some(v.into()),
            Value::I8(v) => Some(v.into()),
            Value::U16(v) => v.try_into().ok(),
            Value::I16(v) => Some(v.into()),
            Value::U32(v) => v.try_into().ok(),
            Value::I32(v) => v.try_into().ok(),
            Value::U64(v) => v.try_into().ok(),
            Value::I64(v) => v.try_into().ok(),
            Value::U128(v) => v.try_into().ok(),
            Value::I128(v) => v.try_into().ok(),
            Value::F32(v) => Some(v as isize),
            Value::F64(v) => Some(v as isize),
            _ => None,
        }
    }
}

impl Receiver for Value {
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
    fn str_owned(&mut self, value: Box<str>) {
        *self = Self::String(value.into_string());
    }

    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        *self = Self::Bytes(value.to_owned());
    }

    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        *self = Self::Bytes(value.into_vec());
    }

    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        *self = Self::Other(value);
    }

    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        // TODO: maybe dowcast to primitive types
        let _ = value;
    }

    #[inline]
    fn accepting() -> impl TypeFilter + 'static {
        // Todo: check if the type is accepted
        crate::filter::Any
    }
}

impl Data for Value {
    #[inline]
    fn query(&self, req: &mut impl Request) {
        req.provide_discriminant(self);
        match *self {
            Self::Bool(b) => b.query(req),
            Self::Bytes(ref b) => b.query(req),
            Self::Char(c) => c.query(req),
            Self::F32(n) => n.query(req),
            Self::F64(n) => n.query(req),
            Self::False => false.query(req),
            Self::I128(n) => n.query(req),
            Self::I16(n) => n.query(req),
            Self::I32(n) => n.query(req),
            Self::I64(n) => n.query(req),
            Self::I8(n) => n.query(req),
            Self::Other(ref o) => o.as_ref().query(req),
            Self::String(ref s) => s.query(req),
            Self::True => true.query(req),
            Self::U128(n) => n.query(req),
            Self::U16(n) => n.query(req),
            Self::U32(n) => n.query(req),
            Self::U64(n) => n.query(req),
            Self::U8(n) => n.query(req),
        }
    }
}

impl std::fmt::Display for Value {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::True => f.write_str("true"),
            Value::False => f.write_str("false"),
            Value::Bool(v) => {
                if v {
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
            Value::String(ref v) => f.write_fmt(format_args!("{v:?}")),
            Value::Bytes(ref v) => f.write_fmt(format_args!("b{}", v.escape_ascii())),
            Value::Other(ref v) => f.write_fmt(format_args!("{v:?}")),
        }
    }
}
#[derive(Debug, Default)]
pub struct AllValues(Vec<Value>);

#[warn(clippy::missing_trait_methods)]
impl Receiver for AllValues {
    #[inline]
    fn bool(&mut self, value: bool) {
        self.0.push(Value::Bool(value));
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        self.0.push(Value::Bytes(value.to_owned()));
    }
    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        self.0.push(Value::Bytes(value.into_vec()));
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
    fn str_owned(&mut self, value: Box<str>) {
        self.0.push(Value::String(value.into_string()));
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
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        self.0.push(Value::Other(Box::new(data)));
    }

    #[inline]
    fn accepting() -> impl TypeFilter + 'static {
        Value::accepting()
    }
}

impl DataQuery for AllValues {
    type LinkQuery<'q> = ();
    type Receiver<'q> = &'q mut Self;
    type Filter<'q> = crate::filter::Any;

    #[inline]
    fn link_query(&mut self) -> Self::LinkQuery<'_> {}

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        self
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        Default::default()
    }
}

impl Data for AllValues {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        for val in &self.0 {
            val.query(request);
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

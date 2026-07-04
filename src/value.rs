use core::any::Any;

use crate::{Data, Request, Visitor};

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
            Self::True => Some(1),
            Self::False => Some(0),
            Self::Bool(v) => Some(v.into()),
            Self::U8(v) => Some(v.into()),
            Self::I8(v) => Some(v.into()),
            Self::U16(v) => v.try_into().ok(),
            Self::I16(v) => Some(v.into()),
            Self::U32(v) => v.try_into().ok(),
            Self::I32(v) => v.try_into().ok(),
            Self::U64(v) => v.try_into().ok(),
            Self::I64(v) => v.try_into().ok(),
            Self::U128(v) => v.try_into().ok(),
            Self::I128(v) => v.try_into().ok(),
            Self::F32(v) => Some(v as isize),
            Self::F64(v) => Some(v as isize),
            _ => None,
        }
    }
}

impl Data for Value {
    #[inline]
    fn query(&self, mut req: impl Request) {
        req.provide_discriminant(self);
        match *self {
            Self::Bool(b) => req.provide_value(b),
            Self::Bytes(ref b) => req.provide_bytes(b),
            Self::Char(c) => req.provide_value(c),
            Self::F32(n) => req.provide_value(n),
            Self::F64(n) => req.provide_value(n),
            Self::False => req.provide_value(false),
            Self::I128(n) => req.provide_value(n),
            Self::I16(n) => req.provide_value(n),
            Self::I32(n) => req.provide_value(n),
            Self::I64(n) => req.provide_value(n),
            Self::I8(n) => req.provide_value(n),
            Self::Other(ref o) => req.visitor().visit_any(o.as_ref()),
            Self::String(ref s) => req.provide_str(s),
            Self::True => req.provide_value(true),
            Self::U128(n) => req.provide_value(n),
            Self::U16(n) => req.provide_value(n),
            Self::U32(n) => req.provide_value(n),
            Self::U64(n) => req.provide_value(n),
            Self::U8(n) => req.provide_value(n),
        }
    }
    #[inline]
    fn query_owned(self, mut req: impl Request) {
        req.provide_discriminant(&self);
        match self {
            Self::Bool(b) => req.provide_value(b),
            Self::Bytes(b) => req.provide_value(b),
            Self::Char(c) => req.provide_value(c),
            Self::F32(n) => req.provide_value(n),
            Self::F64(n) => req.provide_value(n),
            Self::False => req.provide_value(false),
            Self::I128(n) => req.provide_value(n),
            Self::I16(n) => req.provide_value(n),
            Self::I32(n) => req.provide_value(n),
            Self::I64(n) => req.provide_value(n),
            Self::I8(n) => req.provide_value(n),
            Self::Other(o) => req.visitor().visit_any_owned(o),
            Self::String(s) => req.provide_value(s),
            Self::True => req.provide_value(true),
            Self::U128(n) => req.provide_value(n),
            Self::U16(n) => req.provide_value(n),
            Self::U32(n) => req.provide_value(n),
            Self::U64(n) => req.provide_value(n),
            Self::U8(n) => req.provide_value(n),
        }
    }
}

impl core::fmt::Display for Value {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Bool(v) => {
                if v {
                    f.write_str("true")
                } else {
                    f.write_str("false")
                }
            }
            Self::U8(v) => f.write_fmt(format_args!("{v}u8")),
            Self::I8(v) => f.write_fmt(format_args!("{v}i8")),
            Self::U16(v) => f.write_fmt(format_args!("{v}u16")),
            Self::I16(v) => f.write_fmt(format_args!("{v}i16")),
            Self::U32(v) => f.write_fmt(format_args!("{v}u32")),
            Self::I32(v) => f.write_fmt(format_args!("{v}i32")),
            Self::U64(v) => f.write_fmt(format_args!("{v}u64")),
            Self::I64(v) => f.write_fmt(format_args!("{v}i64")),
            Self::U128(v) => f.write_fmt(format_args!("{v}u128")),
            Self::I128(v) => f.write_fmt(format_args!("{v}i128")),
            Self::F32(v) => f.write_fmt(format_args!("{v}f32")),
            Self::F64(v) => f.write_fmt(format_args!("{v}f64")),
            Self::Char(v) => f.write_fmt(format_args!("'{}'", v.escape_default())),
            Self::String(ref v) => f.write_fmt(format_args!("{v:?}")),
            Self::Bytes(ref v) => f.write_fmt(format_args!("b{}", v.escape_ascii())),
            Self::Other(ref v) => f.write_fmt(format_args!("{v:?}")),
        }
    }
}
#[derive(Debug, Default)]
pub struct AllValues(Vec<Value>);

impl Visitor for AllValues {
    #[inline]
    fn visit_bool(&mut self, value: bool) {
        self.0.push(Value::Bool(value));
    }
    #[inline]
    fn visit_bytes(&mut self, value: &[u8]) {
        self.0.push(Value::Bytes(value.to_owned()));
    }
    #[inline]
    fn visit_bytes_owned(&mut self, value: Box<[u8]>) {
        self.0.push(Value::Bytes(value.into_vec()));
    }
    #[inline]
    fn visit_char(&mut self, value: char) {
        self.0.push(Value::Char(value));
    }
    #[inline]
    fn visit_f32(&mut self, value: f32) {
        self.0.push(Value::F32(value));
    }
    #[inline]
    fn visit_f64(&mut self, value: f64) {
        self.0.push(Value::F64(value));
    }
    #[inline]
    fn visit_i128(&mut self, value: i128) {
        self.0.push(Value::I128(value));
    }
    #[inline]
    fn visit_i16(&mut self, value: i16) {
        self.0.push(Value::I16(value));
    }
    #[inline]
    fn visit_i32(&mut self, value: i32) {
        self.0.push(Value::I32(value));
    }
    #[inline]
    fn visit_i64(&mut self, value: i64) {
        self.0.push(Value::I64(value));
    }
    #[inline]
    fn visit_i8(&mut self, value: i8) {
        self.0.push(Value::I8(value));
    }
    #[inline]
    fn visit_str(&mut self, value: &str) {
        self.0.push(Value::String(value.to_owned()));
    }
    #[inline]
    fn visit_str_owned(&mut self, value: Box<str>) {
        self.0.push(Value::String(value.into_string()));
    }
    #[inline]
    fn visit_u128(&mut self, value: u128) {
        self.0.push(Value::U128(value));
    }
    #[inline]
    fn visit_u16(&mut self, value: u16) {
        self.0.push(Value::U16(value));
    }
    #[inline]
    fn visit_u32(&mut self, value: u32) {
        self.0.push(Value::U32(value));
    }
    #[inline]
    fn visit_u64(&mut self, value: u64) {
        self.0.push(Value::U64(value));
    }
    #[inline]
    fn visit_u8(&mut self, value: u8) {
        self.0.push(Value::U8(value));
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        self.0.push(Value::Other(Box::new(value)));
    }
    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, _value: &T) {
        // Can't be stored as Value
    }
    #[inline]
    fn visit_any(&mut self, _value: &dyn core::any::Any) {
        // Can't be stored as Value
    }
    #[inline]
    fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
        self.0.push(Value::Other(value));
    }
}

impl Request for AllValues {
    #[inline]
    fn schema(&self) -> impl crate::schema::Schema {
        crate::schema::ONLY_VALUES
    }

    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        self
    }

    #[inline]
    fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
        self
    }
}

impl crate::request::ErasableRequest for AllValues {
    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        crate::schema::OnlyValues::SCHEMA
    }
    #[inline]
    fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
        Some(self)
    }
}

impl Data for AllValues {
    #[inline]
    fn query(&self, mut request: impl Request) {
        for val in &self.0 {
            val.query(request.by_ref());
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

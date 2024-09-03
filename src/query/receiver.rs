use core::any::{Any, TypeId};

use super::TypeFilter;

// todo forward! macro or function to use in other_ref and other_owned
// todo: fn error(&mut self, value: impl Error/anyhow::Error)
pub trait Receiver {
    #[inline]
    #[allow(unused_variables)]
    fn bool(&mut self, value: bool) {}

    #[inline]
    #[allow(unused_variables)]
    fn i8(&mut self, value: i8) {}

    #[inline]
    #[allow(unused_variables)]
    fn u8(&mut self, value: u8) {}

    #[inline]
    #[allow(unused_variables)]
    fn i16(&mut self, value: i16) {}

    #[inline]
    #[allow(unused_variables)]
    fn u16(&mut self, value: u16) {}

    #[inline]
    #[allow(unused_variables)]
    fn i32(&mut self, value: i32) {}

    #[inline]
    #[allow(unused_variables)]
    fn u32(&mut self, value: u32) {}

    #[inline]
    #[allow(unused_variables)]
    fn i64(&mut self, value: i64) {}

    #[inline]
    #[allow(unused_variables)]
    fn u64(&mut self, value: u64) {}

    #[inline]
    #[allow(unused_variables)]
    fn i128(&mut self, value: i128) {}

    #[inline]
    #[allow(unused_variables)]
    fn u128(&mut self, value: u128) {}

    #[inline]
    #[allow(unused_variables)]
    fn f32(&mut self, value: f32) {}

    #[inline]
    #[allow(unused_variables)]
    fn f64(&mut self, value: f64) {}

    #[inline]
    #[allow(unused_variables)]
    fn char(&mut self, value: char) {}

    #[inline]
    #[allow(unused_variables)]
    fn str(&mut self, value: &str) {}

    #[inline]
    fn str_owned(&mut self, value: Box<str>) {
        self.str(&value);
    }

    #[inline]
    #[allow(unused_variables)]
    fn bytes(&mut self, value: &[u8]) {}

    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        self.bytes(&value);
    }

    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        macro_rules! match_cast {
            ($($ty:ty => $f:ident $(,)?)*) => {
                match value.type_id() {
                    $(id if id == TypeId::of::<$ty>() => {
                        if let Some(v) = value.downcast_ref() {
                            self.$f(*v);
                        }
                    })*
                    _ => {}
                };
            };
        }

        match_cast! {
            bool => bool,
            i8 => i8,
            u8 => u8,
            i16 => i16,
            u16 => u16,
            i32 => i32,
            u32 => u32,
            i64 => i64,
            u64 => u64,
            i128 => i128,
            u128 => u128,
            f32 => f32,
            f64 => f64,
            char => char,
            str => str,
            &str => str,
            String => str,
            [u8] => bytes,
            &[u8] => bytes,
            Vec<u8> => bytes,
        };
    }

    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        macro_rules! match_cast {
            ($($ty:ty => $f:ident $(,)?)*) => {
                match value.type_id() {
                    $(id if id == TypeId::of::<$ty>() => {
                        if let Ok(v) = value.downcast::<$ty>() {
                            self.$f(*v);
                        }
                        return;
                    })*
                    _ => {}
                };
            };
        }

        match_cast! {
            bool => bool,
            i8 => i8,
            u8 => u8,
            i16 => i16,
            u16 => u16,
            i32 => i32,
            u32 => u32,
            i64 => i64,
            u64 => u64,
            i128 => i128,
            u128 => u128,
            f32 => f32,
            f64 => f64,
            char => char,
            &str => str,
            Box<str> => str_owned,
            &[u8] => bytes,
            Box<[u8]> => bytes_owned,
        };

        self.other_ref(value.as_ref());
    }

    #[inline]
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        let _ = data;
    }

    #[inline]
    #[must_use]
    fn accepting() -> impl crate::TypeFilter + 'static
    where
        Self: Sized,
    {
        crate::filter::Any
    }
}

impl core::fmt::Debug for dyn Receiver + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Receiver").finish_non_exhaustive()
    }
}

#[derive(Debug, Default)]
pub struct ErasedReceiver<'q> {
    accepting: super::filter::ValidErasedFilter<'q>,
    receiver: Option<Box<dyn Receiver + 'q>>,
}

impl<'q> ErasedReceiver<'q> {
    #[inline]
    #[must_use]
    pub fn new<R>(receiver: R) -> Self
    where
        R: Receiver + 'q,
    {
        let accepting = R::accepting();
        if accepting.is_empty() {
            return Self::default();
        }
        Self {
            accepting: accepting.into_erased(),
            receiver: Some(Box::new(receiver)),
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl Receiver for ErasedReceiver<'_> {
    #[inline]
    fn bool(&mut self, value: bool) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.bool(value),
            _ => {}
        }
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.i8(value),
            _ => {}
        }
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.u8(value),
            _ => {}
        }
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.i16(value),
            _ => {}
        }
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.u16(value),
            _ => {}
        }
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.i32(value),
            _ => {}
        }
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.u32(value),
            _ => {}
        }
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.i64(value),
            _ => {}
        }
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.u64(value),
            _ => {}
        }
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.i128(value),
            _ => {}
        }
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.u128(value),
            _ => {}
        }
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.f32(value),
            _ => {}
        }
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.f64(value),
            _ => {}
        }
    }
    #[inline]
    fn char(&mut self, value: char) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.char(value),
            _ => {}
        }
    }

    #[inline]
    fn str(&mut self, value: &str) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(value) => r.str(value),
            _ => {}
        }
    }

    #[inline]
    fn str_owned(&mut self, value: Box<str>) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.str_owned(value),
            _ => {}
        }
    }

    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(value) => r.bytes(value),
            _ => {}
        }
    }

    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.bytes_owned(value),
            _ => {}
        }
    }

    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(value) => r.other_ref(value),
            _ => {}
        }
    }

    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&value) => r.other_boxed(value),
            _ => {}
        }
    }

    #[inline]
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        match &mut self.receiver {
            Some(r) if self.accepting.accepts(&data) => r.erased_data(data),
            _ => {}
        }
    }

    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::filter::Any
    }
}

#[warn(clippy::missing_trait_methods)]
impl Receiver for Box<dyn Receiver + '_> {
    #[inline]
    fn bool(&mut self, value: bool) {
        (**self).bool(value);
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        (**self).i8(value);
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        (**self).u8(value);
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        (**self).i16(value);
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        (**self).u16(value);
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        (**self).i32(value);
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        (**self).u32(value);
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        (**self).i64(value);
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        (**self).u64(value);
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        (**self).i128(value);
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        (**self).u128(value);
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        (**self).f32(value);
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        (**self).f64(value);
    }
    #[inline]
    fn char(&mut self, value: char) {
        (**self).char(value);
    }
    #[inline]
    fn str(&mut self, value: &str) {
        (**self).str(value);
    }
    #[inline]
    fn str_owned(&mut self, value: Box<str>) {
        (**self).str_owned(value);
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        (**self).bytes(value);
    }
    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        (**self).bytes_owned(value);
    }
    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        (**self).other_ref(value);
    }
    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        (**self).other_boxed(value);
    }
    #[inline]
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        (**self).erased_data(data);
    }

    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::filter::Any
    }
}

#[warn(clippy::missing_trait_methods)]
impl<R: Receiver> Receiver for &mut R {
    #[inline]
    fn bool(&mut self, value: bool) {
        (**self).bool(value);
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        (**self).i8(value);
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        (**self).u8(value);
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        (**self).i16(value);
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        (**self).u16(value);
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        (**self).i32(value);
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        (**self).u32(value);
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        (**self).i64(value);
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        (**self).u64(value);
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        (**self).i128(value);
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        (**self).u128(value);
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        (**self).f32(value);
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        (**self).f64(value);
    }
    #[inline]
    fn char(&mut self, value: char) {
        (**self).char(value);
    }
    #[inline]
    fn str(&mut self, value: &str) {
        (**self).str(value);
    }
    #[inline]
    fn str_owned(&mut self, value: Box<str>) {
        (**self).str_owned(value);
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        (**self).bytes(value);
    }
    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        (**self).bytes_owned(value);
    }
    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        (**self).other_ref(value);
    }
    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        (**self).other_boxed(value);
    }
    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        R::accepting()
    }
    #[inline]
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        (**self).erased_data(data);
    }
}

macro_rules! impl_option_receiver {
    ($ty:ty, $method:ident) => {
        impl $crate::Receiver for Option<$ty> {
            #[inline]
            fn $method(&mut self, value: $ty) {
                self.replace(value);
            }
            #[inline]
            fn accepting() -> impl $crate::filter::TypeFilter + 'static {
                crate::filter::Only::<$ty>::default()
            }
        }
    };
}

impl_option_receiver!(bool, bool);
impl_option_receiver!(i8, i8);
impl_option_receiver!(u8, u8);
impl_option_receiver!(i16, i16);
impl_option_receiver!(u16, u16);
impl_option_receiver!(i32, i32);
impl_option_receiver!(u32, u32);
impl_option_receiver!(i64, i64);
impl_option_receiver!(u64, u64);
impl_option_receiver!(i128, i128);
impl_option_receiver!(u128, u128);
impl_option_receiver!(f32, f32);
impl_option_receiver!(f64, f64);
impl_option_receiver!(char, char);

impl Receiver for Option<String> {
    #[inline]
    fn str(&mut self, value: &str) {
        self.replace(value.to_owned());
    }

    #[inline]
    fn str_owned(&mut self, value: Box<str>) {
        self.replace(value.into_string());
    }

    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::filter::STRING_LIKE
    }
}

impl Receiver for Option<Vec<u8>> {
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        self.replace(value.to_owned());
    }

    #[inline]
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        self.replace(value.into_vec());
    }

    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::filter::BYTES_LIKE
    }
}

impl Receiver for () {
    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::filter::AnyOf::<()>::default()
    }
}

impl Receiver for Option<crate::id::ID> {
    #[inline]
    fn other_ref(&mut self, value: &dyn std::any::Any) {
        if let Some(id) = value.downcast_ref() {
            self.replace(*id);
        }
    }

    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        if let Ok(id) = value.downcast::<crate::id::ID>() {
            self.replace(*id);
        }
    }

    #[inline]
    fn accepting() -> impl crate::TypeFilter + 'static {
        crate::query::filter::Only::<crate::id::ID>::default()
    }
}

impl Receiver for Option<Box<crate::ErasedData>> {
    #[inline]
    fn other_boxed(&mut self, value: Box<dyn std::any::Any>) {
        if let Ok(data) = value.downcast::<Box<crate::ErasedData>>() {
            self.replace(data);
        }
    }
    #[inline]
    fn erased_data(&mut self, data: Box<crate::ErasedData>) {
        self.replace(data);
    }
}

use core::any::{Any, TypeId};

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
    fn str_owned(&mut self, value: String) {
        self.str(&value);
    }

    #[inline]
    #[allow(unused_variables)]
    fn bytes(&mut self, value: &[u8]) {}

    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
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
            String => str_owned,
            &[u8] => bytes,
            Vec<u8> => bytes_owned,
        };

        self.other_ref(value.as_ref());
    }

    #[inline]
    #[must_use]
    fn accepting() -> impl super::TypeSet + 'static
    where
        Self: Sized,
    {
        super::typeset::All
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
    fn str_owned(&mut self, value: String) {
        (**self).str_owned(value);
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        (**self).bytes(value);
    }
    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
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
    fn accepting() -> impl super::TypeSet + 'static {
        R::accepting()
    }
}

macro_rules! impl_option_receiver {
    ($ty:ty, $method:ident) => {
        impl $crate::rr::Receiver for Option<$ty> {
            #[inline]
            fn $method(&mut self, value: $ty) {
                self.replace(value);
            }
            #[inline]
            fn accepting() -> impl $crate::rr::typeset::TypeSet + 'static {
                crate::rr::typeset::Only::<$ty>::default()
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
    fn str_owned(&mut self, value: String) {
        self.replace(value);
    }

    #[inline]
    fn accepting() -> impl super::TypeSet + 'static {
        super::typeset::STRING_LIKE
    }
}

impl Receiver for Option<Vec<u8>> {
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        self.replace(value.to_owned());
    }

    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
        self.replace(value);
    }

    #[inline]
    fn accepting() -> impl super::TypeSet + 'static {
        super::typeset::BYTES_LIKE
    }
}

pub trait ReceiverExt: Receiver {
    fn accepts_id(&self, type_id: TypeId) -> bool;
}

impl<R: super::Receiver> ReceiverExt for R {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        use super::TypeSet;
        R::accepting().contains_id(type_id)
    }
}

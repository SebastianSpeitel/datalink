use core::any::Any;

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
    #[allow(unused_variables)]
    fn other_ref(&mut self, value: &dyn Any) {
        if let Some(v) = value.downcast_ref() {
            self.bool(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.i8(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.u8(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.i16(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.u16(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.i32(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.u32(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.i64(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.u64(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.i128(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.u128(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.f32(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.f64(*v);
        } else if let Some(v) = value.downcast_ref() {
            self.char(*v);
        } else if let Some(v) = value.downcast_ref::<&str>() {
            self.str(v);
        } else if let Some(v) = value.downcast_ref::<String>() {
            self.str(v);
        } else if let Some(v) = value.downcast_ref::<&[u8]>() {
            self.bytes(v);
        } else if let Some(v) = value.downcast_ref::<Vec<u8>>() {
            self.bytes(v);
        }
    }

    #[inline]
    #[allow(unused_variables)]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        macro_rules! cast_and_provide {
            ($val:ident,$ty:ty, $fn:ident) => {
                match $val.downcast::<$ty>() {
                    Ok(v) => {
                        self.$fn(*v);
                        return;
                    }
                    Err(v) => v,
                }
            };
        }
        let value = cast_and_provide!(value, bool, bool);
        let value = cast_and_provide!(value, i8, i8);
        let value = cast_and_provide!(value, u8, u8);
        let value = cast_and_provide!(value, i16, i16);
        let value = cast_and_provide!(value, u16, u16);
        let value = cast_and_provide!(value, i32, i32);
        let value = cast_and_provide!(value, u32, u32);
        let value = cast_and_provide!(value, i64, i64);
        let value = cast_and_provide!(value, u64, u64);
        let value = cast_and_provide!(value, i128, i128);
        let value = cast_and_provide!(value, u128, u128);
        let value = cast_and_provide!(value, f32, f32);
        let value = cast_and_provide!(value, f64, f64);
        let value = cast_and_provide!(value, char, char);
        let value = cast_and_provide!(value, &str, str);
        let value = cast_and_provide!(value, String, str_owned);
        let value = cast_and_provide!(value, &[u8], bytes);
        let value = cast_and_provide!(value, Vec<u8>, bytes_owned);

        self.other_ref(value.as_ref());
    }

    #[inline]
    #[must_use]
    fn accepts<T: 'static + ?Sized>() -> bool
    where
        Self: Sized,
    {
        true
    }
}

macro_rules! receiver_deref_fns {
    () => {
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
    };
}

#[warn(clippy::missing_trait_methods)]
impl Receiver for &mut dyn Receiver {
    receiver_deref_fns!();

    #[inline]
    fn accepts<U: 'static + ?Sized>() -> bool {
        true
    }
}

#[warn(clippy::missing_trait_methods)]
impl<T: Receiver> Receiver for &mut T {
    receiver_deref_fns!();

    #[inline]
    fn accepts<U: Any + ?Sized>() -> bool {
        T::accepts::<U>()
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
            fn accepts<U: core::any::Any + ?Sized>() -> bool {
                use core::any::TypeId;
                TypeId::of::<$ty>() == TypeId::of::<U>()
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
    fn accepts<T: Any + ?Sized>() -> bool
    where
        Self: Sized,
    {
        use core::any::TypeId;
        let id = TypeId::of::<T>();
        id == TypeId::of::<String>() || id == TypeId::of::<&str>()
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
    fn accepts<T: Any + ?Sized>() -> bool
    where
        Self: Sized,
    {
        use core::any::TypeId;
        let id = TypeId::of::<T>();
        id == TypeId::of::<Vec<u8>>() || id == TypeId::of::<&[u8]>()
    }
}

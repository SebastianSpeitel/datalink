use super::Receiver;
use super::Req;
use core::any::Any;

macro_rules! type_eq {
    ($ty1:ty, $ty2:ty) => {
        core::any::TypeId::of::<$ty1>() == core::any::TypeId::of::<$ty2>()
    };
}

/// # Safety
///
/// The caller must ensure that the type of `value` is `T`.
#[inline(always)]
unsafe fn downcast_ref_unchecked<T: Any>(any: &dyn Any) -> &T {
    debug_assert_eq!(any.type_id(), core::any::TypeId::of::<T>());
    any.downcast_ref::<T>().unwrap_unchecked()
}

/// # Safety
///
/// The caller must ensure that the type of `value` is `T`.
// #[inline(always)]
// unsafe fn downcast_unchecked<T: Any>(any: Box<dyn Any>) -> Box<T> {
//     debug_assert_eq!(any.type_id(), core::any::TypeId::of::<T>());
//     any.downcast::<T>().unwrap_unchecked()
// }

macro_rules! provide_typed {
    ($rec:expr, $ty1:ty, $ty2:ty, $method:ident, $val:expr) => {
        if type_eq!($ty1, $ty2) {
            unsafe {
                $rec.$method(*downcast_ref_unchecked::<$ty2>($val));
            }
            return;
        }
    };
}

macro_rules! provision_fn {
    ($m:ident, $ty:ty, $m_r:ident) => {
        #[inline]
        pub fn $m(&mut self, value: $ty) {
            dbg!(&value);
            dbg!(std::any::type_name::<R>());
            dbg!(R::requests::<$ty>());
            if !R::requests::<$ty>() {
                return;
            }
            self.0.$m_r(value);
        }
    };
}

#[derive(Debug)]
pub struct Request<'d, T: Req + ?Sized = super::Unknown>(pub(crate) T::Receiver<'d>);
impl<'d, R: Req> Request<'d, R> {
    #[inline]
    pub const fn new(receiver: R::Receiver<'d>) -> Self {
        Self(receiver)
    }

    #[inline]
    pub fn provide_ref<T: Any>(&mut self, value: &T) {
        if !R::requests::<T>() {
            return;
        }
        provide_typed!(self.0, T, bool, bool, value);
        provide_typed!(self.0, T, u8, u8, value);
        provide_typed!(self.0, T, i8, i8, value);
        provide_typed!(self.0, T, u16, u16, value);
        provide_typed!(self.0, T, i16, i16, value);
        provide_typed!(self.0, T, u32, u32, value);
        provide_typed!(self.0, T, i32, i32, value);
        provide_typed!(self.0, T, u64, u64, value);
        provide_typed!(self.0, T, i64, i64, value);
        provide_typed!(self.0, T, u128, u128, value);
        provide_typed!(self.0, T, i128, i128, value);
        provide_typed!(self.0, T, f32, f32, value);
        provide_typed!(self.0, T, f64, f64, value);
        provide_typed!(self.0, T, char, char, value);

        if type_eq!(T, &str) {
            unsafe {
                self.0.str(downcast_ref_unchecked::<&str>(value));
            }
            return;
        }

        if type_eq!(T, String) {
            // this sends a &str but only checked if String was requested
            unsafe {
                self.0.str(downcast_ref_unchecked::<String>(value));
            }
            return;
        }

        if type_eq!(T, &[u8]) {
            unsafe {
                self.0.bytes(downcast_ref_unchecked::<&[u8]>(value));
            }
            return;
        }

        if type_eq!(T, Vec<u8>) {
            // this sends a &[u8] but only checked if Vec<u8> was requested
            unsafe {
                self.0.bytes(downcast_ref_unchecked::<Vec<u8>>(value));
            }
            return;
        }

        self.0.other_ref(value);
    }

    #[inline]
    pub fn provide_owned<T1: Any>(&mut self, value: T1) {
        if !R::requests::<T1>() {
            return;
        }

        provide_typed!(self.0, T1, bool, bool, &value);
        provide_typed!(self.0, T1, u8, u8, &value);
        provide_typed!(self.0, T1, i8, i8, &value);
        provide_typed!(self.0, T1, u16, u16, &value);
        provide_typed!(self.0, T1, i16, i16, &value);
        provide_typed!(self.0, T1, u32, u32, &value);
        provide_typed!(self.0, T1, i32, i32, &value);
        provide_typed!(self.0, T1, u64, u64, &value);
        provide_typed!(self.0, T1, i64, i64, &value);
        provide_typed!(self.0, T1, u128, u128, &value);
        provide_typed!(self.0, T1, i128, i128, &value);
        provide_typed!(self.0, T1, f32, f32, &value);
        provide_typed!(self.0, T1, f64, f64, &value);
        provide_typed!(self.0, T1, char, char, &value);

        if type_eq!(T1, &str) {
            unsafe {
                self.0.str(downcast_ref_unchecked::<&str>(&value));
            }
            return;
        }
        if type_eq!(T1, String) {
            unsafe {
                self.0
                    .str_owned(downcast_ref_unchecked::<String>(&value).to_owned());
            }
            return;
        }

        if type_eq!(T1, &[u8]) {
            unsafe {
                self.0.bytes(downcast_ref_unchecked::<&[u8]>(&value));
            }
            return;
        }
        if type_eq!(T1, Vec<u8>) {
            unsafe {
                self.0
                    .bytes_owned(downcast_ref_unchecked::<Vec<u8>>(&value).to_owned());
            }
            return;
        }

        self.0.other_boxed(Box::new(value));
    }

    provision_fn!(provide_bool, bool, bool);
    provision_fn!(provide_i8, i8, i8);
    provision_fn!(provide_u8, u8, u8);
    provision_fn!(provide_i16, i16, i16);
    provision_fn!(provide_u16, u16, u16);
    provision_fn!(provide_i32, i32, i32);
    provision_fn!(provide_u32, u32, u32);
    provision_fn!(provide_i64, i64, i64);
    provision_fn!(provide_u64, u64, u64);
    provision_fn!(provide_i128, i128, i128);
    provision_fn!(provide_u128, u128, u128);
    provision_fn!(provide_f32, f32, f32);
    provision_fn!(provide_f64, f64, f64);
    provision_fn!(provide_char, char, char);
    provision_fn!(provide_str, &str, str);
    provision_fn!(provide_bytes, &[u8], bytes);

    #[inline]
    pub fn provide_str_owned(&mut self, value: String) {
        if R::requests::<String>() {
            self.0.str_owned(value);
        } else if R::requests::<&str>() {
            self.0.str(value.as_str());
        }
    }

    #[inline]
    pub fn provide_bytes_owned(&mut self, value: Vec<u8>) {
        if R::requests::<Vec<u8>>() {
            self.0.bytes_owned(value);
        } else if R::requests::<&[u8]>() {
            self.0.bytes(value.as_slice());
        }
    }
}

impl<'d, T: Req> Default for Request<'d, T>
where
    T::Receiver<'d>: Default,
{
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'d, T: Req + ?Sized> core::convert::AsMut<Self> for Request<'d, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
// impl<'d, T: Req> core::ops::Deref for Request<'d, T> {
//     type Target = T::Receiver<'d>;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<'d, T: Req> core::ops::DerefMut for Request<'d, T> {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

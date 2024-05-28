use super::query::Query;
use super::Receiver;
use crate::type_eq;
use core::any::Any;

/// # Safety
///
/// The caller must ensure that the type of `value` is `T`.
#[inline(always)]
unsafe fn downcast_ref_unchecked<T: Any>(any: &dyn Any) -> &T {
    debug_assert_eq!(any.type_id(), core::any::TypeId::of::<T>());
    any.downcast_ref::<T>().unwrap_unchecked()
}

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

macro_rules! provide_fn {
    ($m:ident, $ty:ty, $m_r:ident) => {
        #[inline]
        pub fn $m(&mut self, value: $ty) {
            if !self.requests::<$ty>() {
                return;
            }
            Q::get_receiver(&mut self.0).$m_r(value);
        }
    };
}

#[derive(Debug)]
pub struct Request<'d, Q: Query + ?Sized = super::erased::Erased<'d>>(
    Q::Request,
    core::marker::PhantomData<&'d ()>,
);

impl<Q: Query + ?Sized> Request<'_, Q> {
    #[inline]
    pub const fn new(request: Q::Request) -> Self {
        Self(request, core::marker::PhantomData)
    }

    #[inline]
    pub fn take(self) -> Q::Request {
        self.0
    }

    #[inline]
    pub fn requesting(&self) -> Q::Requesting<'_> {
        Q::get_requesting(&self.0)
    }

    #[inline]
    pub fn requests<T: 'static + ?Sized>(&self) -> bool {
        use super::TypeSet;
        self.requesting().contains_type::<T>()
    }

    #[inline]
    pub fn requests_type_of<T: 'static>(&self, value: &T) -> bool {
        use super::TypeSet;
        self.requesting().contains_type_of(value)
    }

    #[inline]
    pub fn provide_ref<T: 'static>(&mut self, value: &T) {
        if !self.requests_type_of(value) {
            return;
        }

        let mut rec = Q::get_receiver(&mut self.0);

        provide_typed!(rec, T, bool, bool, value);
        provide_typed!(rec, T, u8, u8, value);
        provide_typed!(rec, T, i8, i8, value);
        provide_typed!(rec, T, u16, u16, value);
        provide_typed!(rec, T, i16, i16, value);
        provide_typed!(rec, T, u32, u32, value);
        provide_typed!(rec, T, i32, i32, value);
        provide_typed!(rec, T, u64, u64, value);
        provide_typed!(rec, T, i64, i64, value);
        provide_typed!(rec, T, u128, u128, value);
        provide_typed!(rec, T, i128, i128, value);
        provide_typed!(rec, T, f32, f32, value);
        provide_typed!(rec, T, f64, f64, value);
        provide_typed!(rec, T, char, char, value);

        if type_eq!(T, &str) {
            unsafe {
                rec.str(downcast_ref_unchecked::<&str>(value));
            }
            return;
        }

        if type_eq!(T, String) {
            // this sends a &str but only checked if String was requested
            unsafe {
                rec.str(downcast_ref_unchecked::<String>(value));
            }
            return;
        }

        if type_eq!(T, &[u8]) {
            unsafe {
                rec.bytes(downcast_ref_unchecked::<&[u8]>(value));
            }
            return;
        }

        if type_eq!(T, Vec<u8>) {
            // this sends a &[u8] but only checked if Vec<u8> was requested
            unsafe {
                rec.bytes(downcast_ref_unchecked::<Vec<u8>>(value));
            }
            return;
        }

        rec.other_ref(value);
    }

    #[inline]
    pub fn provide_owned<T: 'static>(&mut self, value: T) {
        if !self.requests_type_of(&value) {
            return;
        }

        let mut rec = Q::get_receiver(&mut self.0);

        provide_typed!(rec, T, bool, bool, &value);
        provide_typed!(rec, T, u8, u8, &value);
        provide_typed!(rec, T, i8, i8, &value);
        provide_typed!(rec, T, u16, u16, &value);
        provide_typed!(rec, T, i16, i16, &value);
        provide_typed!(rec, T, u32, u32, &value);
        provide_typed!(rec, T, i32, i32, &value);
        provide_typed!(rec, T, u64, u64, &value);
        provide_typed!(rec, T, i64, i64, &value);
        provide_typed!(rec, T, u128, u128, &value);
        provide_typed!(rec, T, i128, i128, &value);
        provide_typed!(rec, T, f32, f32, &value);
        provide_typed!(rec, T, f64, f64, &value);
        provide_typed!(rec, T, char, char, &value);

        if type_eq!(T, &str) {
            unsafe {
                rec.str(downcast_ref_unchecked::<&str>(&value));
            }
            return;
        }
        if type_eq!(T, String) {
            unsafe {
                rec.str_owned(downcast_ref_unchecked::<String>(&value).to_owned());
            }
            return;
        }

        if type_eq!(T, &[u8]) {
            unsafe {
                rec.bytes(downcast_ref_unchecked::<&[u8]>(&value));
            }
            return;
        }
        if type_eq!(T, Vec<u8>) {
            unsafe {
                rec.bytes_owned(downcast_ref_unchecked::<Vec<u8>>(&value).to_owned());
            }
            return;
        }

        rec.other_boxed(Box::new(value));
    }

    provide_fn!(provide_bool, bool, bool);
    provide_fn!(provide_i8, i8, i8);
    provide_fn!(provide_u8, u8, u8);
    provide_fn!(provide_i16, i16, i16);
    provide_fn!(provide_u16, u16, u16);
    provide_fn!(provide_i32, i32, i32);
    provide_fn!(provide_u32, u32, u32);
    provide_fn!(provide_i64, i64, i64);
    provide_fn!(provide_u64, u64, u64);
    provide_fn!(provide_i128, i128, i128);
    provide_fn!(provide_u128, u128, u128);
    provide_fn!(provide_f32, f32, f32);
    provide_fn!(provide_f64, f64, f64);
    provide_fn!(provide_char, char, char);
    provide_fn!(provide_str, &str, str);
    provide_fn!(provide_bytes, &[u8], bytes);

    #[inline]
    pub fn provide_str_owned(&mut self, value: String) {
        if self.requests::<String>() {
            Q::get_receiver(&mut self.0).str_owned(value);
        } else if self.requests::<&str>() {
            Q::get_receiver(&mut self.0).str(value.as_str());
        }
    }

    #[inline]
    pub fn provide_bytes_owned(&mut self, value: Vec<u8>) {
        if self.requests::<Vec<u8>>() {
            Q::get_receiver(&mut self.0).bytes_owned(value);
        } else if self.requests::<&[u8]>() {
            Q::get_receiver(&mut self.0).bytes(value.as_slice());
        }
    }

    #[inline]
    pub fn as_erased(&mut self) -> Request<'_, super::erased::Erased>
    where
        Q::Request: Receiver,
    {
        Request::new_erased(&mut self.0)
    }
}

impl<'d> Request<'d, super::erased::Erased<'d>> {
    #[inline]
    pub fn new_erased<R: Receiver>(receiver: &'d mut R) -> Self {
        Self::new(receiver as _)
    }
}

impl<'d, Q: Query + ?Sized> Default for Request<'d, Q>
where
    Q::Request: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(Q::Request::default())
    }
}

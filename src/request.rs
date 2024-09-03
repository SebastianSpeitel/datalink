use crate::{
    link::Link,
    query::{DataQuery, ErasedDataQuery, Receiver, TypeFilter},
};

pub type ErasedRequest<'q> = ErasedDataQuery<'q>;

macro_rules! match_type {
    (@make_ty $ty:ty) => {$ty};
    (@make_ty) => {_};
    (match $val:ident { $($bind:ident $(@ $ty:ty)? => $arm:expr $(,)?)*} ) => {
        $(
            let mut opt = Some($val);
            let $val =
                match <dyn ::core::any::Any>::downcast_mut::<::core::option::Option<match_type!(@make_ty $($ty)?)>>(&mut opt) {
                    Some(opt_b) => {
                        let $bind = opt_b.take().unwrap();
                        return $arm;
                    }
                    _ => opt.take().unwrap(),
                };
        )*
    };
}

macro_rules! forward_fns {
    ($($f:ident($ty:ty))*) => {
        $(
            #[inline]
            fn $f(&mut self, value: $ty) {
                self.provide(value);
            }
        )*
    };
}

// macro_rules! try_cast_then {
//     ($ident:ident $(if $cond:expr)? => $p:expr) => {
//         let mut opt = Some($ident);
//         let $ident =
//             match <dyn ::core::any::Any>::downcast_mut::<::core::option::Option<_>>(&mut opt) {
//                 Some(opt_b) $(if $cond)? => {
//                     return ($p)(opt_b.take().unwrap());
//                 }
//                 _ => opt.take().unwrap(),
//             };
//     };
// }

pub trait Request {
    type Query: DataQuery + ?Sized;

    fn query_ref(&self) -> &Self::Query;
    fn provide_ref_unchecked<T: 'static>(&mut self, value: &T);
    fn push_link<L: Link>(&mut self, link: L);

    #[inline]
    fn requests_value_of<T: 'static + ?Sized>(&self) -> bool {
        self.query_ref().filter().accepts_value_of::<T>()
    }

    #[inline]
    fn requests<T: 'static + ?Sized>(&self, value: &T) -> bool {
        dbg!(core::any::type_name_of_val(value));
        self.query_ref().filter().accepts(value)
    }

    #[inline]
    fn provide_unchecked<T: 'static>(&mut self, value: T) {
        self.provide_ref_unchecked(&value);
    }

    #[inline]
    fn provide<T: 'static>(&mut self, value: T) {
        if self.requests(&value) {
            self.provide_unchecked(value);
        }
    }

    #[inline]
    fn provide_with<T: 'static>(&mut self, f: impl FnOnce() -> T) {
        if !self.requests_value_of::<T>() {
            return;
        }
        self.provide(f());
    }

    #[inline]
    fn provide_default_of<T: Default + 'static>(&mut self) {
        self.provide_with(T::default);
    }

    #[cfg(feature = "unique")]
    #[inline]
    fn provide_id(&mut self, id: impl Into<crate::id::ID>) {
        self.provide_with(|| id.into());
    }

    #[inline]
    fn provide_type_id_of<T: core::any::Any>(&mut self, t: &T) {
        self.provide_with(|| t.type_id());
    }

    #[inline]
    fn provide_discriminant<T: 'static>(&mut self, v: &T) {
        use core::mem::discriminant;
        self.provide_with(|| discriminant(v));
    }

    forward_fns!(
        provide_bool(bool)
        provide_u8(u8)
        provide_i8(i8)
        provide_u16(u16)
        provide_i16(i16)
        provide_u32(u32)
        provide_i32(i32)
        provide_u64(u64)
        provide_i64(i64)
        provide_u128(u128)
        provide_i128(i128)
        provide_f32(f32)
        provide_f64(f64)
        provide_char(char)
        provide_str_owned(Box<str>)
        provide_bytes_owned(Box<[u8]>)
        provide_string(String)
        provide_erased_data(Box<crate::ErasedData>)
    );

    #[inline]
    fn provide_str(&mut self, value: &str) {
        self.provide_str_owned(value.to_owned().into_boxed_str());
    }

    #[inline]
    fn provide_bytes(&mut self, value: &[u8]) {
        self.provide_bytes_owned(value.to_owned().into_boxed_slice());
    }

    #[inline]
    fn is_erasing(&self) -> bool {
        DataQuery::is_erasing(self.query_ref())
    }

    fn as_erased(&mut self) -> ErasedRequest
    where
        Self: Sized;
}

impl<Q> Request for Q
where
    Q: DataQuery + ?Sized,
{
    type Query = Q;

    #[inline]
    fn query_ref(&self) -> &Self::Query {
        self
    }

    #[inline]
    fn provide_ref_unchecked<T: 'static>(&mut self, value: &T) {
        use core::any::Any;

        macro_rules! cast_copy_provide {
            ($($f:ident)*) => {
                $(
                <dyn Any>::downcast_ref(value).map(|v| self.receiver().$f(*v));
                )*
            };
        }

        cast_copy_provide!(bool u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 f32 f64 char str bytes);

        self.receiver().other_ref(value);
    }

    #[inline]
    fn provide_unchecked<T: 'static>(&mut self, value: T) {
        match_type! {
            match value {
                v => self.receiver().bool(v),
                v => self.receiver().u8(v),
                v => self.receiver().i8(v),
                v => self.receiver().u16(v),
                v => self.receiver().i16(v),
                v => self.receiver().u32(v),
                v => self.receiver().i32(v),
                v => self.receiver().u64(v),
                v => self.receiver().i64(v),
                v => self.receiver().u128(v),
                v => self.receiver().i128(v),
                v => self.receiver().f32(v),
                v => self.receiver().f64(v),
                v => self.receiver().char(v),
                v => self.receiver().str(v),
                v => self.receiver().str_owned(v),
                v => self.receiver().str_owned(String::into_boxed_str(v)),
                v => self.receiver().bytes(v),
                v => self.receiver().bytes_owned(v),
                v => self.receiver().bytes_owned(Vec::into_boxed_slice(v)),

                v => self.receiver().erased_data(v),

                v @ &bool => self.receiver().bool(*v),
                v @ &u8 => self.receiver().u8(*v),
                v @ &i8 => self.receiver().i8(*v),
                v @ &u16 => self.receiver().u16(*v),
                v @ &i16 => self.receiver().i16(*v),
                v @ &u32 => self.receiver().u32(*v),
                v @ &i32 => self.receiver().i32(*v),
                v @ &u64 => self.receiver().u64(*v),
                v @ &i64 => self.receiver().i64(*v),
                v @ &u128 => self.receiver().u128(*v),
                v @ &i128 => self.receiver().i128(*v),
                v @ &f32 => self.receiver().f32(*v),
                v @ &f64 => self.receiver().f64(*v),
                v @ &char => self.receiver().char(*v),

                v @ &String => self.receiver().str(v),
                v @ &Box<str> => self.receiver().str(v),
                v @ &Vec<u8> => self.receiver().bytes(v),
                v @ &Box<[u8]> => self.receiver().bytes(v),
            }
        };

        self.receiver().other_boxed(Box::new(value));
    }

    #[inline]
    fn provide<T: 'static>(&mut self, value: T) {
        match_type! {
            match value {
                v => if self.requests(&v) { self.receiver().bool(v) },
                v => if self.requests(&v) { self.receiver().u8(v) },
                v => if self.requests(&v) { self.receiver().i8(v) },
                v => if self.requests(&v) { self.receiver().u16(v) },
                v => if self.requests(&v) { self.receiver().i16(v) },
                v => if self.requests(&v) { self.receiver().u32(v) },
                v => if self.requests(&v) { self.receiver().i32(v) },
                v => if self.requests(&v) { self.receiver().u64(v) },
                v => if self.requests(&v) { self.receiver().i64(v) },
                v => if self.requests(&v) { self.receiver().u128(v) },
                v => if self.requests(&v) { self.receiver().i128(v) },
                v => if self.requests(&v) { self.receiver().f32(v) },
                v => if self.requests(&v) { self.receiver().f64(v) },
                v => if self.requests(&v) { self.receiver().char(v) },
                v => if self.requests(&v) { self.receiver().str(v) },
                v => if self.requests(&v) { self.receiver().str_owned(v) },
                v => {
                    let s = String::into_boxed_str(v);
                    if self.requests(&s) {
                        self.receiver().str_owned(s);
                    }
                },
                v => if self.requests(&v) { self.receiver().bytes(v) },
                v => if self.requests(&v) { self.receiver().bytes_owned(v) },
                v => {
                    let b = Vec::into_boxed_slice(v);
                    if self.requests(&b) {
                        self.receiver().bytes_owned(b);
                    }
                },

                v => if self.requests(&v) { self.receiver().erased_data(v) },

                v => if self.requests(v) { self.receiver().bool(*v) },
                v => if self.requests(v) { self.receiver().u8(*v) },
                v => if self.requests(v) { self.receiver().i8(*v) },
                v => if self.requests(v) { self.receiver().u16(*v) },
                v => if self.requests(v) { self.receiver().i16(*v) },
                v => if self.requests(v) { self.receiver().u32(*v) },
                v => if self.requests(v) { self.receiver().i32(*v) },
                v => if self.requests(v) { self.receiver().u64(*v) },
                v => if self.requests(v) { self.receiver().i64(*v) },
                v => if self.requests(v) { self.receiver().u128(*v) },
                v => if self.requests(v) { self.receiver().i128(*v) },
                v => if self.requests(v) { self.receiver().f32(*v) },
                v => if self.requests(v) { self.receiver().f64(*v) },
                v => if self.requests(v) { self.receiver().char(*v) },

                v @ &String => if self.requests(v.as_str()) { self.receiver().str(v) },
                v @ &Box<str> => if self.requests(v.as_ref()) { self.receiver().str(v) },
                v @ &Vec<u8> => if self.requests(v.as_slice()) { self.receiver().bytes(v) },
                v @ &Box<[u8]> => if self.requests(v.as_ref()) { self.receiver().bytes(v) },
            }
        };

        self.receiver().other_boxed(Box::new(value));
    }

    #[inline]
    fn provide_str(&mut self, value: &str) {
        if self.requests(value) {
            self.receiver().str(value);
        }
    }

    #[inline]
    fn provide_bytes(&mut self, value: &[u8]) {
        if self.requests(value) {
            self.receiver().bytes(value);
        }
    }

    #[inline]
    fn push_link<L: Link>(&mut self, link: L) {
        link.query_owned(self.link_query());
    }

    #[inline]
    fn as_erased(&mut self) -> crate::query::ErasedDataQuery
    where
        Self: Sized,
    {
        self.into_erased()
    }
}

// #[allow(clippy::unwrap_used)]
// fn downcast_owned<T: 'static, U: 'static>(t: T) -> Result<U, T> {
//     use core::any::Any;
//     let mut opt_t = Some(t);
//     <dyn Any>::downcast_mut::<Option<U>>(&mut opt_t)
//         .and_then(Option::take)
//         .ok_or_else(|| opt_t.take().unwrap())
// }

#[cfg(test)]
mod tests {
    use crate::Data;

    #[test]
    fn providable() {
        let mut rec = Option::<u8>::None;
        42u8.query(&mut rec);
        assert_eq!(rec, Some(42));

        let mut rec = Option::<u8>::None;
        let r = &42u8;
        r.query(&mut rec);
        assert_eq!(rec, Some(42));

        let mut rec = Option::<String>::None;
        "Hello world".query(&mut rec);
        assert_eq!(rec.unwrap(), "Hello world");

        let mut rec = Option::<String>::None;
        let s = String::from("Hello world");
        s.query(&mut rec);
        assert_eq!(rec.unwrap(), "Hello world");
    }
}

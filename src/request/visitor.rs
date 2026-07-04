#[allow(unused_variables)]
pub trait Visitor {
    #[inline]
    fn visit_bool(&mut self, value: bool) {}
    #[inline]
    fn visit_i8(&mut self, value: i8) {}
    #[inline]
    fn visit_i16(&mut self, value: i16) {}
    #[inline]
    fn visit_i32(&mut self, value: i32) {}
    #[inline]
    fn visit_i64(&mut self, value: i64) {}
    #[inline]
    fn visit_i128(&mut self, value: i128) {}
    #[inline]
    fn visit_u8(&mut self, value: u8) {}
    #[inline]
    fn visit_u16(&mut self, value: u16) {}
    #[inline]
    fn visit_u32(&mut self, value: u32) {}
    #[inline]
    fn visit_u64(&mut self, value: u64) {}
    #[inline]
    fn visit_u128(&mut self, value: u128) {}
    #[inline]
    fn visit_f32(&mut self, value: f32) {}
    #[inline]
    fn visit_f64(&mut self, value: f64) {}
    #[inline]
    fn visit_char(&mut self, value: char) {}
    #[inline]
    fn visit_str(&mut self, value: &str) {}
    #[inline]
    fn visit_bytes(&mut self, value: &[u8]) {}
    #[inline]
    fn visit_any(&mut self, value: &dyn core::any::Any) {}

    #[inline]
    fn visit_str_owned(&mut self, value: Box<str>) {
        self.visit_str(value.as_ref());
    }
    #[cfg(feature = "std")]
    #[inline]
    fn visit_bytes_owned(&mut self, value: Box<[u8]>) {
        self.visit_bytes(value.as_ref());
    }
    #[cfg(feature = "std")]
    #[inline]
    fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
        self.visit_any(value.as_ref());
    }

    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, value: &T)
    where
        Self: Sized,
    {
        self.visit_any(value);
    }

    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T)
    where
        Self: Sized,
    {
        // depend on alloc
        #[cfg(feature = "std")]
        self.visit_any_owned(Box::new(value));

        #[cfg(not(feature = "std"))]
        self.visit_any(&value);
    }
}

pub trait VisitorExt: Visitor + Sized {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn visit<T: 'static>(&mut self, value: T) {
        let mut opt = Some(value);

        macro_rules! try_visit {
            ($($m:ident)*) => {
                $(
                if let Some(v) = crate::util::cast_opt(&mut opt) {
                    self.$m(v);
                    return;
                }
                )*
            };
        }

        try_visit!(
            visit_i8 visit_i16 visit_i32 visit_i64 visit_i128
            visit_u8 visit_u16 visit_u32 visit_u64 visit_u128
            visit_f32 visit_f64 visit_char visit_bool
            visit_str visit_bytes visit_any
        );
        #[cfg(feature = "std")]
        try_visit!(
            visit_str_owned visit_bytes_owned visit_any_owned
        );

        #[cfg(feature = "std")]
        if let Some(v) = crate::util::cast_opt(&mut opt) {
            self.visit_str_owned(String::into_boxed_str(v));
            return;
        }
        #[cfg(feature = "std")]
        if let Some(v) = crate::util::cast_opt(&mut opt) {
            self.visit_bytes_owned(Vec::into_boxed_slice(v));
            return;
        }

        let Some(value) = opt else {
            if cfg!(debug_assertions) {
                unreachable!();
            }
            return;
        };

        self.visit_other(value);
    }

    #[inline(always)]
    fn visit_ref<T: 'static>(&mut self, r#ref: &T) {
        macro_rules! try_visit {
            ($($m:ident)*) => {
                $(
                if let Some(v) = crate::util::cast_ref(r#ref) {
                    self.$m(*v);
                    return;
                }
                )*
            };
        }

        try_visit!(
            visit_i8 visit_i16 visit_i32 visit_i64 visit_i128
            visit_u8 visit_u16 visit_u32 visit_u64 visit_u128
            visit_f32 visit_f64 visit_char visit_bool
            visit_str visit_bytes visit_any
        );

        self.visit_other_ref(r#ref);
    }
}
impl<V: Visitor> VisitorExt for V {}

macro_rules! forward {
    (@copy) => {
        forward!(visit_bool: bool);
        forward!(visit_i8: i8);
        forward!(visit_i16: i16);
        forward!(visit_i32: i32);
        forward!(visit_i64: i64);
        forward!(visit_i128: i128);
        forward!(visit_u8: u8);
        forward!(visit_u16: u16);
        forward!(visit_u32: u32);
        forward!(visit_u64: u64);
        forward!(visit_u128: u128);
        forward!(visit_f32: f32);
        forward!(visit_f64: f64);
        forward!(visit_char: char);
        forward!(visit_str: &str);
        forward!(visit_bytes: &[u8]);
        forward!(visit_any: &dyn core::any::Any);
    };
    (@alloc) => {
        forward!(visit_str_owned: Box<str>);
        forward!(visit_bytes_owned: Box<[u8]>);
        forward!(visit_any_owned: Box<dyn core::any::Any>);
    };
    (@opt $m:ident: $v:ty) => {
        #[inline]
        fn $m(&mut self, value: $v) {
            if let Some(v) = self {
                v.$m(value);
            }
        }
    };
    ($m:ident: $v:ty) => {
        #[inline]
        fn $m(&mut self, value: $v) {
            (**self).$m(value);
        }
    };
}

impl<V: Visitor> Visitor for &mut V {
    forward!(@copy);
    #[cfg(feature = "std")]
    forward!(@alloc);

    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, value: &T) {
        (**self).visit_any(value);
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        #[cfg(feature = "std")]
        (**self).visit_any_owned(Box::new(value));

        #[cfg(not(feature = "std"))]
        (**self).visit_any(&value);
    }
}

impl Visitor for &mut (dyn Visitor + '_) {
    forward!(@copy);
    #[cfg(feature = "std")]
    forward!(@alloc);

    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, value: &T) {
        (**self).visit_any(value);
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        #[cfg(feature = "std")]
        (**self).visit_any_owned(Box::new(value));

        #[cfg(not(feature = "std"))]
        (**self).visit_any(&value);
    }
}

#[cfg(feature = "std")]
impl Visitor for Box<dyn Visitor + '_> {
    forward!(@copy);
    forward!(@alloc);

    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, value: &T) {
        (**self).visit_any(value);
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        (**self).visit_any_owned(Box::new(value));
    }
}

impl Visitor for () {}

#[deny(clippy::missing_trait_methods)]
impl<V: Visitor> Visitor for Option<V> {
    forward!(@opt visit_bool: bool);
    forward!(@opt visit_i8: i8);
    forward!(@opt visit_i16: i16);
    forward!(@opt visit_i32: i32);
    forward!(@opt visit_i64: i64);
    forward!(@opt visit_i128: i128);
    forward!(@opt visit_u8: u8);
    forward!(@opt visit_u16: u16);
    forward!(@opt visit_u32: u32);
    forward!(@opt visit_u64: u64);
    forward!(@opt visit_u128: u128);
    forward!(@opt visit_f32: f32);
    forward!(@opt visit_f64: f64);
    forward!(@opt visit_char: char);
    forward!(@opt visit_str: &str);
    forward!(@opt visit_bytes: &[u8]);
    forward!(@opt visit_any: &dyn core::any::Any);
    #[cfg(feature = "std")]
    forward!(@opt visit_str_owned: Box<str>);
    #[cfg(feature = "std")]
    forward!(@opt visit_bytes_owned: Box<[u8]>);
    #[cfg(feature = "std")]
    forward!(@opt visit_any_owned: Box<dyn core::any::Any>);
    #[inline]
    fn visit_other_ref<T: 'static>(&mut self, value: &T) {
        if let Some(visitor) = self {
            visitor.visit_any(value);
        }
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        if let Some(visitor) = self {
            #[cfg(feature = "std")]
            visitor.visit_any_owned(Box::new(value));

            #[cfg(not(feature = "std"))]
            visitor.visit_any(&value);
        }
    }
}

use super::{ErasableRequest, Request, Schema, Visitor};
use crate::r#type::{BytesLike, StringLike, TypeOf};

macro_rules! impl_copy_visitor {
    ($($m:ident: $ty:ty)*) => {
        $(
        impl Visitor for Option<$ty> {
            #[inline]
            fn $m(&mut self, value: $ty) {
                self.get_or_insert_with(|| value);
            }
            #[inline]
            fn visit_any(&mut self, value: &dyn core::any::Any) {
                if self.is_some() {
                    return;
                }
                *self = value.downcast_ref().copied();
            }
            #[inline]
            fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
                if self.is_some() {
                    return;
                }
                *self = value.downcast().ok().map(|v| *v);
            }
            #[inline]
            fn visit_other<T: 'static>(&mut self, value: T) {
                if self.is_some() {
                    return;
                }
                *self = crate::util::cast_opt(&mut Some(value));
            }
            #[inline]
            fn visit_other_ref<T: 'static>(&mut self, value: &T) {
                if self.is_some() {
                    return;
                }
                *self = crate::util::cast_ref(value).copied();
            }
        }


        )*
    };
}

macro_rules! impl_request {
    ($($ty:ty: $types:ty)*) => {
        $(
        impl ErasableRequest for Option<$ty> {
            #[inline]
            fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
                crate::schema::SimpleSchema::values(<$types>::SET)
            }
            #[inline]
            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                Some(self)
            }
        }

        impl Request for Option<$ty> {
            #[inline]
            fn schema(&self) -> impl Schema {
                <$types>::default()
            }
            #[inline]
            fn visitor(&mut self) -> impl Visitor {
                self
            }
            #[inline]
            fn as_erased(&mut self) -> impl ErasableRequest {
                self
            }
        }
    )*
    };
}

impl_copy_visitor!(
    visit_bool: bool
    visit_char: char
    visit_u8: u8
    visit_i8: i8
    visit_u16: u16
    visit_i16: i16
    visit_u32: u32
    visit_i32: i32
    visit_u64: u64
    visit_i64: i64
    visit_u128: u128
    visit_i128: i128
    visit_f32: f32
    visit_f64: f64
);

impl Visitor for Option<String> {
    #[inline]
    fn visit_str(&mut self, value: &str) {
        self.get_or_insert_with(|| value.to_string());
    }
    #[inline]
    fn visit_str_owned(&mut self, value: Box<str>) {
        self.get_or_insert_with(|| value.into_string());
    }
    #[inline]
    fn visit_any(&mut self, value: &dyn core::any::Any) {
        if self.is_some() {
            return;
        }
        if let Some(s) = value.downcast_ref::<String>() {
            *self = Some(s.to_owned());
        } else if let Some(s) = value.downcast_ref::<Box<str>>() {
            *self = Some(s.to_owned().into_string());
        } else if let Some(s) = value.downcast_ref::<&str>() {
            *self = Some((*s).to_string());
        }
    }
    #[inline]
    fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
        if self.is_some() {
            return;
        }
        if value.is::<String>() {
            *self = value.downcast().ok().map(|s| *s);
        } else if value.is::<Box<str>>() {
            *self = value
                .downcast()
                .ok()
                .map(|s: Box<Box<str>>| (*s).into_string());
        } else if value.is::<&str>() {
            *self = value.downcast().ok().map(|s: Box<&str>| (*s).to_string());
        } else if let Some(s) = value.downcast_ref::<&str>() {
            *self = Some((*s).to_string());
        }
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        if self.is_some() {
            return;
        }
        let mut opt = Some(value);
        if let Some(s) = crate::util::cast_opt(&mut opt) {
            *self = Some(s);
        } else if let Some(s) = crate::util::cast_opt::<Box<str>>(&mut opt) {
            *self = Some(s.into_string());
        } else if let Some(s) = crate::util::cast_opt::<&str>(&mut opt) {
            *self = Some(s.to_string());
        }
    }
}

impl Visitor for Option<Vec<u8>> {
    #[inline]
    fn visit_bytes(&mut self, value: &[u8]) {
        self.get_or_insert_with(|| value.to_vec());
    }
    #[inline]
    fn visit_bytes_owned(&mut self, value: Box<[u8]>) {
        self.get_or_insert_with(|| value.into_vec());
    }
    #[inline]
    fn visit_any(&mut self, value: &dyn core::any::Any) {
        if self.is_some() {
            return;
        }
        if let Some(bytes) = value.downcast_ref::<Vec<u8>>() {
            *self = Some(bytes.clone());
        } else if let Some(bytes) = value.downcast_ref::<Box<[u8]>>() {
            *self = Some(bytes.clone().into_vec());
        } else if let Some(bytes) = value.downcast_ref::<&[u8]>() {
            *self = Some(bytes.to_vec());
        }
    }
    #[inline]
    fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
        if self.is_some() {
            return;
        }
        if value.is::<Vec<u8>>() {
            *self = value.downcast().ok().map(|v| *v);
        } else if value.is::<Box<[u8]>>() {
            *self = value
                .downcast()
                .ok()
                .map(|v: Box<Box<[u8]>>| (*v).into_vec());
        } else if value.is::<&[u8]>() {
            *self = value.downcast().ok().map(|v: Box<&[u8]>| (*v).to_vec());
        } else if let Some(bytes) = value.downcast_ref::<&[u8]>() {
            *self = Some(bytes.to_vec());
        }
    }
    #[inline]
    fn visit_other<T: 'static>(&mut self, value: T) {
        if self.is_some() {
            return;
        }
        let mut opt = Some(value);
        if let Some(bytes) = crate::util::cast_opt(&mut opt) {
            *self = Some(bytes);
        } else if let Some(bytes) = crate::util::cast_opt::<Box<[u8]>>(&mut opt) {
            *self = Some(bytes.into_vec());
        } else if let Some(bytes) = crate::util::cast_opt::<&[u8]>(&mut opt) {
            *self = Some(bytes.to_vec());
        }
    }
}

impl_request!(
    bool: TypeOf::<bool>
    char: TypeOf::<char>
    u8: TypeOf::<u8>
    i8: TypeOf::<i8>
    u16: TypeOf::<u16>
    i16: TypeOf::<i16>
    u32: TypeOf::<u32>
    i32: TypeOf::<i32>
    u64: TypeOf::<u64>
    i64: TypeOf::<i64>
    u128: TypeOf::<u128>
    i128: TypeOf::<i128>
    f32: TypeOf::<f32>
    f64: TypeOf::<f64>
    String: StringLike
    Vec<u8>: BytesLike
);

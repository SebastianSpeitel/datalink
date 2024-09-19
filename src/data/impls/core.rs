use crate::meta;
use crate::{Data, Request};

impl Data for () {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_default_of::<meta::IsUnit>();
    }
}

impl Data for core::convert::Infallible {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_default_of::<meta::IsInfallible>();
    }
}

macro_rules! impl_copy_data {
    ($ty:ty,$fn:ident) => {
        impl Data for $ty {
            #[inline]
            fn query(&self, request: &mut impl Request) {
                (*self).query_owned(request);
            }
            #[inline]
            fn query_owned(self, request: &mut impl Request) {
                request.$fn(self);
            }
        }
    };
}

impl_copy_data!(bool, provide_bool);
impl_copy_data!(u8, provide_u8);
impl_copy_data!(i8, provide_i8);
impl_copy_data!(u16, provide_u16);
impl_copy_data!(i16, provide_i16);
impl_copy_data!(u32, provide_u32);
impl_copy_data!(i32, provide_i32);
impl_copy_data!(u64, provide_u64);
impl_copy_data!(i64, provide_i64);
impl_copy_data!(u128, provide_u128);
impl_copy_data!(i128, provide_i128);
impl_copy_data!(f32, provide_f32);
impl_copy_data!(f64, provide_f64);
impl_copy_data!(char, provide_char);

impl Data for str {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_str(self);
    }
}

impl Data for &str {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_str(self);
    }
}

impl Data for [u8] {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_bytes(self);
    }
}

impl Data for &[u8] {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_bytes(self);
    }
}

impl<const N: usize> Data for [u8; N] {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_bytes(self);
    }
}

impl Data for usize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if usize::BITS >= u128::BITS {
            request.provide_u128(*self as u128);
        }
        if usize::BITS >= u64::BITS {
            request.provide_u64(*self as u64);
        }
        if usize::BITS >= u32::BITS {
            request.provide_u32(*self as u32);
        }
        if usize::BITS >= u16::BITS {
            request.provide_u16(*self as u16);
        }
        if usize::BITS >= u8::BITS {
            request.provide_u8(*self as u8);
        }
    }
}

impl Data for isize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if isize::BITS >= i128::BITS {
            request.provide_i128(*self as i128);
        }
        if isize::BITS >= i64::BITS {
            request.provide_i64(*self as i64);
        }
        if isize::BITS >= i32::BITS {
            request.provide_i32(*self as i32);
        }
        if isize::BITS >= i16::BITS {
            request.provide_i16(*self as i16);
        }
        if isize::BITS >= i8::BITS {
            request.provide_i8(*self as i8);
        }
    }
}

macro_rules! impl_nonzero_data {
    ($ty:ty) => {
        impl Data for $ty {
            #[inline]
            fn query(&self, request: &mut impl Request) {
                self.get().query(request);
            }
        }
    };
}

impl_nonzero_data!(core::num::NonZeroU8);
impl_nonzero_data!(core::num::NonZeroI8);
impl_nonzero_data!(core::num::NonZeroU16);
impl_nonzero_data!(core::num::NonZeroI16);
impl_nonzero_data!(core::num::NonZeroU32);
impl_nonzero_data!(core::num::NonZeroI32);
impl_nonzero_data!(core::num::NonZeroU64);
impl_nonzero_data!(core::num::NonZeroI64);
impl_nonzero_data!(core::num::NonZeroU128);
impl_nonzero_data!(core::num::NonZeroI128);
impl_nonzero_data!(core::num::NonZeroUsize);
impl_nonzero_data!(core::num::NonZeroIsize);

#[warn(clippy::missing_trait_methods)]
impl<D: Data> Data for Option<D> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if let Some(d) = self {
            request.provide_default_of::<meta::IsSome>();
            d.query(request);
        } else {
            request.provide_default_of::<meta::IsNone>();
        }
    }
    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        if let Some(d) = self {
            request.provide_default_of::<meta::IsSome>();
            d.query_owned(request);
        } else {
            request.provide_default_of::<meta::IsNone>();
        }
    }
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.as_ref().and_then(Data::get_id)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D> Data for std::borrow::Cow<'_, D>
where
    D: Data + ToOwned<Owned: Data> + ?Sized,
{
    #[inline]
    fn query(&self, request: &mut impl Request) {
        match self {
            Self::Borrowed(data) => {
                request.provide_default_of::<meta::IsBorrowed>();
                data.query(request);
            }
            Self::Owned(data) => {
                request.provide_default_of::<meta::IsOwned>();
                data.query(request);
            }
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        match self {
            Self::Borrowed(data) => data.query(request),
            Self::Owned(data) => data.query_owned(request),
        }
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        match self {
            Self::Borrowed(data) => data.get_id(),
            Self::Owned(data) => data.get_id(),
        }
    }
}

#[cfg(feature = "unique")]
#[warn(clippy::missing_trait_methods)]
impl<D> crate::data::unique::Unique for std::borrow::Cow<'_, D>
where
    D: crate::data::unique::Unique + ?Sized + ToOwned,
    D::Owned: crate::data::unique::Unique,
{
    #[inline]
    fn id(&self) -> crate::id::ID {
        match self {
            Self::Borrowed(data) => data.id(),
            Self::Owned(data) => data.id(),
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data> Data for core::cell::OnceCell<D> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if let Some(d) = self.get() {
            d.query(request);
        } else {
            request.provide_default_of::<meta::IsEmptyCell>();
        }
    }
    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        if let Some(d) = self.into_inner() {
            d.query_owned(request);
        } else {
            request.provide_default_of::<meta::IsEmptyCell>();
        }
    }
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.get().and_then(Data::get_id)
    }
}

// move to std
impl<D: Data> Data for std::sync::OnceLock<D> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if let Some(d) = self.get() {
            d.query(request);
        } else {
            request.provide_default_of::<meta::IsEmptyCell>();
        }
    }
}

impl Data for dyn core::any::Any {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_with(|| self.type_id());
        // todo: provide more information
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;

    #[test]
    fn once_cell() {
        let cell = core::cell::OnceCell::new();

        let values = cell.all_values();
        dbg!(&values);
        assert!(values.iter().find(|v| v.as_i32().is_some()).is_none());
        assert!(values.as_i32().is_none());

        cell.get_or_init(|| 100);
        let values = cell.all_values();
        dbg!(&values);

        assert!(values.iter().find(|v| v.as_i32().is_some()).is_some());
        assert!(values.as_i32().is_some());
    }
}

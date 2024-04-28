use crate::data::Data;
use crate::links::{LinkError, Links};
use crate::rr::{meta, Req, Request};

impl<R: Req> Data<R> for () {}

macro_rules! impl_copy_data {
    ($ty:ty) => {
        impl<R: Req> Data<R> for $ty {
            #[inline]
            fn provide_value<'d>(&self, mut request: Request<'d, R>) {
                request.provide_owned(*self);
            }
        }
    };
}

impl_copy_data!(bool);
impl_copy_data!(u8);
impl_copy_data!(i8);
impl_copy_data!(u16);
impl_copy_data!(i16);
impl_copy_data!(u32);
impl_copy_data!(i32);
impl_copy_data!(u64);
impl_copy_data!(i64);
impl_copy_data!(u128);
impl_copy_data!(i128);
impl_copy_data!(f32);
impl_copy_data!(f64);
impl_copy_data!(char);

impl<R: Req> Data<R> for str {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        request.provide_str(self);
    }

    #[inline]
    fn with_req<R2: Req>(&self) -> Option<impl Data<R2>>
    {
        Some(self)
    }
}

impl<R: Req> Data<R> for [u8] {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        request.provide_bytes(self);
    }
}

impl<R: Req> Data<R> for usize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if usize::BITS == u8::BITS {
            request.provide_owned(*self as u8);
        } else if usize::BITS == u16::BITS {
            request.provide_owned(*self as u16);
        } else if usize::BITS == u32::BITS {
            request.provide_owned(*self as u32);
        } else if usize::BITS == u64::BITS {
            request.provide_owned(*self as u64);
        }
    }
}

impl<R: Req> Data<R> for isize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if isize::BITS == i8::BITS {
            request.provide_owned(*self as i8);
        } else if isize::BITS == i16::BITS {
            request.provide_owned(*self as i16);
        } else if isize::BITS == i32::BITS {
            request.provide_owned(*self as i32);
        } else if isize::BITS == i64::BITS {
            request.provide_owned(*self as i64);
        }
    }
}

macro_rules! impl_nonzero_data {
    ($ty:ty) => {
        impl<R: Req> Data<R> for $ty {
            #[inline]
            fn provide_value<'d>(&self, mut request: Request<'d, R>) {
                request.provide_owned(self.get());
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
impl<R: Req, D: Data<R>> Data<R> for Option<D> {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if let Some(d) = self {
            request.provide_owned(meta::IsSome);
            d.provide_value(request);
        } else {
            request.provide_owned(meta::IsNone);
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        if let Some(d) = self {
            d.provide_links(links)?;
        }

        Ok(())
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        if let Some(d) = self {
            d.query_links(links, query)?;
        }

        Ok(())
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.as_ref().and_then(Data::get_id)
    }

    #[inline]
    fn with_req<R2: Req>(&self) -> Option<impl Data<R2>> {
        None::<()>
    }
}

#[warn(clippy::missing_trait_methods)]
impl<R: Req, D: Data<R> + ?Sized> Data<R> for std::borrow::Cow<'_, D>
where
    D: ToOwned,
    D::Owned: Data<R>,
{
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        match self {
            Self::Borrowed(data) => {
                request.provide_owned(meta::IsBorrowed);
                data.provide_value(request);
            }
            Self::Owned(data) => {
                request.provide_owned(meta::IsOwned);
                data.provide_value(request);
            }
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Self::Borrowed(data) => data.provide_links(links),
            Self::Owned(data) => data.provide_links(links),
        }
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        match self {
            Self::Borrowed(data) => data.query_links(links, query),
            Self::Owned(data) => data.query_links(links, query),
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
impl<D: crate::data::unique::Unique + ?Sized> crate::data::unique::Unique
    for std::borrow::Cow<'_, D>
where
    D: ToOwned,
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
impl<R: Req, D: Data<R>> Data<R> for core::cell::OnceCell<D> {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if let Some(d) = self.get() {
            request.provide_owned(meta::IsSome);
            d.provide_value(request);
        } else {
            request.provide_owned(meta::IsNone);
        }
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        if let Some(d) = self.get() {
            d.provide_links(links)?;
        }

        Ok(())
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        if let Some(d) = self.get() {
            d.query_links(links, query)?;
        }

        Ok(())
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.get().and_then(Data::get_id)
    }

    #[inline]
    fn with_req<R2: Req>(&self) -> Option<impl Data<R2>> {
        None::<()>
    }
}

impl<R: Req, D: Data<R>> Data<R> for std::sync::OnceLock<D> {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if let Some(d) = self.get() {
            request.provide_owned(meta::IsSome);
            d.provide_value(request);
        } else {
            request.provide_owned(meta::IsNone);
        }
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        if let Some(d) = self.get() {
            d.provide_links(links)?;
        }

        Ok(())
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        if let Some(d) = self.get() {
            d.query_links(links, query)?;
        }

        Ok(())
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.get().and_then(Data::get_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;

    #[test]
    fn once_cell() {
        let cell = core::cell::OnceCell::new();

        let values = (&&&cell).all_values();
        dbg!(&values);
        assert!(values.single().is_none());

        cell.get_or_init(|| 100);
        let values = cell.all_values();

        assert_eq!(values.as_i32().unwrap(), 100);
    }
}

use crate::data::Data;
use crate::links::{LinkError, Links};
use crate::value::ValueBuiler;
use std::borrow::Cow;

impl Data for () {}

impl Data for bool {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.bool(*self);
    }
}

impl Data for u8 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u8(*self);
    }
}

impl Data for i8 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.i8(*self);
    }
}

impl Data for u16 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u16(*self);
    }
}

impl Data for i16 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.i16(*self);
    }
}

impl Data for u32 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u32(*self);
    }
}

impl Data for i32 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.i32(*self);
    }
}

impl Data for u64 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u64(*self);
    }
}

impl Data for i64 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.i64(*self);
    }
}

impl Data for u128 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u128(*self);
    }
}

impl Data for i128 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.i128(*self);
    }
}

impl Data for f32 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.f32(*self);
    }
}

impl Data for usize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        match usize::BITS {
            s if s == u8::BITS => value.u8(*self as u8),
            s if s == u16::BITS => value.u16(*self as u16),
            s if s == u32::BITS => value.u32(*self as u32),
            s if s == u64::BITS => value.u64(*self as u64),
            _ => {}
        }
    }
}

impl Data for isize {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        match isize::BITS {
            s if s == i8::BITS => value.i8(*self as i8),
            s if s == i16::BITS => value.i16(*self as i16),
            s if s == i32::BITS => value.i32(*self as i32),
            s if s == i64::BITS => value.i64(*self as i64),
            _ => {}
        }
    }
}

impl Data for f64 {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.f64(*self);
    }
}

impl Data for str {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.str(Cow::Borrowed(self));
    }
}

mod num {
    use super::*;
    use ::std::num;

    impl Data for num::NonZeroU8 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.u8(self.get());
        }
    }

    impl Data for num::NonZeroI8 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.i8(self.get());
        }
    }

    impl Data for num::NonZeroU16 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.u16(self.get());
        }
    }

    impl Data for num::NonZeroI16 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.i16(self.get());
        }
    }

    impl Data for num::NonZeroU32 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.u32(self.get());
        }
    }

    impl Data for num::NonZeroI32 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.i32(self.get());
        }
    }

    impl Data for num::NonZeroU64 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.u64(self.get());
        }
    }

    impl Data for num::NonZeroI64 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.i64(self.get());
        }
    }

    impl Data for num::NonZeroU128 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.u128(self.get());
        }
    }

    impl Data for num::NonZeroI128 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.i128(self.get());
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data> Data for Option<D> {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        if let Some(d) = self {
            d.provide_value(value);
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
}

#[warn(clippy::missing_trait_methods)]
impl<D: Data + ?Sized> Data for std::borrow::Cow<'_, D>
where
    D: ToOwned,
    D::Owned: Data,
{
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        match self {
            Self::Borrowed(data) => data.provide_value(builder),
            Self::Owned(data) => data.provide_value(builder),
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
impl<D: Data> Data for core::cell::OnceCell<D> {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        if let Some(d) = self.get() {
            d.provide_value(builder);
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
        if let Some(d) = self.get() {
            d.get_id()
        } else {
            None
        }
    }
}

impl<D: Data> Data for std::sync::OnceLock<D> {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        if let Some(d) = self.get() {
            d.provide_value(builder);
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
        if let Some(d) = self.get() {
            d.get_id()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn once_cell() {
        let cell = core::cell::OnceCell::new();

        let val = Value::from_data(&cell);
        assert!(val.as_enum().flatten().is_none());

        cell.get_or_init(|| 100);
        let val = Value::from_data(&cell);

        assert_eq!(val.as_i32().unwrap(), 100);
    }
}

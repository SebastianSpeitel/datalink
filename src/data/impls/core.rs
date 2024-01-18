use crate::data::Data;
use crate::links::{LinkError, Links};
use crate::value::ValueBuiler;

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
        match self {
            Some(data) => data.provide_value(value),
            None => {}
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Some(data) => data.provide_links(links),
            None => Ok(()),
        }
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        match self {
            Some(data) => data.query_links(links, query),
            None => Ok(()),
        }
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        match self {
            Some(d) => d.get_id(),
            None => None,
        }
    }
}

use crate::data::{BoxedData, Data, Primitive};
use crate::id::ID;
use crate::link_builder::{LinkBuilder, LinkBuilderError as LBE};
use crate::value::ValueBuiler;

mod core;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "std")]
mod std;
#[cfg(feature = "toml")]
mod toml;

#[warn(clippy::missing_trait_methods)]
impl Data for &dyn Data {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        (**self).provide_value(builder)
    }
    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) -> Result<(), LBE> {
        (**self).provide_links(builder)
    }
    #[inline]
    fn query_links(
        &self,
        builder: &mut dyn LinkBuilder,
        query: &crate::query::Query,
    ) -> Result<(), LBE> {
        (**self).query_links(builder, query)
    }
    #[inline]
    #[cfg(feature = "unique")]
    fn get_id(&self) -> Option<ID> {
        (**self).get_id()
    }
}
impl Primitive for &dyn Data {}

#[warn(clippy::missing_trait_methods)]
impl Data for BoxedData {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        (**self).provide_value(builder)
    }
    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) -> Result<(), LBE> {
        (**self).provide_links(builder)
    }
    #[inline]
    fn query_links(
        &self,
        builder: &mut dyn LinkBuilder,
        query: &crate::query::Query,
    ) -> Result<(), LBE> {
        (**self).query_links(builder, query)
    }
    #[inline]
    #[cfg(feature = "unique")]
    fn get_id(&self) -> Option<ID> {
        (**self).get_id()
    }
}
impl Primitive for BoxedData {}

// #[warn(clippy::missing_trait_methods)]
// impl<D: Data + ?Sized, F> Data for F
// where
//     D: 'static,
//     F: Fn() -> &'static D,
// {
//     #[inline]
//     fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
//         self().provide_value(builder)
//     }
//     #[inline]
//     fn provide_links(&self, builder: &mut dyn LinkBuilder) {
//         self().provide_links(builder)
//     }
//     #[cfg(feature = "unique")]
//     #[inline]
//     fn id(&self) -> Option<ID> {
//         self().id()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataExt;

    #[test]
    fn dyn_data() {
        let i = 100;
        let b = true;

        let int_data = &i as &dyn Data;
        let bool_data: &dyn Data = &b;

        assert_eq!(DataExt::as_i32(&int_data), Some(100));
        assert_eq!(DataExt::as_i32(&bool_data), None);

        assert_eq!(DataExt::as_bool(&int_data), None);
        assert_eq!(DataExt::as_bool(&bool_data), Some(true));
    }
}

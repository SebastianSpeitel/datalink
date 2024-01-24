use crate::data::Data;
use crate::id::ID;
use crate::links::{LinkError, Links};
use crate::value::ValueBuiler;

mod core;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "std")]
mod std;
#[cfg(feature = "toml")]
mod toml;

#[warn(clippy::missing_trait_methods)]
impl<D: Data + ?Sized> Data for Box<D> {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        (**self).provide_value(builder)
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        (**self).provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        (**self).query_links(links, query)
    }
    #[inline]
    fn get_id(&self) -> Option<ID> {
        (**self).get_id()
    }
}

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

use crate::Data;

use super::Provided;

mod core;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "std")]
mod std;
#[cfg(feature = "toml")]
mod toml;

#[macro_export]
macro_rules! impl_deref {
    ($ty:ty) => {
        impl<D: $crate::data::Data> $crate::data::Data for $ty {
            #[inline]
            fn provide_value(&self, request: $crate::rr::Request) {
                (**self).provide_value(request)
            }
            #[inline]
            fn provide_requested<'d, R: $crate::rr::Req>(
                &self,
                request: &mut $crate::rr::Request<'d, R>,
            ) -> impl $crate::data::Provided {
                (**self).provide_requested(request)
            }
            #[inline]
            fn provide_links(
                &self,
                links: &mut dyn $crate::links::Links,
            ) -> $crate::links::Result<()> {
                (**self).provide_links(links)
            }
            #[inline]
            fn query_links(
                &self,
                links: &mut dyn $crate::links::Links,
                query: &$crate::query::Query,
            ) -> $crate::links::Result<()> {
                (**self).query_links(links, query)
            }
            #[inline]
            fn get_id(&self) -> Option<$crate::id::ID> {
                (**self).get_id()
            }
        }
        #[cfg(feature = "unique")]
        impl<D: $crate::data::unique::Unique + ?Sized> $crate::data::unique::Unique for $ty {
            #[inline]
            fn id(&self) -> $crate::id::ID {
                (**self).id()
            }
        }
    };
}

impl_deref!(&D);
impl_deref!(Box<D>);
impl_deref!(::std::sync::Arc<D>);
impl_deref!(::std::rc::Rc<D>);
impl_deref!(::std::sync::MutexGuard<'_, D>);
impl_deref!(::std::sync::RwLockReadGuard<'_, D>);
impl_deref!(::std::sync::RwLockWriteGuard<'_, D>);
impl_deref!(::std::cell::Ref<'_, D>);
impl_deref!(::std::cell::RefMut<'_, D>);

#[warn(clippy::missing_trait_methods)]
impl Data for Box<dyn Data> {
    #[inline]
    fn provide_value(&self, request: crate::rr::Request) {
        (**self).provide_value(request);
    }
    #[inline]
    fn provide_requested<'d, R: crate::rr::Req>(
        &self,
        _request: &mut crate::rr::Request<'d, R>,
    ) -> impl Provided {
        super::internal::DefaultImpl
    }
    #[inline]
    fn provide_links(
        &self,
        links: &mut dyn crate::links::Links,
    ) -> Result<(), crate::links::LinkError> {
        (**self).provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn crate::links::Links,
        query: &crate::query::Query,
    ) -> Result<(), crate::links::LinkError> {
        (**self).query_links(links, query)
    }
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        (**self).get_id()
    }
}

#[warn(clippy::missing_trait_methods)]
impl Data for &dyn Data {
    #[inline]
    fn provide_value(&self, request: crate::rr::Request) {
        (**self).provide_value(request);
    }
    #[inline]
    fn provide_requested<'d, R: crate::rr::Req>(
        &self,
        _request: &mut crate::rr::Request<'d, R>,
    ) -> impl Provided {
        super::internal::DefaultImpl
    }
    #[inline]
    fn provide_links(
        &self,
        links: &mut dyn crate::links::Links,
    ) -> Result<(), crate::links::LinkError> {
        (**self).provide_links(links)
    }
    #[inline]
    fn query_links(
        &self,
        links: &mut dyn crate::links::Links,
        query: &crate::query::Query,
    ) -> Result<(), crate::links::LinkError> {
        (**self).query_links(links, query)
    }
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        (**self).get_id()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Data, DataExt};

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

use crate::Data;

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
            fn provide_requested<R: $crate::rr::Req>(
                &self,
                request: &mut $crate::rr::Request<R>,
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

macro_rules! impl_dyn {
    ($ty:ty) => {
        impl Data for $ty {
            #[inline]
            fn provide_value(&self, request: crate::rr::Request) {
                (**self).provide_value(request);
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
    };
}

impl_dyn!(&dyn Data);
impl_dyn!(&mut dyn Data);
impl_dyn!(Box<dyn Data>);
impl_dyn!(::std::sync::Arc<dyn Data>);
impl_dyn!(::std::rc::Rc<dyn Data>);
impl_dyn!(::std::sync::MutexGuard<'_, dyn Data>);
impl_dyn!(::std::sync::RwLockReadGuard<'_, dyn Data>);
impl_dyn!(::std::sync::RwLockWriteGuard<'_, dyn Data>);
impl_dyn!(::std::cell::Ref<'_, dyn Data>);
impl_dyn!(::std::cell::RefMut<'_, dyn Data>);

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

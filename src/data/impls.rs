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
        impl<R: $crate::rr::Req, D: $crate::data::Data + ?Sized> $crate::data::Data<R> for $ty {
            #[inline]
            fn provide_value<'d>(&self, mut request: $crate::rr::Request<'d, R>) {
                if let Some(opt) = $crate::data::Data::<$crate::rr::Unknown>::with_req::<R>(self) {
                    opt.provide_value(request);
                    return;
                }
                let request =
                    $crate::rr::Request::new(&mut request.0 as &mut dyn $crate::rr::Receiver);
                (**self).provide_value(request)
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
                debug_assert!($crate::data::Data::<$crate::rr::Unknown>::get_id(self)
                    .is_some_and(|id| id == (*self).id()));
                (**self).id()
            }
        }
    };
}

impl_deref!(Box<D>);
impl_deref!(::std::sync::Arc<D>);
impl_deref!(::std::rc::Rc<D>);
impl_deref!(::std::sync::MutexGuard<'_, D>);
impl_deref!(::std::sync::RwLockReadGuard<'_, D>);
impl_deref!(::std::sync::RwLockWriteGuard<'_, D>);
impl_deref!(::std::cell::Ref<'_, D>);
impl_deref!(::std::cell::RefMut<'_, D>);

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

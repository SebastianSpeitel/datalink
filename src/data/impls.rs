mod core;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "smol_str")]
mod smol_str;
#[cfg(feature = "std")]
mod std;
#[cfg(feature = "toml")]
mod toml;

// pub mod kernel;

#[macro_export]
macro_rules! impl_deref {
    ($ty:ty) => {
        impl<D: $crate::Data> $crate::Data for $ty {
            #[inline]
            fn query(&self, request: impl $crate::Request) {
                (**self).query(request);
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
impl_deref!(&mut D);
impl_deref!(Box<D>);
impl_deref!(::std::sync::Arc<D>);
impl_deref!(::std::rc::Rc<D>);
impl_deref!(::std::sync::MutexGuard<'_, D>);
impl_deref!(::std::sync::RwLockReadGuard<'_, D>);
impl_deref!(::std::sync::RwLockWriteGuard<'_, D>);
impl_deref!(::core::cell::Ref<'_, D>);
impl_deref!(::core::cell::RefMut<'_, D>);

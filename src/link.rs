use core::convert::Infallible;

use crate::{data::DataExt, Data, Query};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nothing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Optional<T, N = Nothing> {
    Some(T),
    None(N),
}

impl<T> Optional<T, Infallible> {
    #[inline]
    #[must_use]
    pub const fn always(key: T) -> Self {
        Self::Some(key)
    }
}

impl Optional<Infallible, Nothing> {
    #[inline]
    #[must_use]
    pub const fn never() -> Self {
        Self::None(Nothing)
    }
}

impl<T> From<Option<T>> for Optional<T, Nothing> {
    #[inline]
    fn from(option: Option<T>) -> Self {
        match option {
            Some(key) => Optional::Some(key),
            None => Optional::None(Nothing),
        }
    }
}

impl<T, N> From<Optional<T, N>> for Option<T> {
    #[inline]
    fn from(maybe: Optional<T, N>) -> Self {
        match maybe {
            Optional::Some(key) => Some(key),
            Optional::None(_) => None,
        }
    }
}

pub trait Link {
    fn query(&self, target_query: impl Query, key_query: impl Query);

    #[inline]
    fn query_owned(self, target_query: impl Query, key_query: impl Query)
    where
        Self: Sized,
    {
        self.query(target_query, key_query);
    }
}

impl<K, T> Link for (K, T)
where
    K: Data + ToOwned<Owned: Data + 'static>,
    T: Data + ToOwned<Owned: Data + 'static>,
{
    #[inline]
    fn query(&self, mut target_query: impl Query, mut key_query: impl Query) {
        self.0.query(&mut key_query);
        self.1.query(&mut target_query);
    }

    #[inline]
    fn query_owned(self, mut target_query: impl Query, mut key_query: impl Query) {
        self.0
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut key_query);
        self.1
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut target_query);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unkeyed<T>(pub T);

impl<T> Link for Unkeyed<T>
where
    T: Data + ToOwned<Owned: Data + 'static>,
{
    #[inline]
    fn query(&self, mut query: impl Query, _: impl Query) {
        self.0.query(&mut query);
    }

    #[inline]
    fn query_owned(self, mut query: impl Query, _: impl Query) {
        self.0
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut query);
    }
}

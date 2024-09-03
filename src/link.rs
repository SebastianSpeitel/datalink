use core::convert::Infallible;

use crate::{data::DataExt, Data, LinkQuery};

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
    type Key: Data;
    type Target: Data + ?Sized;

    fn query(&self, query: impl LinkQuery);

    #[inline]
    fn query_owned(self, query: impl LinkQuery)
    where
        Self: Sized,
    {
        self.query(query);
    }

    fn into_tuple(self) -> (Optional<Self::Key, impl Copy>, Self::Target);
}

impl<K, T> Link for (K, T)
where
    K: Data + ToOwned<Owned: Data + 'static>,
    T: Data + ToOwned<Owned: Data + 'static>,
{
    type Key = K;
    type Target = T;

    #[inline]
    fn query(&self, mut query: impl LinkQuery) {
        self.0.query(&mut query.key_query());
        self.1.query(&mut query.target_query());
    }

    #[inline]
    fn query_owned(self, mut query: impl LinkQuery) {
        self.0
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut query.key_query());
        self.1
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut query.target_query());
    }

    #[inline]
    fn into_tuple(self) -> (Optional<Self::Key, impl Copy>, Self::Target) {
        (Optional::always(self.0), self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unkeyed<T>(pub T);

impl<T> Link for Unkeyed<T>
where
    T: Data + ToOwned<Owned: Data + 'static>,
{
    type Key = core::convert::Infallible;
    type Target = T;

    #[inline]
    fn query(&self, mut query: impl LinkQuery) {
        self.0.query(&mut query.target_query());
    }

    #[inline]
    fn query_owned(self, mut query: impl LinkQuery) {
        self.0
            .to_owned()
            .ensure_erasablity()
            .query_owned(&mut query.target_query());
    }

    #[inline]
    fn into_tuple(self) -> (Optional<Self::Key, impl Copy>, Self::Target) {
        (Optional::never(), self.0)
    }
}

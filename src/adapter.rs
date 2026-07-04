use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Data, Request};

pub trait Adapter<T> {
    fn query(data: &T, request: impl Request);

    #[inline]
    fn query_owned(data: T, request: impl Request)
    where
        Self: Sized,
    {
        Self::query(&data, request);
    }

    #[inline]
    fn adapt(data: T) -> Adapted<T, Self>
    where
        Self: Default,
    {
        Adapted::new(data)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Adapted<T, A = ()> {
    data: T,
    adapter: PhantomData<A>,
}

impl<T, A> Adapted<T, A> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            data,
            adapter: PhantomData,
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T: Default, A: Default> Default for Adapted<T, A> {
    #[inline]
    fn default() -> Self {
        Self {
            data: T::default(),
            adapter: PhantomData,
        }
    }
}

impl<T, A: Default> From<T> for Adapted<T, A> {
    #[inline]
    fn from(data: T) -> Self {
        Self {
            data,
            adapter: PhantomData,
        }
    }
}

impl<T, A: Adapter<T>> Data for Adapted<T, A> {
    #[inline]
    fn query(&self, request: impl Request) {
        A::query(&self.data, request);
    }

    #[inline]
    fn query_owned(self, request: impl Request) {
        A::query_owned(self.data, request);
    }
}

impl<T, A> Deref for Adapted<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, A> DerefMut for Adapted<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, A> Borrow<T> for Adapted<T, A> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.data
    }
}
impl<T, A> BorrowMut<T> for Adapted<T, A> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T, A> AsRef<T> for Adapted<T, A> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T, A> AsMut<T> for Adapted<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Hash, A> Hash for Adapted<T, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<T: PartialEq, A> PartialEq for Adapted<T, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Eq, A> Eq for Adapted<T, A> {}

impl<T: PartialOrd, A> PartialOrd for Adapted<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T: Ord, A> Ord for Adapted<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T: Data> Adapter<T> for () {
    #[inline]
    fn query(data: &T, request: impl Request) {
        data.query(request);
    }
    #[inline]
    fn query_owned(data: T, request: impl Request) {
        data.query_owned(request);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DerefAdapter;

impl<T> Adapter<T> for DerefAdapter
where
    T: Deref<Target: Data>,
{
    #[inline]
    fn query(data: &T, request: impl Request) {
        (**data).query(request);
    }
    #[inline]
    fn query_owned(data: T, request: impl Request) {
        data.query(request);
    }
}

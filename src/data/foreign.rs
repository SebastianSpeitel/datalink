use crate::{Data, Request};

/// Function to use when querying a foreign data type.
///
/// # Example
/// ```rust
/// use datalink::{ForeignData, QueryFn, Request};
///
/// struct Foo;
///
/// #[derive(Default)]
/// struct QueryFoo;
/// impl QueryFn<Foo> for QueryFoo {
///     fn query(&self, data: &Foo, mut request: impl Request) {
///        request.provide_str("foo");
///    }
/// }
///
/// let wrapped_foo = Foo.wrap::<QueryFoo>();
/// ```
pub trait QueryFn<T>: Sized {
    fn query(&self, data: &T, request: impl Request);

    #[inline]
    fn query_owned(self, data: T, request: impl Request) {
        self.query(&data, request);
    }
}

pub trait ForeignData: Sized {
    #[inline]
    fn wrap_with<Q: QueryFn<Self>>(self, query_fn: Q) -> Foreign<Self, Q> {
        Foreign::new_with(self, query_fn)
    }

    #[inline]
    fn wrap<Q: QueryFn<Self> + Default>(self) -> Foreign<Self, Q> {
        Foreign::new_with(self, Q::default())
    }
}

impl<T> ForeignData for T {}

#[derive(Debug, Clone, Copy)]
pub struct Foreign<T, Q: QueryFn<T>> {
    query_fn: Q,
    data: T,
}

impl<T: core::hash::Hash, Q: QueryFn<T>> core::hash::Hash for Foreign<T, Q> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<T: PartialEq, Q: QueryFn<T>> PartialEq for Foreign<T, Q> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.data.ne(other)
    }
}

impl<T: Eq, Q: QueryFn<T>> Eq for Foreign<T, Q> {}

impl<T: PartialOrd, Q: QueryFn<T>> PartialOrd for Foreign<T, Q> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl<T: Ord, Q: QueryFn<T>> Ord for Foreign<T, Q> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T, Q: QueryFn<T>> Foreign<T, Q> {
    #[inline]
    pub const fn new_with(data: T, query_fn: Q) -> Self {
        Self { query_fn, data }
    }

    #[inline]
    pub fn new(data: T) -> Self
    where
        Q: Default,
    {
        Self::new_with(data, Q::default())
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T, Q: QueryFn<T> + Default> From<T> for Foreign<T, Q> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T, Q: QueryFn<T>> Data for Foreign<T, Q> {
    #[inline]
    fn query(&self, request: impl Request) {
        self.query_fn.query(&self.data, request);
    }

    #[inline]
    fn query_owned(self, request: impl Request) {
        self.query_fn.query_owned(self.data, request);
    }
}

impl<T, Q: QueryFn<T>> core::borrow::Borrow<T> for Foreign<T, Q> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.data
    }
}

impl<T, Q: QueryFn<T>> core::borrow::BorrowMut<T> for Foreign<T, Q> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T, Q: QueryFn<T>> AsRef<T> for Foreign<T, Q> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T, Q: QueryFn<T>> AsMut<T> for Foreign<T, Q> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T, Q: QueryFn<T>> core::ops::Deref for Foreign<T, Q> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, Q: QueryFn<T>> core::ops::DerefMut for Foreign<T, Q> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

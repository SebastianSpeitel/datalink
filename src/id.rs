use core::{
    fmt::{Debug, Display},
    num::NonZeroU128,
    str::FromStr,
};

/// ID used for uniquely identifying `Data`
///
/// By default uses `NonZeroU128` as the underlying type for the following reasons:
/// * Implements `Hash`, `PartialEq`, `Eq`, `PartialOrd`and `Ord` for using it as a key in a `HashMap`
/// * Implements `Copy` for ergonomics
/// * 128 bits to future-proof scenarios with huge amounts of data
/// * 128 bits is the size of a `UUID`
/// * Non-zero for niche optimizations
///
/// ```rust
/// use datalink::id::ID;
/// use std::mem::size_of;
/// use std::num::NonZeroU128;
///
/// // Niche optimization
/// assert_eq!(size_of::<ID>(), size_of::<Option<ID>>());
///
/// // Constant ID
/// const ID: ID = ID::from_raw(unsafe { NonZeroU128::new_unchecked(42) });
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct ID<T = NonZeroU128>(T);

impl<T> ID<T> {
    /// Create a new `ID` from any value convertible into `T`
    ///
    /// ```rust
    /// use datalink::id::ID;
    ///
    /// let id = ID::<i32>::new(42);
    /// ```
    #[inline]
    pub fn new<U: Into<T>>(id: U) -> Self {
        Self::from_raw(id.into())
    }

    /// Convenience method for creating an `ID` from a type that only has a `TryInto<T>` implementation
    ///
    /// ```rust
    /// use datalink::id::ID;
    ///
    /// // NonZeroU128 implements TryFrom<u128>
    /// let id: ID = ID::try_new(42).unwrap();
    /// ```
    #[inline]
    pub fn try_new<U: TryInto<T>>(id: U) -> Result<Self, U::Error> {
        id.try_into().map(Self::from_raw)
    }

    /// Create a new `ID` from a raw `T` value
    ///
    /// # Note
    /// Can be used in const contexts
    ///
    /// ```rust
    /// use datalink::id::ID;
    /// use std::num::NonZeroU128;
    ///
    /// const ID: ID = ID::from_raw(unsafe { NonZeroU128::new_unchecked(42) });
    /// ```
    #[inline]
    pub const fn from_raw(id: T) -> Self {
        Self(id)
    }

    /// Get the raw `T` value
    #[inline]
    pub fn into_raw(self) -> T {
        self.0
    }

    /// Get a reference to the raw `T` value
    #[inline]
    pub const fn as_raw(&self) -> &T {
        &self.0
    }
}

impl ID<NonZeroU128> {
    /// Create a new `ID` from a raw `u128` value
    ///
    /// # Safety
    /// The value must not be zero
    ///
    /// # Note
    /// Can be used in const contexts
    ///
    /// ```rust
    /// use datalink::id::ID;
    ///
    /// const id: ID = unsafe { ID::new_unchecked(42) };
    /// assert_eq!(id.into_raw().get(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(id: u128) -> Self {
        unsafe { Self::from_raw(NonZeroU128::new_unchecked(id)) }
    }
}

impl<T: FromStr> FromStr for ID<T> {
    type Err = T::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(Self::from_raw)
    }
}

impl<T: Display> Display for ID<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for ID<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("ID").field(&self.0).finish()
    }
}

#[cfg(feature = "random")]
impl rand::distr::Distribution<ID> for rand::distr::StandardUniform {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ID {
        loop {
            use rand::RngExt;

            let id = rng.random();
            if let Some(id) = NonZeroU128::new(id) {
                break ID(id);
            }
        }
    }
}

impl<T> From<T> for ID<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::from_raw(value)
    }
}

impl<T: Copy + 'static> crate::Visitor for Option<ID<T>> {
    #[inline]
    fn visit_any(&mut self, value: &dyn core::any::Any) {
        if self.is_some() {
            return;
        }
        *self = value.downcast_ref().copied();
    }
    #[inline]
    fn visit_any_owned(&mut self, value: Box<dyn core::any::Any>) {
        if self.is_some() {
            return;
        }
        if let Ok(id) = value.downcast::<ID<T>>() {
            *self = Some(*id);
        }
    }
    // todo: impl other
}

impl<T: Copy + 'static> crate::request::ErasableRequest for Option<ID<T>> {
    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        // todo: more specific
        crate::schema::OnlyValues::SCHEMA
    }
    #[inline]
    fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
        Some(self)
    }
}

impl<T: Copy + 'static> crate::Request for Option<ID<T>> {
    #[inline]
    fn schema(&self) -> impl crate::schema::Schema {
        crate::r#type::Types::<(ID<T>,)>::new()
    }
    #[inline]
    fn visitor(&mut self) -> impl crate::request::Visitor {
        self
    }
    #[inline]
    fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
        self
    }
}

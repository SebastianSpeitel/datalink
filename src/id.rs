use std::{
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

    /// Convenience method for creating an `ID` from a type that only has a `TryInto<T>` implementation
    ///
    /// # Safety
    /// The caller must ensure that `U::try_into` never fails
    ///
    /// ```rust
    /// use datalink::id::ID;
    ///
    /// // NonZeroU128 implements TryFrom<u128>
    /// let id: ID = unsafe { ID::new_unchecked(42) };
    /// ```
    #[inline]
    pub unsafe fn new_unchecked<U: TryInto<T>>(id: U) -> Self {
        Self::from_raw(id.try_into().unwrap_unchecked())
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for ID<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ID").field(&self.0).finish()
    }
}

#[cfg(feature = "random")]
impl rand::distributions::Distribution<ID> for rand::distributions::Standard {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ID {
        loop {
            let id = rng.gen();
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

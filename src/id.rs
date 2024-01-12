use std::{
    fmt::{Debug, Display},
    num::NonZeroU128,
    str::FromStr,
};

/// ID used for uniquely identifying Data
///
/// By default uses `NonZeroU128` as the underlying type for the following reasons:
/// * Implements `Hash`, `PartialEq`, `Eq`, `PartialOrd`and `Ord` for using it as a key in a `HashMap`
/// * Implements `Copy` for ergonomics
/// * 128 bits to future-proof scenarios with huge amounts of data
/// * 128 bits is the size of a `UUID`
/// * Non-zero for niche optimizations
/// ```rust
/// use datalink::id::ID;
/// use std::mem::size_of;
///
/// // Niche optimization
/// assert_eq!(size_of::<ID>(), size_of::<Option<ID>>());
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID<T = NonZeroU128>(T);

impl<T> ID<T> {
    #[inline]
    pub const fn new(id: T) -> Self {
        Self(id)
    }
}

impl<T: FromStr> FromStr for ID<T> {
    type Err = T::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(Self)
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
        Self::new(value)
    }
}

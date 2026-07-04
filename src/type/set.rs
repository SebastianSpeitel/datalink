use super::{SimpleTypeSet, Type};

pub trait TypeSet: Clone {
    fn contains(&self, typ: impl Type) -> bool;
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        [] as [core::convert::Infallible; 0]
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        SimpleTypeSet::from_trait(self)
    }
}

impl TypeSet for () {
    #[inline]
    fn contains(&self, _typ: impl Type) -> bool {
        false
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        SimpleTypeSet::empty()
    }
}

impl<T: Type> TypeSet for Option<T> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        if let Some(t) = self { t.eq(typ) } else { false }
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
}

impl<T: Type> TypeSet for &[T] {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self.iter().copied()
    }
}

impl<T: Type, const N: usize> TypeSet for [T; N] {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
}

#[cfg(feature = "std")]
impl<T: Type> TypeSet for Vec<T> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
}

#[cfg(feature = "std")]
impl<T: Type, S: core::hash::BuildHasher + Clone> TypeSet for std::collections::HashSet<T, S> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
}

#[cfg(feature = "std")]
impl<T: Type> TypeSet for std::collections::BTreeSet<T> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
}

#[cfg(feature = "std")]
impl<T: Type> TypeSet for std::borrow::Cow<'_, [T]> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.iter().any(|t| t.eq(typ))
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self.into_owned()
    }
}

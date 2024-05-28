use core::any::TypeId;
use core::convert::Infallible;
use core::marker::PhantomData;

pub trait TypeSet {
    type Error;

    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error>;

    #[inline]
    fn contains_type_checked<T: 'static + ?Sized>(&self) -> Result<bool, Self::Error>
    where
        Self: Sized,
    {
        self.contains_id_checked(TypeId::of::<T>())
    }
    #[allow(unused_variables)]
    #[inline]
    fn contains_type_of_checked<T: 'static + ?Sized>(&self, value: &T) -> Result<bool, Self::Error>
    where
        Self: Sized,
    {
        self.contains_type_checked::<T>()
    }

    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool
    where
        Self: TypeSet<Error = Infallible> + Sized,
    {
        match self.contains_id_checked(type_id) {
            Ok(result) => result,
            Err(never) => match never {},
        }
    }
    #[inline]
    fn contains_type<T: 'static + ?Sized>(&self) -> bool
    where
        Self: TypeSet<Error = Infallible> + Sized,
    {
        match self.contains_type_checked::<T>() {
            Ok(result) => result,
            Err(never) => match never {},
        }
    }
    #[allow(unused_variables)]
    #[inline]
    fn contains_type_of<T: 'static + ?Sized>(&self, value: &T) -> bool
    where
        Self: TypeSet<Error = Infallible> + Sized,
    {
        match self.contains_type_of_checked::<T>(value) {
            Ok(result) => result,
            Err(never) => match never {},
        }
    }
}

// impl<E> TypeSet for &dyn TypeSet<Error = E> {
//     type Error = E;
//     #[inline]
//     fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
//         (**self).contains_id_checked(type_id)
//     }
// }

// impl<E> TypeSet for Box<dyn TypeSet<Error = E>> {
//     type Error = E;
//     #[inline]
//     fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
//         (**self).contains_id_checked(type_id)
//     }
// }

#[derive(Debug)]
pub struct Only<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Default for Only<T> {
    #[inline]
    fn default() -> Self {
        Only(PhantomData)
    }
}

impl<T> TypeSet for Only<T>
where
    T: 'static + ?Sized,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(type_id == TypeId::of::<T>())
    }
}

// impl<const N: usize> TypeSet for [TypeId; N] {
//     type Error = Infallible;
//     #[inline]
//     fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
//         Ok(self.contains(&type_id))
//     }
// }

// impl TypeSet for Vec<TypeId> {
//     type Error = Infallible;
//     #[inline]
//     fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
//         Ok(self.contains(&type_id))
//     }
// }

// impl TypeSet for [TypeId] {
//     type Error = Infallible;
//     #[inline]
//     fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
//         Ok(self.contains(&type_id))
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct All;

impl TypeSet for All {
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, _type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnyOf<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Default for AnyOf<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T1> TypeSet for AnyOf<(T1,)>
where
    T1: 'static + ?Sized,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(type_id == TypeId::of::<T1>())
    }
}

impl<T1, T2> TypeSet for AnyOf<(T1, T2)>
where
    T1: 'static,
    T2: 'static,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(type_id == TypeId::of::<T1>() || type_id == TypeId::of::<T2>())
    }
}

impl<T1, T2, T3, T4, T5, T6> TypeSet for AnyOf<(T1, T2, T3, T4, T5, T6)>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>())
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TypeSet
    for AnyOf<(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    )>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
    T11: 'static,
    T12: 'static,
    T13: 'static,
    T14: 'static,
    T15: 'static,
    T16: 'static,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
            || type_id == TypeId::of::<T7>()
            || type_id == TypeId::of::<T8>()
            || type_id == TypeId::of::<T9>()
            || type_id == TypeId::of::<T10>()
            || type_id == TypeId::of::<T11>()
            || type_id == TypeId::of::<T12>()
            || type_id == TypeId::of::<T13>()
            || type_id == TypeId::of::<T14>()
            || type_id == TypeId::of::<T15>()
            || type_id == TypeId::of::<T16>())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct And<T: TypeSet, U: TypeSet>(pub T, pub U);

impl<T, U> TypeSet for And<T, U>
where
    T: TypeSet<Error = Infallible>,
    U: TypeSet<Error = Infallible>,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(self.0.contains_id(type_id) && self.1.contains_id(type_id))
    }
    #[inline]
    fn contains_type_checked<Typ: 'static + ?Sized>(&self) -> Result<bool, Self::Error> {
        Ok(self.0.contains_type::<Typ>() && self.1.contains_type::<Typ>())
    }
    #[inline]
    fn contains_type_of_checked<Typ: 'static + ?Sized>(
        &self,
        val: &Typ,
    ) -> Result<bool, Self::Error> {
        Ok(self.0.contains_type_of(val) && self.1.contains_type_of(val))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Or<T: TypeSet, U: TypeSet>(pub T, pub U);

impl<T, U> TypeSet for Or<T, U>
where
    T: TypeSet<Error = Infallible>,
    U: TypeSet<Error = Infallible>,
{
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(self.0.contains_id(type_id) || self.1.contains_id(type_id))
    }
    #[inline]
    fn contains_type_checked<Typ: 'static + ?Sized>(&self) -> Result<bool, Self::Error> {
        Ok(self.0.contains_type::<Typ>() || self.1.contains_type::<Typ>())
    }
    #[inline]
    fn contains_type_of_checked<Typ: 'static + ?Sized>(
        &self,
        val: &Typ,
    ) -> Result<bool, Self::Error> {
        Ok(self.0.contains_type_of(val) || self.1.contains_type_of(val))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Not<T: TypeSet>(pub T);

impl<T: TypeSet> TypeSet for Not<T> {
    type Error = T::Error;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        self.0.contains_id_checked(type_id).map(|result| !result)
    }
    #[inline]
    fn contains_type_checked<Typ: 'static + ?Sized>(&self) -> Result<bool, Self::Error> {
        self.0.contains_type_checked::<Typ>().map(|result| !result)
    }
    #[inline]
    fn contains_type_of_checked<Typ: 'static + ?Sized>(
        &self,
        val: &Typ,
    ) -> Result<bool, Self::Error> {
        self.0.contains_type_of_checked(val).map(|result| !result)
    }
}

pub type StringLike = AnyOf<(String, &'static str)>;

pub type BytesLike = AnyOf<(Vec<u8>, &'static [u8])>;

#[derive(Debug)]
pub struct AcceptedBy<R>(PhantomData<R>);

impl<R> AcceptedBy<R> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for AcceptedBy<R> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<R: super::Receiver> TypeSet for AcceptedBy<R> {
    type Error = Infallible;
    #[inline]
    fn contains_id_checked(&self, type_id: TypeId) -> Result<bool, Self::Error> {
        Ok(R::accepting()
            .contains_id_checked(type_id)
            .unwrap_or_default())
    }
    #[inline]
    fn contains_type_checked<T: 'static + ?Sized>(&self) -> Result<bool, Self::Error> {
        Ok(R::accepting()
            .contains_type_checked::<T>()
            .unwrap_or_default())
    }
    #[inline]
    fn contains_type_of_checked<T: 'static + ?Sized>(
        &self,
        value: &T,
    ) -> Result<bool, Self::Error> {
        Ok(R::accepting()
            .contains_type_of_checked(value)
            .unwrap_or_default())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Invalid<E>(E);

impl<E: Clone> TypeSet for Invalid<E> {
    type Error = E;
    #[inline]
    fn contains_id_checked(&self, _type_id: TypeId) -> Result<bool, Self::Error> {
        Err(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only() {
        let set = Only::<u8>::default();
        assert!(set.contains_type::<u8>());
        assert!(!set.contains_type::<u16>());
    }

    #[test]
    fn infallible_result_size() {
        assert_eq!(
            core::mem::size_of::<Result<bool, Infallible>>(),
            core::mem::size_of::<bool>()
        );
    }

    #[test]
    fn stringlike() {
        let set = StringLike::default();
        assert!(set.contains_type::<String>());
        assert!(set.contains_type::<&str>());
        assert!(!set.contains_type::<u8>());
    }

    #[test]
    fn byteslike() {
        let set = BytesLike::default();
        assert!(set.contains_type::<Vec<u8>>());
        assert!(set.contains_type::<&[u8]>());
        assert!(!set.contains_type::<u8>());
    }
}

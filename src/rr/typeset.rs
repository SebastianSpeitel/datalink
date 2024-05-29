use core::any::TypeId;
use core::marker::PhantomData;

pub trait TypeSet {
    fn contains_id(&self, type_id: TypeId) -> bool;
    #[inline]
    fn contains_type<T: 'static + ?Sized>(&self) -> bool
    where
        Self: Sized,
    {
        self.contains_id(TypeId::of::<T>())
    }
    #[allow(unused_variables)]
    #[inline]
    fn contains_type_of<T: 'static + ?Sized>(&self, value: &T) -> bool
    where
        Self: Sized,
    {
        self.contains_type::<T>()
    }
}

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
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T>()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct All;

impl TypeSet for All {
    #[inline]
    fn contains_id(&self, _type_id: TypeId) -> bool {
        true
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
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
    }
}

impl<T1, T2> TypeSet for AnyOf<(T1, T2)>
where
    T1: 'static,
    T2: 'static,
{
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>() || type_id == TypeId::of::<T2>()
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
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
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
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
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
            || type_id == TypeId::of::<T16>()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct And<T: TypeSet, U: TypeSet>(pub T, pub U);

impl<T, U> TypeSet for And<T, U>
where
    T: TypeSet,
    U: TypeSet,
{
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        self.0.contains_id(type_id) && self.1.contains_id(type_id)
    }
    #[inline]
    fn contains_type<Typ: 'static + ?Sized>(&self) -> bool {
        self.0.contains_type::<Typ>() && self.1.contains_type::<Typ>()
    }
    #[inline]
    fn contains_type_of<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        self.0.contains_type_of(val) && self.1.contains_type_of(val)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Or<T: TypeSet, U: TypeSet>(pub T, pub U);

impl<T, U> TypeSet for Or<T, U>
where
    T: TypeSet,
    U: TypeSet,
{
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        self.0.contains_id(type_id) || self.1.contains_id(type_id)
    }
    #[inline]
    fn contains_type<Typ: 'static + ?Sized>(&self) -> bool {
        self.0.contains_type::<Typ>() || self.1.contains_type::<Typ>()
    }
    #[inline]
    fn contains_type_of<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        self.0.contains_type_of(val) || self.1.contains_type_of(val)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Not<T: TypeSet>(pub T);

impl<T: TypeSet> TypeSet for Not<T> {
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        !self.0.contains_id(type_id)
    }
    #[inline]
    fn contains_type<Typ: 'static + ?Sized>(&self) -> bool {
        !self.0.contains_type::<Typ>()
    }
    #[inline]
    fn contains_type_of<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        !self.0.contains_type_of(val)
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
    #[inline]
    fn contains_id(&self, type_id: TypeId) -> bool {
        R::accepting().contains_id(type_id)
    }
    #[inline]
    fn contains_type<T: 'static + ?Sized>(&self) -> bool {
        R::accepting().contains_type::<T>()
    }
    #[inline]
    fn contains_type_of<T: 'static + ?Sized>(&self, value: &T) -> bool {
        R::accepting().contains_type_of(value)
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

use core::any::TypeId;
use core::borrow::{Borrow, BorrowMut};
use core::convert::Infallible;
use core::marker::PhantomData;

use marker::TypeIdExt;

use crate::schema::prelude::*;

type DefaultContainer = [TypeId; 3];
pub type ErasedTypeSet = TypeArray<DefaultContainer>;

pub trait IterError {
    fn is_empty(&self) -> bool;
    fn is_any(&self) -> bool;
}

impl IterError for bool {
    #[inline]
    fn is_empty(&self) -> bool {
        !*self
    }
    #[inline]
    fn is_any(&self) -> bool {
        *self
    }
}

impl<T, F> IterError for crate::util::Bool<T, F> {
    #[inline]
    fn is_empty(&self) -> bool {
        self == false
    }
    #[inline]
    fn is_any(&self) -> bool {
        self == true
    }
}

impl IterError for Infallible {
    #[inline]
    fn is_empty(&self) -> bool {
        false
    }
    #[inline]
    fn is_any(&self) -> bool {
        false
    }
}

// pub trait TypeSet {
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError>;

//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         match self.try_iter() {
//             Ok(ids) => ids.into_iter().any(|i| i == id),
//             Err(e) => e.is_any(),
//         }
//     }

//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         self.contains_id(TypeId::of::<T>())
//     }

//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         |id| self.contains_id(*id)
//     }

//     #[inline]
//     fn is_empty(&self) -> bool {
//         match self.try_iter() {
//             Ok(ids) => ids.into_iter().size_hint().1 == Some(0),
//             Err(e) => e.is_empty(),
//         }
//     }

//     #[inline]
//     fn or<S: TypeSet>(self, other: S) -> impl TypeSet
//     where
//         Self: Sized,
//     {
//         crate::util::Union(self, other)
//     }

//     #[inline]
//     fn and<S: TypeSet>(self, other: S) -> impl TypeSet
//     where
//         Self: Sized,
//     {
//         crate::util::Intersection(self, other)
//     }
// }

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Any;

// impl TypeSet for Any {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         Err::<crate::util::Never, _>(crate::util::TRUE)
//     }

//     #[inline]
//     fn contains_id(&self, _id: TypeId) -> bool {
//         true
//     }
//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         true
//     }
//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         |_| true
//     }
// }

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct None;

// impl<T: 'static + ?Sized> TypeSet for super::TypeOf<T> {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         Ok::<_, Infallible>(iter::once_with(TypeId::of::<T>))
//     }

//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         id == TypeId::of::<T>()
//     }

//     #[inline]
//     fn is_empty(&self) -> bool {
//         false
//     }

//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         |id| *id == TypeId::of::<T>()
//     }
// }

#[allow(clippy::inline_always)]
mod marker {
    use super::{Infallible, TypeId};

    enum Any {}
    enum Empty {}
    enum Sorted {}

    #[inline(always)]
    pub(super) fn any() -> TypeId {
        TypeId::of::<Any>()
    }

    #[inline(always)]
    pub(super) fn empty() -> TypeId {
        TypeId::of::<Empty>()
    }

    #[inline(always)]
    pub(super) fn sorted() -> TypeId {
        TypeId::of::<Sorted>()
    }

    #[inline(always)]
    pub(super) fn never() -> TypeId {
        TypeId::of::<Infallible>()
    }

    pub(super) trait TypeIdExt {
        fn is_marker(&self) -> bool;
    }

    impl TypeIdExt for TypeId {
        #[inline(always)]
        fn is_marker(&self) -> bool {
            *self == any() || *self == empty() || *self == sorted()
        }
    }
}

#[derive(Clone, Copy)]
pub struct TypeArray<C: Borrow<[TypeId]> = DefaultContainer> {
    marker: TypeId,
    ids: C,
}

impl<C: BorrowMut<[TypeId]>> PartialEq<bool> for &TypeArray<C> {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        match self.marker {
            m if m == marker::any() => true,
            m if m == marker::empty() => false,
            _ => *other,
        }
    }
}

// impl<C: BorrowMut<[TypeId]>> Snek<TypeMarker> for TypeArray<C> {
//     type Head = TypeId;
//     type Tail = Self;

//     #[inline]
//     fn split(mut self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         match self.marker {
//             m if m == marker::any() => return (Err(()), self),
//             m if m == marker::empty() => return (Err(()), self),
//             m if m == marker::never() => return (Err(()), self),
//             _ => {}
//         }

//         let ids = self.ids.borrow_mut();
//         for id in ids {
//             if *id == marker::never() {
//                 continue;
//             }
//             let head = core::mem::replace(id, marker::never());
//             return (Ok(head), self);
//         }

//         let head = core::mem::replace(&mut self.marker, marker::never());

//         if head == marker::never() {
//             return (Err(()), self);
//         }

//         if head.is_marker() {
//             return (Err(()), self);
//         }

//         (Ok(head), self)
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         match self.marker {
//             m if m == marker::any() => 0,
//             m if m == marker::empty() => 0,
//             m if m == marker::sorted() => self.ids.borrow().len(),
//             _ => self.ids.borrow().len() + 1, // +1 for the marker
//         }
//     }
// }

// impl<C: BorrowMut<[TypeId]> + Clone> TypeValidator for TypeArray<C> {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         self.contains_id(&TypeId::of::<T>())
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         use super::Maybe;
//         let Some(id) = typ.id().get() else {
//             return false;
//         };
//         self.contains_id(&id)
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::union(self)
//     }
// }

// impl<C: BorrowMut<[TypeId]>> Decomposable for TypeArray<C> {
//     type Intersection = ();
//     type Not = Infallible;
//     type Simple = Infallible;
//     type Union = Self;
//     #[inline]
//     fn decompose(
//         self,
//     ) -> crate::schema::Composed<Self::Intersection, Self::Union, Self::Not, Self::Simple> {
//         crate::schema::Composed::Union(self)
//     }
// }

impl<C: Borrow<[TypeId]>> core::fmt::Debug for TypeArray<C> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut set = f.debug_set();

        if self.marker == marker::any() {
            return set.entry(&"[Any]").finish_non_exhaustive();
        }

        if self.marker == marker::empty() {
            return set.finish();
        }

        if self.marker == marker::sorted() {
            set.entry(&"[Sorted]");
        } else {
            set.entry(&self.marker);
        }

        for id in self.ids.borrow() {
            if *id == marker::never() {
                continue;
            }
            set.entry(id);
        }

        set.finish()
    }
}

impl<C: Borrow<[TypeId]>> TypeArray<C> {
    #[inline]
    #[must_use]
    pub fn from_type<T: 'static + ?Sized>() -> Self
    where
        Self: Default,
    {
        let id = TypeId::of::<T>();
        Self::from_id(id)
    }

    #[inline]
    #[must_use]
    pub fn from_id(id: TypeId) -> Self
    where
        Self: Default,
    {
        debug_assert!(!id.is_marker());
        Self {
            marker: id,
            ..Default::default()
        }
    }

    #[inline]
    #[must_use]
    pub fn empty() -> Self
    where
        Self: Default,
    {
        Self {
            marker: marker::empty(),
            ..Default::default()
        }
    }

    #[inline]
    #[must_use]
    pub fn any() -> Self
    where
        Self: Default,
    {
        Self {
            marker: marker::any(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_sorted(&self) -> bool {
        self.marker == marker::sorted()
    }

    #[inline]
    pub fn contains_id(&self, id: &TypeId) -> bool {
        match self.marker {
            m if m == marker::any() => true,
            m if m == *id => true,
            m if m == marker::empty() => false,
            m if m == marker::sorted() => self.ids.borrow().binary_search(id).is_ok(),
            _ => self.ids.borrow().contains(id),
        }
    }

    #[inline]
    pub fn sort(&mut self)
    where
        C: BorrowMut<[TypeId]> + Extend<TypeId>,
    {
        match self.marker {
            m if m == marker::any() => return,
            m if m == marker::empty() => return,
            m if m == marker::sorted() => return,
            _ => {}
        }
        let id = core::mem::replace(&mut self.marker, marker::sorted());
        self.ids.extend([id]);
        self.ids.borrow_mut().sort_unstable();
    }
}

// impl<C, S: TypeSet> core::ops::BitOrAssign<S> for TypeArray<C>
// where
//     C: Borrow<[TypeId]>,
//     Self: FromIterator<TypeId>,
// {
//     #[inline]
//     fn bitor_assign(&mut self, rhs: S) {
//         if self.marker == marker::any() {
//             return;
//         }

//         match rhs.try_iter() {
//             Err(e) if e.is_any() => {
//                 self.marker = marker::any();
//             }
//             Err(_) => {}
//             Ok(ids) if self.marker == marker::empty() => {
//                 *self = ids.into_iter().collect();
//             }
//             Ok(ids) if self.marker.is_marker() => {
//                 let rhs_filter = rhs.as_filter();
//                 let own = self
//                     .ids
//                     .borrow()
//                     .iter()
//                     .filter(|id| !rhs_filter(id))
//                     .copied();
//                 let combined = iter::chain(own, ids.into_iter());
//                 *self = combined.collect();
//             }
//             Ok(ids) => {
//                 let rhs_filter = rhs.as_filter();
//                 let marker = iter::once_with(|| self.marker);
//                 let own = iter::chain(marker, self.ids.borrow().iter().copied());
//                 let filtered = own.filter(|id| !rhs_filter(id));
//                 let combined = iter::chain(filtered, ids.into_iter());
//                 *self = combined.collect();
//             }
//         }
//     }
// }

// impl<C, S: TypeSet> core::ops::BitAndAssign<S> for TypeArray<C>
// where
//     C: Borrow<[TypeId]>,
//     Self: FromIterator<TypeId>,
// {
//     #[inline]
//     fn bitand_assign(&mut self, rhs: S) {
//         if self.marker == marker::empty() {
//             return;
//         }

//         match rhs.try_iter() {
//             Err(e) if e.is_empty() => {
//                 self.marker = marker::empty();
//             }
//             Err(_) => {}
//             Ok(ids) if self.marker == marker::any() => {
//                 *self = ids.into_iter().collect();
//             }
//             Ok(ids) => {
//                 let f = self.as_filter();
//                 *self = ids.into_iter().filter(f).collect();
//             }
//         }
//     }
// }

// impl<const N: usize> Extend<TypeId> for TypeArray<[TypeId; N]> {
//     #[inline]
//     fn extend<T: IntoIterator<Item = TypeId>>(&mut self, iter: T) {
//         if self.marker == marker::any() {
//             return;
//         }
//         let iter = iter.into_iter();
//         if iter.size_hint().0 > N {
//             *self = Self::any()
//         }
//     }
// }

impl<const N: usize> FromIterator<TypeId> for TypeArray<[TypeId; N]> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = TypeId>>(iter: I) -> Self {
        let mut iter = iter.into_iter().filter(|id| *id != marker::never());

        match iter.size_hint() {
            (_, Some(0)) => {
                return Self::empty();
            }
            (low, _) if low > N => {
                return Self::any();
            }
            _ => {}
        }

        let mut arr = Self::empty();
        if let Some(id) = iter.next() {
            assert!(!id.is_marker());
            arr.marker = id;
        } else {
            return arr;
        }

        for (slot, id) in arr.ids.iter_mut().zip(iter.by_ref()) {
            assert!(!id.is_marker());
            *slot = id;
        }

        if iter.next().is_some() {
            arr.marker = marker::any();
        }

        arr
    }
}

// impl core::ops::BitOr for ErasedTypeSet {
//     type Output = Self;

//     #[inline]
//     fn bitor(self, rhs: Self) -> Self {
//         match (self.is_empty(), rhs.is_empty()) {
//             (true, true) => ErasedTypeSet::empty(),
//             (true, false) => rhs,
//             (false, true) => self,
//             _ => {
//                 use core::iter::once;
//                 let iter = once(self.marker)
//                     .chain(self.ids)
//                     .chain(once(rhs.marker))
//                     .chain(rhs.ids)
//                     .filter(|id| !id.is_marker());
//                 iter.collect()
//             }
//         }
//     }
// }

// impl core::ops::BitAnd for ErasedTypeSet {
//     type Output = Self;

//     #[inline]
//     fn bitand(self, rhs: Self) -> Self {
//         match (self.is_empty(), rhs.is_empty()) {
//             (true, ..) | (.., true) => ErasedTypeSet::empty(),
//             _ => {
//                 use core::iter::once;

//                 let lhs = once(self.marker)
//                     .chain(self.ids)
//                     .filter(|id| rhs.contains_id(id));

//                 let rhs = once(rhs.marker)
//                     .chain(rhs.ids)
//                     .filter(|id| self.contains_id(id));

//                 lhs.chain(rhs).filter(|id| !id.is_marker()).collect()
//             }
//         }
//     }
// }

impl<const N: usize> Default for TypeArray<[TypeId; N]> {
    #[inline]
    fn default() -> Self {
        Self {
            marker: marker::any(),
            ids: [TypeId::of::<Infallible>(); N],
        }
    }
}

impl Default for TypeArray<Box<[TypeId]>> {
    #[inline]
    fn default() -> Self {
        Self {
            marker: marker::any(),
            ids: Box::new([]),
        }
    }
}

impl Default for TypeArray<Vec<TypeId>> {
    #[inline]
    fn default() -> Self {
        Self {
            marker: marker::any(),
            ids: Vec::new(),
        }
    }
}

// impl<C: Borrow<[TypeId]>> TypeSet for TypeArray<C> {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         use crate::util::Bool;
//         if self.marker == marker::any() {
//             return Err::<_, Bool>(Bool::r#true());
//         }
//         if self.marker == marker::empty() {
//             return Err(Bool::r#false());
//         }
//         let iter = if self.marker == marker::sorted() {
//             iter::Chain::B(self.ids.borrow().iter().copied())
//         } else {
//             iter::chain(
//                 iter::once_with(|| self.marker),
//                 self.ids.borrow().iter().copied(),
//             )
//         };
//         Ok(iter)
//     }

//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         TypeArray::contains_id(self, &id)
//     }

//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         self.contains_id(&TypeId::of::<T>())
//     }

//     #[inline]
//     fn is_empty(&self) -> bool {
//         self.marker == marker::empty()
//     }
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Types<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Clone for Types<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Types<T> {}

impl<T: ?Sized> Types<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Types<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl super::TypeSet for Types<()> {
    #[inline]
    fn contains(&self, _typ: impl Type) -> bool {
        false
    }
    #[inline]
    fn into_simple(self) -> super::SimpleTypeSet<'static> {
        super::SimpleTypeSet::empty()
    }
}

impl crate::schema::Schema for Types<()> {}

// impl TypeValidator for Types<()> {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_type(&self, _typ: impl Type) -> bool {
//         false
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::r#false()
//     }
// }
// impl Snek<TypeMarker> for Types<()> {
//     type Head = Infallible;
//     type Tail = Self;

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Err(()), self)
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         0
//     }
// }

// impl TypeSet for Types<()> {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         Err::<core::iter::Empty<_>, _>(crate::util::FALSE)
//     }
//     #[inline]
//     fn contains_id(&self, _id: TypeId) -> bool {
//         false
//     }
//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn is_empty(&self) -> bool {
//         true
//     }
//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         |_| false
//     }
// }

macro_rules! impl_types {
    ($($T:ident $(,)?)*) => {
        impl< $($T: 'static, )* > Types<($($T,)*)> {
            #[allow(unused)]
            pub(crate) const SET: super::SimpleTypeSet<'static> = super::SimpleTypeSet::new(&[
                $(
                    ::core::any::TypeId::of::<$T>(),
                )*
            ], &[]);
        }

        impl< $($T: 'static, )* > super::TypeSet for Types<($($T,)*)> {
            #[inline]
            fn contains(&self, typ: impl Type) -> bool {
                $(typ.is::<$T>() ||)* false
            }
            #[inline]
            fn types(self) -> impl IntoIterator<Item: Type> {
                Self::SET
            }
            #[inline]
            fn into_simple(self) -> super::SimpleTypeSet<'static> {
                Self::SET
            }
        }

        impl< $($T: 'static ,)* > crate::schema::Schema for Types<($($T,)*)> {
            #[inline]
            fn accepted_value_types(&self) -> impl super::TypeSet {
                *self
            }
            #[inline]
            fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl super::TypeSet, impl super::TypeSet)> {
                [] as [(super::Any, super::Any);0]
            }
        }

        // impl< $($T: 'static ,)* > TypeValidator for Types<($($T,)*)> {
        //     #[inline]
        //     fn validate<T: 'static>(&self) -> bool {
        //         $(TypeId::of::<T>() == TypeId::of::<$T>() ||)* false
        //     }
        //     #[inline]
        //     fn validate_type(&self, typ: impl Type) -> bool {
        //         $(typ.is::<$T>() ||)* false
        //     }
        //     #[inline]
        //     fn decompose(
        //         self,
        //     ) -> Composition<
        //         impl TypeValidators,
        //         impl TypeValidators,
        //         impl TypeValidator,
        //         impl TypeSchema,
        //     > {
        //         Composition::union((
        //             $(
        //                 TypeOf::<$T>::new(),
        //             )*
        //         ))
        //     }
        // }
    };
}

impl_types!(T1);
impl_types!(T1, T2);
impl_types!(T1, T2, T3);
impl_types!(T1, T2, T3, T4);
impl_types!(T1, T2, T3, T4, T5);
impl_types!(T1, T2, T3, T4, T5, T6);
impl_types!(T1, T2, T3, T4, T5, T6, T7);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_types!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_types!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_types!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);

// impl<const N: usize> TypeValidator for [TypeId; N]
// where
//     [TypeId; N]: Snek<TypeMarker>,
// {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         self.contains(&TypeId::of::<T>())
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         use super::Maybe;
//         let Some(id) = typ.id().get() else {
//             return false;
//         };
//         self.contains(&id)
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::union(self)
//     }
// }

// impl<const N: usize> TypeSet for [TypeId; N] {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         if N == 0 {
//             return Err(crate::util::FALSE);
//         }
//         Ok(self.iter().copied())
//     }
//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         self.as_ref().contains(&id)
//     }
//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         self.as_ref().contains(&TypeId::of::<T>())
//     }
//     #[inline]
//     fn is_empty(&self) -> bool {
//         N == 0
//     }
// }

// impl<T1: TypeSet, T2: TypeSet> TypeSet for crate::util::Union<T1, T2> {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         use crate::util::TRUE;
//         use iter::{chain, optional, AnyOf};
//         let mut iter = Default::default();
//         match (self.0.try_iter(), self.1.try_iter()) {
//             (Ok(l), Ok(r)) => {
//                 iter = optional(AnyOf::A::<_, _, _>(chain(l.into_iter(), r.into_iter())));
//             }
//             (Err(e), _) if e.is_any() => {
//                 return Err(TRUE);
//             }
//             (_, Err(e)) if e.is_any() => {
//                 return Err(TRUE);
//             }
//             (Err(_), Ok(t)) => {
//                 iter = optional(AnyOf::B(t.into_iter()));
//             }
//             (Ok(t), Err(_)) => {
//                 iter = optional(AnyOf::C(t.into_iter()));
//             }
//             (Err(_), Err(_)) => {}
//         }

//         Ok(iter)
//     }

//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         self.0.contains_id(id) || self.1.contains_id(id)
//     }
//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         self.0.contains::<T>() || self.1.contains::<T>()
//     }
//     #[inline]
//     fn is_empty(&self) -> bool {
//         self.0.is_empty() && self.1.is_empty()
//     }
//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         let left = self.0.as_filter();
//         let right = self.1.as_filter();
//         move |id| left(id) || right(id)
//     }
// }

// impl<T1: TypeSet, T2: TypeSet> TypeSet for crate::util::Intersection<T1, T2> {
//     #[inline]
//     fn try_iter(&self) -> Result<impl IntoIterator<Item = TypeId>, impl IterError> {
//         use crate::util::FALSE;
//         use iter::{optional, AnyOf};

//         let mut iter = Default::default();
//         match (self.0.try_iter(), self.1.try_iter()) {
//             (Ok(l), Ok(r)) => {
//                 // Iterate over the smaller iterator and filter by the larger set
//                 let l = l.into_iter();
//                 let r = r.into_iter();
//                 if l.size_hint().0 < r.size_hint().0 {
//                     iter = optional(AnyOf::A(l.filter(self.1.as_filter())));
//                 } else {
//                     iter = optional(AnyOf::B(r.filter(self.0.as_filter())));
//                 }
//             }
//             (Err(e), _) if e.is_empty() => {
//                 return Err(FALSE);
//             }
//             (_, Err(e)) if e.is_empty() => {
//                 return Err(FALSE);
//             }
//             (Err(_), Ok(s)) => {
//                 iter = optional(AnyOf::C(s.into_iter()));
//             }
//             (Ok(s), Err(_)) => {
//                 iter = optional(AnyOf::D(s.into_iter()));
//             }
//             (Err(_), Err(_)) => {}
//         }

//         Ok(iter)
//     }
//     #[inline]
//     fn contains_id(&self, id: TypeId) -> bool {
//         self.0.contains_id(id) && self.1.contains_id(id)
//     }
//     #[inline]
//     fn contains<T: 'static + ?Sized>(&self) -> bool {
//         self.0.contains::<T>() && self.1.contains::<T>()
//     }
//     #[inline]
//     fn is_empty(&self) -> bool {
//         self.0.is_empty() || self.1.is_empty()
//     }
//     #[inline]
//     fn as_filter(&self) -> impl Fn(&TypeId) -> bool {
//         let left = self.0.as_filter();
//         let right = self.1.as_filter();
//         move |id| left(id) && right(id)
//     }
// }

#[allow(unreachable_pub, unused)]
pub mod iter {
    use crate::util::Never;
    use core::iter::FusedIterator;

    #[inline]
    pub const fn once_with<T>(f: impl FnOnce() -> T) -> OnceWith<impl FnOnce() -> T> {
        OnceWith::With(f)
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum OnceWith<F> {
        With(F),
        #[default]
        Done,
    }

    impl<T, F: FnOnce() -> T> Iterator for OnceWith<F> {
        type Item = T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if matches!(self, Self::Done) {
                return None;
            }
            let Self::With(f) = core::mem::take(self) else {
                return None;
            };

            Some(f())
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::With(_) => (1, Some(1)),
                Self::Done => (0, Some(0)),
            }
        }
    }

    impl<T, F: FnOnce() -> T> FusedIterator for OnceWith<F> {}

    #[inline]
    pub const fn chain<A, B>(a: A, b: B) -> Chain<A, B> {
        Chain::A(a, b)
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Chain<A, B> {
        A(A, B),
        B(B),
        #[default]
        End,
    }

    impl<A, B> Iterator for Chain<A, B>
    where
        A: Iterator,
        B: Iterator<Item = A::Item>,
    {
        type Item = A::Item;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if matches!(self, Self::A(..)) {
                let next = match self {
                    Self::A(a, ..) => a.next(),
                    _ => unreachable!(),
                };
                if next.is_some() {
                    return next;
                }

                let Self::A(_, mut b) = core::mem::take(self) else {
                    unreachable!()
                };

                let next = b.next();

                if next.is_some() {
                    *self = Self::B(b);
                }

                return next;
            }

            match self {
                Self::B(b) => b.next(),
                Self::End => None,
                Self::A(..) => unreachable!(),
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::A(a, b) => {
                    let (a_lower, a_upper) = a.size_hint();
                    let (b_lower, b_upper) = b.size_hint();
                    let lower = a_lower.saturating_add(b_lower);
                    let upper = if let (Some(a), Some(b)) = (a_upper, b_upper) {
                        Some(a.saturating_add(b))
                    } else {
                        None
                    };

                    (lower, upper)
                }
                Self::B(b) => b.size_hint(),
                Self::End => (0, Some(0)),
            }
        }
    }

    impl<A, B> FusedIterator for Chain<A, B>
    where
        A: FusedIterator,
        B: FusedIterator<Item = A::Item>,
    {
    }

    #[inline]
    pub const fn optional<T>(iter: T) -> Optional<T> {
        Optional::Iter(iter)
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Optional<T> {
        Iter(T),
        #[default]
        End,
    }

    impl<T> Iterator for Optional<T>
    where
        T: Iterator,
    {
        type Item = T::Item;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            let next = match self {
                Self::Iter(iter) => iter.next(),
                Self::End => return None,
            };

            if next.is_none() {
                *self = Self::End;
            }

            next
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::Iter(iter) => iter.size_hint(),
                Self::End => (0, Some(0)),
            }
        }
    }

    impl<T> FusedIterator for Optional<T> where T: FusedIterator {}

    #[derive(Debug, Clone, Copy)]
    pub enum AnyOf<A, B = Never, C = Never, D = Never> {
        A(A),
        B(B),
        C(C),
        D(D),
    }

    impl<A, B, C, D> Iterator for AnyOf<A, B, C, D>
    where
        A: Iterator,
        B: Iterator<Item = A::Item>,
        C: Iterator<Item = A::Item>,
        D: Iterator<Item = A::Item>,
    {
        type Item = A::Item;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::A(a) => a.next(),
                Self::B(b) => b.next(),
                Self::C(c) => c.next(),
                Self::D(d) => d.next(),
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                Self::A(a) => a.size_hint(),
                Self::B(b) => b.size_hint(),
                Self::C(c) => c.size_hint(),
                Self::D(d) => d.size_hint(),
            }
        }
    }

    impl<A, B, C, D> FusedIterator for AnyOf<A, B, C, D>
    where
        A: FusedIterator,
        B: FusedIterator<Item = A::Item>,
        C: FusedIterator<Item = A::Item>,
        D: FusedIterator<Item = A::Item>,
    {
    }
}

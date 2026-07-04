use core::any::TypeId;
use core::convert::Infallible;
use core::marker::PhantomData;

mod array;
mod set;
mod simple;

pub use array::{ErasedTypeSet, IterError, TypeArray, Types};
pub use set::TypeSet;
pub use simple::{SimpleType, SimpleTypeSet};

use crate::util::{Maybe, always};

#[inline(always)]
fn const_eq<T1: Type, T2: Type>(_t1: &T1, _t2: &T2) -> bool {
    let Some(id1) = T1::CONSTANT else {
        return false;
    };
    let Some(id2) = T2::CONSTANT else {
        return false;
    };
    id1 == id2
}

pub trait Type: Copy {
    const CONSTANT: Option<TypeId> = None;

    #[inline]
    #[must_use]
    fn id(&self) -> impl Maybe<TypeId> {}

    #[inline]
    #[must_use]
    fn name(&self) -> impl Maybe<&'static str> {}

    #[inline]
    #[must_use]
    fn is<T: 'static + ?Sized>(&self) -> bool {
        self.eq(TypeOf::<T>::new())
    }

    #[inline]
    fn eq(&self, other: impl Type) -> bool {
        if const_eq(self, &other) {
            return true;
        }

        if let Ok(id) = other.id().get()
            && self.id().eq(id)
        {
            return true;
        }

        if let Ok(name) = other.name().get()
            && self.name().eq(name)
        {
            return true;
        }

        false
    }
}

#[deny(clippy::missing_trait_methods)]
impl<T: Type> Type for &T {
    const CONSTANT: Option<TypeId> = T::CONSTANT;

    #[inline]
    fn id(&self) -> impl Maybe<TypeId> {
        (*self).id()
    }
    #[inline]
    fn name(&self) -> impl Maybe<&'static str> {
        (*self).name()
    }
    #[inline]
    fn is<U: 'static + ?Sized>(&self) -> bool {
        (*self).is::<U>()
    }
    #[inline]
    fn eq(&self, other: impl Type) -> bool {
        (*self).eq(other)
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum TypeImpl {
//     Id(TypeId),
//     Name(&'static str),
//     Either(TypeId, &'static str),
//     Both(TypeId, &'static str),
// }

// impl TypeImpl {
//     #[inline]
//     pub const fn for_type<T: 'static>() -> Self {
//         Self::Id(TypeId::of::<T>())
//     }

//     #[inline]
//     pub fn try_from(typ: impl Type) -> Result<Self,()> {
//         match (typ.id().get(), typ.name().get()) {
//             (Some(id), Some(name)) => Ok(Self::Both(id, name)),
//             (Some(id), None) => Ok(Self::Id(id)),
//             (None, Some(name)) => Ok(Self::Name(name)),
//             (None, None) => Err(()),
//         }
//     }
// }

// impl Type for TypeImpl {
//     #[inline]
//     fn id(&self) -> impl Maybe<TypeId> {
//         match *self {
//             Self::Id(id) => Some(id),
//             Self::Name(_) => None,
//             Self::Either(id, _) | Self::Both(id,_ )=> Some(id),
//         }
//     }
//     #[inline]
//     fn name(&self) -> impl Maybe<&'static str> {
//         match *self {
//             Self::Id(_) => None,
//             Self::Name(name) => Some(name),
//             Self::Either(_, name) | Self::Both(_, name) => Some(name),
//         }
//     }
//     #[inline]
//     fn is<T: 'static + ?Sized>(&self) -> bool {
//         match *self {
//             Self::Id(id) => id == TypeId::of::<T>(),
//             Self::Name(n) => n == type_name::<T>(),
//             Self::Either(id, n) => id == TypeId::of::<T>() || n == type_name::<T>(),
//             Self::Both(id, n) => id == TypeId::of::<T>() && n == type_name::<T>(),
//         }
//     }

//     #[inline]
//     fn eq(&self, other: impl Type) -> bool {
//         match *self {
//             Self::Id(id) => other.id().eq(id),
//             Self::Name(name) => other.name().eq(name),
//             Self::Either(id, n) => {
//                 other.id().eq(id) || other.name().eq(n)
//             },
//             Self::Both(id, n) => {
//                 other.id().eq(id) && other.name().eq(n)
//             },
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Any;

impl TypeSet for Any {
    #[inline]
    fn contains(&self, _typ: impl Type) -> bool {
        true
    }
}

impl Type for TypeId {
    #[inline]
    fn id(&self) -> impl Maybe<TypeId> {
        always(*self)
    }
    #[inline]
    fn is<T: 'static + ?Sized>(&self) -> bool {
        *self == TypeId::of::<T>()
    }
    #[inline]
    fn eq(&self, other: impl Type) -> bool {
        other.id().eq(*self)
    }
}

impl Type for &'static str {
    #[inline]
    fn name(&self) -> impl Maybe<&'static str> {
        always(*self)
    }
    #[inline]
    fn is<T: 'static + ?Sized>(&self) -> bool {
        *self == core::any::type_name::<T>()
    }
    #[inline]
    fn eq(&self, other: impl Type) -> bool {
        other.name().eq(*self)
    }
}

pub struct TypeOf<T: ?Sized + 'static>(PhantomData<T>);

impl<T: ?Sized + 'static> core::fmt::Debug for TypeOf<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TypeOf")
            .field("name", &core::any::type_name::<T>())
            .field("id", &TypeId::of::<T>())
            .finish()
    }
}

unsafe impl<T: ?Sized + 'static> Send for TypeOf<T> {}
unsafe impl<T: ?Sized + 'static> Sync for TypeOf<T> {}

impl<T: ?Sized + 'static> Clone for TypeOf<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + 'static> Copy for TypeOf<T> {}

impl<T: ?Sized + 'static> TypeOf<T> {
    pub(crate) const SET: SimpleTypeSet<'static> = SimpleTypeSet::new(&[TypeId::of::<T>()], &[]);

    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized + 'static> Default for TypeOf<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static + ?Sized> Type for TypeOf<T> {
    const CONSTANT: Option<TypeId> = Some(TypeId::of::<T>());

    #[inline]
    fn id(&self) -> impl Maybe<TypeId> {
        always(TypeId::of::<T>())
    }
    #[inline]
    fn name(&self) -> impl Maybe<&'static str> {
        always(core::any::type_name::<T>())
    }
}

impl<T: 'static + ?Sized> TypeSet for TypeOf<T> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.eq(typ)
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        core::iter::once(self)
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        SimpleTypeSet::for_type::<T>()
    }
}

impl<T: 'static + ?Sized> crate::schema::Schema for TypeOf<T> {
    #[inline]
    fn accepts_value<Ty: core::any::Any + ?Sized>(&self, _value: &Ty) -> bool {
        self.is::<Ty>()
    }
    #[inline]
    fn accepts_value_type<Ty: Type>(&self, typ: Ty) -> bool {
        self.eq(typ)
    }
    #[inline]
    fn accepts_value_of<Ty: core::any::Any + ?Sized>(&self) -> bool {
        self.is::<Ty>()
    }
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        *self
    }
    #[inline]
    fn accepts_link<L: crate::Link>(&self, _link: &L) -> bool {
        false
    }
    #[inline]
    fn accepts_link_type<L: crate::Link>(&self) -> bool {
        false
    }
    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [] as [(Any, Any); 0]
    }
}

// impl<T: 'static + ?Sized> TypeSchema for TypeOf<T> {
//     #[inline]
//     fn id(&self) -> Option<TypeId> {
//         Some(TypeId::of::<T>())
//     }
//     #[inline]
//     fn name(&self) -> Option<&'static str> {
//         Some(core::any::type_name::<T>())
//     }
//     #[inline]
//     fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
//         (Some(TypeId::of::<T>()), Some(core::any::type_name::<T>()))
//     }
// }

// impl<Ty: 'static> TypeValidator for TypeOf<Ty> {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         self.is::<T>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl self::Type) -> bool {
//         typ.is::<Ty>()
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::simple(self)
//     }
// }

impl Type for Infallible {}

impl TypeSet for Infallible {
    #[inline]
    fn contains(&self, _typ: impl Type) -> bool {
        false
    }
}

// Todo: remove
// impl Type for u128 {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct StringLike;

impl StringLike {
    pub(crate) const SET: SimpleTypeSet<'static> = SimpleTypeSet::new(
        &[
            TypeId::of::<&str>(),
            TypeId::of::<Box<str>>(),
            TypeId::of::<String>(),
        ],
        &["str", "String"],
    );
}

impl TypeSet for StringLike {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        typ.is::<&str>()
            || typ.is::<Box<str>>()
            || typ.is::<String>()
            || typ.name().eq("str")
            || typ.name().eq("String")
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        Self::SET
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        Self::SET
    }
}

impl crate::schema::Schema for StringLike {
    #[inline]
    fn accepts_value<Ty: core::any::Any + ?Sized>(&self, _value: &Ty) -> bool {
        self.contains(TypeOf::<Ty>::new())
    }
    #[inline]
    fn accepts_value_type<Ty: Type>(&self, typ: Ty) -> bool {
        self.contains(typ)
    }
    #[inline]
    fn accepts_value_of<Ty: core::any::Any + ?Sized>(&self) -> bool {
        self.contains(TypeOf::<Ty>::new())
    }
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        *self
    }
    #[inline]
    fn accepts_link<L: crate::Link>(&self, _link: &L) -> bool {
        false
    }
    #[inline]
    fn accepts_link_type<L: crate::Link>(&self) -> bool {
        false
    }
    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [] as [(Any, Any); 0]
    }
}

// impl TypeValidator for StringLike {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         let id = TypeId::of::<T>();
//         id == TypeId::of::<&str>() || id == TypeId::of::<Box<str>>() || id == TypeId::of::<String>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl self::Type) -> bool {
//         typ.is::<&str>() || typ.is::<Box<str>>() || typ.is::<String>()
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         type Validators = (TypeOf<&'static str>, TypeOf<Box<str>>, TypeOf<String>);
//         Composition::union(Validators::default())
//     }
//     #[inline]
//     fn into_repr(self) -> schema::TypeSchemaImpl<'static> {
//         use schema::TypeSchemaImpl;
//         static SCHEMAS: [TypeSchemaImpl; 3] = [
//             TypeSchemaImpl::Dyn(&TypeOf::<&str>::new()),
//             TypeSchemaImpl::Dyn(&TypeOf::<Box<str>>::new()),
//             TypeSchemaImpl::Dyn(&TypeOf::<String>::new()),
//         ];
//         TypeSchemaImpl::Union(&SCHEMAS)
//     }
// }

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BytesLike;

impl BytesLike {
    pub(crate) const SET: SimpleTypeSet<'static> = SimpleTypeSet::new(
        &[
            TypeId::of::<[u8]>(),
            TypeId::of::<&[u8]>(),
            TypeId::of::<Box<[u8]>>(),
            TypeId::of::<Vec<u8>>(),
        ],
        &["[u8]", "Vec<u8>", "Bytes", "bytes"],
    );
}

impl TypeSet for BytesLike {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        typ.is::<[u8]>()
            || typ.is::<&[u8]>()
            || typ.is::<Box<[u8]>>()
            || typ.is::<Vec<u8>>()
            || typ.name().eq("[u8]")
            || typ.name().eq("&[u8]")
            || typ.name().eq("Vec<u8>")
            || typ.name().eq("Bytes")
            || typ.name().eq("bytes")
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        Self::SET
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        Self::SET
    }
}

impl crate::schema::Schema for BytesLike {
    #[inline]
    fn accepts_value<Ty: core::any::Any + ?Sized>(&self, _value: &Ty) -> bool {
        self.contains(TypeOf::<Ty>::new())
    }
    #[inline]
    fn accepts_value_type<Ty: Type>(&self, typ: Ty) -> bool {
        self.contains(typ)
    }
    #[inline]
    fn accepts_value_of<Ty: core::any::Any + ?Sized>(&self) -> bool {
        self.contains(TypeOf::<Ty>::new())
    }
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        *self
    }
    #[inline]
    fn accepts_link<L: crate::Link>(&self, _link: &L) -> bool {
        false
    }
    #[inline]
    fn accepts_link_type<L: crate::Link>(&self) -> bool {
        false
    }
    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [] as [(Any, Any); 0]
    }
}

// impl TypeValidator for BytesLike {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         let id = TypeId::of::<T>();
//         id == TypeId::of::<&[u8]>()
//             || id == TypeId::of::<Box<[u8]>>()
//             || id == TypeId::of::<Vec<u8>>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         typ.is::<&[u8]>() || typ.is::<Box<[u8]>>() || typ.is::<Vec<u8>>()
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         type Validators = (TypeOf<&'static [u8]>, TypeOf<Box<[u8]>>, TypeOf<Vec<u8>>);
//         Composition::union(Validators::default())
//     }
//     #[inline]
//     fn into_repr(self) -> schema::TypeSchemaImpl<'static> {
//         use schema::TypeSchemaImpl;

//         static SCHEMAS: [TypeSchemaImpl; 3] = [
//             TypeSchemaImpl::Dyn(&TypeOf::<&[u8]>::new()),
//             TypeSchemaImpl::Dyn(&TypeOf::<Box<[u8]>>::new()),
//             TypeSchemaImpl::Dyn(&TypeOf::<Vec<u8>>::new()),
//         ];
//         schema::TypeSchemaImpl::Intersection(&SCHEMAS)
//     }
// }

// impl TypeSchema for TypeId {
//     #[inline]
//     fn id(&self) -> Option<TypeId> {
//         Some(*self)
//     }
//     #[inline]
//     fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
//         (Some(self), None)
//     }
// }

// impl TypeValidator for TypeId {
//     #[inline]
//     fn validate<T: 'static>(&self) -> bool {
//         *self == Self::of::<T>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         typ.id().get().is_some_and(|id| id == *self)
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::simple(self)
//     }
// }

pub const STRING_LIKE: StringLike = StringLike;
pub const BYTES_LIKE: BytesLike = BytesLike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_like() {
        assert!(STRING_LIKE.contains(TypeOf::<&str>::new()));
        assert!(STRING_LIKE.contains(TypeOf::<Box<str>>::new()));
        assert!(STRING_LIKE.contains(TypeOf::<String>::new()));
    }

    #[test]
    fn bytes_like() {
        assert!(BYTES_LIKE.contains(TypeOf::<&[u8]>::new()));
        assert!(BYTES_LIKE.contains(TypeOf::<Box<[u8]>>::new()));
        assert!(BYTES_LIKE.contains(TypeOf::<Vec<u8>>::new()));
    }
}

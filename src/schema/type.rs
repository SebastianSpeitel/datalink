use core::any::{TypeId, type_name};

use super::prelude::*;

#[deprecated]
pub trait Schema {
    fn id(&self) -> Option<TypeId>;
    #[inline]
    fn name(&self) -> Option<&str> {
        None
    }
    fn into_parts(self) -> (Option<TypeId>, Option<&'static str>);
}

impl core::fmt::Debug for dyn Schema + '_ + Sync {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Schema")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl Schema for &(dyn Schema + Sync + '_) {
    #[inline]
    fn id(&self) -> Option<TypeId> {
        (**self).id()
    }
    #[inline]
    fn name(&self) -> Option<&str> {
        (**self).name()
    }
    #[inline]
    fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
        (self.id(), None)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Simple<'a> {
    pub id: Option<TypeId>,
    pub name: Option<&'a str>,
}

impl Simple<'_> {
    #[inline]
    pub fn new(schema: impl Schema) -> Self {
        let (id, name) = schema.into_parts();
        Self { id, name }
    }
    fn into_owned(self) -> Simple<'static> {
        let name = self.name.and_then(|n| format_args!("{n}").as_str());
        Simple { id: self.id, name }
    }
}

impl Schema for Simple<'_> {
    #[inline]
    fn id(&self) -> Option<TypeId> {
        self.id
    }
    #[inline]
    fn name(&self) -> Option<&str> {
        self.name
    }
    #[inline]
    fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
        (self.id, None)
    }
}

#[derive(Debug, Clone)]
pub enum Impl<'a> {
    Intersection(&'a [Self]),
    IntersectionBox(Box<[Self]>),
    Union(&'a [Self]),
    UnionBox(Box<[Self]>),
    NotBox(Box<Self>),
    Not(&'a Self),
    Simple(Simple<'a>),
    SimpleNot(Simple<'a>),
    Lazy(fn() -> Self),
    SimpleLazy(fn() -> Simple<'a>),
    Dyn(&'a (dyn Schema + Sync + 'a)),
}

impl<'a> Impl<'a> {
    pub const ANY: Self = Self::Intersection(&[]);
    pub const NONE: Self = Self::Union(&[]);
    fn decompose_spec(self) -> Composition<Box<[Self]>, Box<[Self]>, Self, Simple<'a>> {
        match self {
            Self::Intersection(i) => Composition::Intersection(i.into()),
            Self::IntersectionBox(i) => Composition::Intersection(i),
            Self::Union(u) => Composition::Union(u.into()),
            Self::UnionBox(u) => Composition::Union(u),
            Self::Not(n) => Composition::Not(n.to_owned()),
            Self::NotBox(n) => Composition::Not(*n),
            Self::SimpleNot(s) => Composition::Not(Self::Simple(s)),
            Self::Simple(s) => Composition::Simple(s),
            Self::Lazy(f) => f().decompose_spec(),
            Self::SimpleLazy(f) => Composition::Simple(f()),
            Self::Dyn(s) => Composition::Simple(Simple::new(s)),
        }
    }

    #[inline]
    #[must_use]
    pub const fn for_type<T: 'static>() -> Self {
        Self::SimpleLazy(|| Simple {
            id: Some(TypeId::of::<T>()),
            name: Some(type_name::<T>()),
        })
    }
}

pub trait Validator {
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        self.validate_type(TypeOf::<T>::new())
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        let _ = typ;
        true
    }
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl Validator, impl Schema>;
    #[inline]
    fn into_repr(self) -> Impl<'static>
    where
        Self: Sized,
    {
        self.decompose().into()
    }
}

#[deny(clippy::missing_trait_methods)]
impl Validator for &(dyn Schema + '_ + Sync) {
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        let id_matches = self.id().is_none_or(|id| id == TypeId::of::<T>());
        let name_matches = self.name().is_none_or(|name| name == type_name::<T>());
        id_matches && name_matches
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        use crate::r#type::Maybe;
        if let Some(id) = self.id() {
            return typ.id().eq(id);
        }
        self.name() == typ.name().get()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl Validator, impl Schema> {
        Composition::simple(Simple::new(self))
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        Impl::Simple(Simple::new(self))
    }
}

#[deny(clippy::missing_trait_methods)]
impl Validator for Impl<'_> {
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        match self {
            Self::Intersection(i) => i.iter().all(Validator::validate::<T>),
            Self::IntersectionBox(i) => i.iter().all(Validator::validate::<T>),
            Self::Union(u) => u.iter().any(Validator::validate::<T>),
            Self::UnionBox(u) => u.iter().any(Validator::validate::<T>),
            Self::Not(n) => !n.validate::<T>(),
            Self::Lazy(f) => f().validate::<T>(),
            Self::SimpleNot(s) => !s.validate::<T>(),
            Self::NotBox(n) => !n.validate::<T>(),
            Self::Simple(s) => s.validate::<T>(),
            Self::SimpleLazy(f) => f().validate::<T>(),
            Self::Dyn(s) => s.validate::<T>(),
        }
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        match self {
            Self::Intersection(i) => i.iter().all(|s| s.validate_type(typ)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate_type(typ)),
            Self::Union(u) => u.iter().any(|s| s.validate_type(typ)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate_type(typ)),
            Self::Not(n) => !n.validate_type(typ),
            Self::Lazy(f) => f().validate_type(typ),
            Self::SimpleNot(s) => !s.validate_type(typ),
            Self::NotBox(n) => !n.validate_type(typ),
            Self::Simple(s) => s.validate_type(typ),
            Self::SimpleLazy(f) => f().validate_type(typ),
            Self::Dyn(s) => s.validate_type(typ),
        }
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl Schema>
    {
        self.decompose_spec()
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        self.decompose().into()
    }
}

#[deny(clippy::missing_trait_methods)]
impl TypeValidator for Simple<'_> {
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        self.id.is_some_and(|id| id == TypeId::of::<T>())
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        use crate::r#type::Maybe;
        if let Some(id) = self.id {
            return typ.id().eq(id);
        }
        self.name() == typ.name().get()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl Schema>
    {
        Composition::simple(self)
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        Impl::Simple(self.into_owned())
    }
}

impl<V: Validator + Clone> Validator for std::borrow::Cow<'_, V> {
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        self.as_ref().validate::<T>()
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        self.as_ref().validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl Validator, impl Schema> {
        self.into_owned().decompose()
    }
}

impl<I, U, N, S> From<Composition<I, U, N, S>> for Impl<'_>
where
    I: TypeValidators,
    U: TypeValidators,
    N: Validator,
    S: Schema,
{
    #[inline]
    fn from(composition: Composition<I, U, N, S>) -> Self {
        match composition {
            Composition::Intersection(i) if i.len() == 0 => {
                // An empty intersection is equivalent to "any" schema
                Self::ANY
            }
            Composition::Intersection(i) => Self::IntersectionBox(i.into_boxed_slice()),
            Composition::Union(u) if u.len() == 0 => {
                // An empty union is equivalent to "none" schema
                Self::NONE
            }
            Composition::Union(u) => Self::UnionBox(u.into_boxed_slice()),
            Composition::Not(n) => {
                let n = n.into_repr();
                match n {
                    Self::Simple(s) => Self::SimpleNot(s),
                    _ => Self::NotBox(Box::new(n)),
                }
            }
            Composition::Simple(s) => Self::Simple(Simple::new(s)),
        }
    }
}

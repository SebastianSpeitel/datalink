use std::sync::Arc;

use crate::{Data, ErasableData};

use super::{prelude::*, TypeSchemaImpl};

#[deprecated]
pub trait Schema {
    fn r#type(&self) -> &impl TypeValidator;
    fn constant(&self) -> Option<&(impl Data + Send + Sync + '_ + ?Sized)>;
    fn into_parts(
        self,
    ) -> (
        impl TypeValidator,
        Option<impl Data + Send + Sync + 'static>,
    );
}

#[derive(Debug, Clone)]
pub struct Simple<'a> {
    pub r#type: TypeSchemaImpl<'a>,
    pub constant: Option<Arc<dyn ErasableData + Send + Sync + 'a>>,
}

impl Simple<'_> {
    #[inline]
    pub fn new(schema: impl Schema) -> Self {
        let (r#type, constant) = schema.into_parts();
        let r#type = r#type.into_repr();
        let constant = constant.map(|c| Arc::new(c) as _);
        Self { r#type, constant }
    }
    fn into_owned(self) -> Simple<'static> {
        todo!()
    }
}

impl Schema for Simple<'_> {
    #[inline]
    fn r#type(&self) -> &impl TypeValidator {
        &self.r#type
    }
    #[inline]
    fn constant(&self) -> Option<&(impl Data + Send + Sync + '_ + ?Sized)> {
        self.constant.as_deref()
    }
    #[inline]
    fn into_parts(
        self,
    ) -> (
        impl TypeValidator,
        Option<impl Data + Send + Sync + 'static>,
    ) {
        (self.r#type, None::<core::convert::Infallible>)
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
        }
    }
}

pub trait Validator {
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        let _ = value;
        self.validate_type(TypeOf::<T>::new())
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        let _ = typ;
        true
    }
    fn decompose(
        self,
    ) -> Composition<impl ValueValidators, impl ValueValidators, impl Validator, impl Schema>;
    #[inline]
    fn into_repr(self) -> Impl<'static>
    where
        Self: Sized,
    {
        self.decompose().into()
    }
}

#[deny(clippy::missing_trait_methods)]
impl Validator for Impl<'_> {
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate(value)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate(value)),
            Self::Union(u) => u.iter().any(|s| s.validate(value)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate(value)),
            Self::Not(n) => !n.validate(value),
            Self::SimpleNot(s) => !s.validate(value),
            Self::Simple(s) => s.validate(value),
            Self::NotBox(n) => !n.validate(value),
        }
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate_type(typ)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate_type(typ)),
            Self::Union(u) => u.iter().any(|s| s.validate_type(typ)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate_type(typ)),
            Self::Not(n) => !n.validate_type(typ),
            Self::NotBox(n) => !n.validate_type(typ),
            Self::SimpleNot(s) => !s.validate_type(typ),
            Self::Simple(s) => s.validate_type(typ),
        }
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl ValueValidators, impl ValueValidators, impl Validator, impl Schema> {
        self.decompose_spec()
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        self.decompose_spec().into()
    }
}

#[deny(clippy::missing_trait_methods)]
impl Validator for Simple<'_> {
    #[inline]
    fn validate<T: 'static>(&self, _value: &T) -> bool {
        self.r#type.validate::<T>()
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        self.r#type.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl ValueValidators, impl ValueValidators, impl Validator, impl Schema> {
        Composition::simple(self)
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        Impl::Simple(self.into_owned())
    }
}

impl<V: Validator + Clone> Validator for std::borrow::Cow<'_, V> {
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        self.as_ref().validate(value)
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        self.as_ref().validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl ValueValidators, impl ValueValidators, impl Validator, impl Schema> {
        self.into_owned().decompose()
    }
}

impl<I, U, N, S> From<Composition<I, U, N, S>> for Impl<'_>
where
    I: ValueValidators,
    U: ValueValidators,
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

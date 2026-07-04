use crate::Link;

use super::{prelude::*, LinkSchemaImpl, ValueSchemaImpl};

#[deprecated]
pub trait Schema {
    fn value(&self) -> &impl ValueValidator;
    fn links(&self) -> Option<&impl LinkValidators>;
    fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>);
}

#[derive(Debug, Clone)]
pub struct Simple<'a> {
    pub value: ValueSchemaImpl<'a>,
    pub links: Option<Box<[LinkSchemaImpl<'a>]>>,
}

impl Simple<'_> {
    #[inline]
    pub fn new(schema: impl Schema) -> Self {
        let (value, links) = schema.into_parts();
        let value = value.into_repr();
        let links = links.map(super::LinkValidators::into_boxed_slice);
        Self { value, links }
    }
    fn into_owned(self) -> Simple<'static> {
        todo!()
    }
}

impl Schema for Simple<'_> {
    #[inline]
    fn value(&self) -> &impl ValueValidator {
        &self.value
    }
    #[inline]
    fn links(&self) -> Option<&impl LinkValidators> {
        self.links.as_ref()
    }
    #[inline]
    fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
        (self.value, self.links)
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
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        let _ = data;
        self.validate_value_type(TypeOf::<T>::new())
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        let _ = typ;
        true
    }
    #[inline]
    fn validate_link<L: Link>(&self, link: &L) -> bool {
        let _ = link;
        self.validate_link_type::<L>()
    }
    #[inline]
    fn validate_link_type<L: Link>(&self) -> bool {
        true
    }
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl Validator, impl DataSchema>;
    #[inline]
    fn into_repr(self) -> Impl<'static>
    where
        Self: Sized,
    {
        self.decompose().into()
    }
}

#[cfg(feature = "std")]
impl<V: Validator + Clone> Validator for std::borrow::Cow<'_, V> {
    #[inline]
    fn validate_link<L: Link>(&self, link: &L) -> bool {
        self.as_ref().validate_link(link)
    }
    #[inline]
    fn validate_link_type<L: Link>(&self) -> bool {
        self.as_ref().validate_link_type::<L>()
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        self.as_ref().validate_value(data)
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        self.as_ref().validate_value_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl Validator, impl Schema> {
        self.into_owned().decompose()
    }
}

#[deny(clippy::missing_trait_methods)]
impl Validator for Impl<'_> {
    #[inline]
    fn validate_link<L: crate::Link>(&self, link: &L) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate_link(link)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate_link(link)),
            Self::Union(u) => u.iter().any(|s| s.validate_link(link)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate_link(link)),
            Self::Not(n) => !n.validate_link(link),
            Self::NotBox(n) => !n.validate_link(link),
            Self::SimpleNot(s) => !s.validate_link(link),
            Self::Simple(s) => s.validate_link(link),
        }
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(Validator::validate_link_type::<L>),
            Self::IntersectionBox(i) => i.iter().all(Validator::validate_link_type::<L>),
            Self::Union(u) => u.iter().any(Validator::validate_link_type::<L>),
            Self::UnionBox(u) => u.iter().any(Validator::validate_link_type::<L>),
            Self::Not(n) => !n.validate_link_type::<L>(),
            Self::NotBox(n) => !n.validate_link_type::<L>(),
            Self::SimpleNot(s) => !s.validate_link_type::<L>(),
            Self::Simple(s) => s.validate_link_type::<L>(),
        }
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate_value(data)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate_value(data)),
            Self::Union(u) => u.iter().any(|s| s.validate_value(data)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate_value(data)),
            Self::Not(n) => !n.validate_value(data),
            Self::NotBox(n) => !n.validate_value(data),
            Self::SimpleNot(s) => !s.validate_value(data),
            Self::Simple(s) => s.validate_value(data),
        }
    }

    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate_value_type(typ)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate_value_type(typ)),
            Self::Union(u) => u.iter().any(|s| s.validate_value_type(typ)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate_value_type(typ)),
            Self::Not(n) => !n.validate_value_type(typ),
            Self::NotBox(n) => !n.validate_value_type(typ),
            Self::SimpleNot(s) => !s.validate_value_type(typ),
            Self::Simple(s) => s.validate_value_type(typ),
        }
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl Validator, impl Schema> {
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
    fn validate_link<L: crate::Link>(&self, link: &L) -> bool {
        let Some(links) = &self.links else {
            return true;
        };
        links.iter().all(|s| s.validate(link))
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        let Some(links) = &self.links else {
            return true;
        };
        links.iter().any(LinkValidator::validate_type::<L>)
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        self.value.validate(data)
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        self.value.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl Validator, impl Schema> {
        Composition::simple(self)
    }
    #[inline]
    fn into_repr(self) -> Impl<'static> {
        Impl::Simple(self.into_owned())
    }
}

impl<I, U, N, S> From<Composition<I, U, N, S>> for Impl<'_>
where
    I: DataValidators,
    U: DataValidators,
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

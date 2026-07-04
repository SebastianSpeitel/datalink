use crate::Link;

use super::{prelude::*, DataSchemaImpl};

#[deprecated]
pub trait Schema {
    fn key(&self) -> &impl DataValidator;
    fn target(&self) -> &impl DataValidator;
    fn optional(&self) -> bool;
    fn into_parts(self) -> (impl DataValidator, impl DataValidator, bool);
}

#[derive(Debug, Clone)]
pub struct Simple<'a> {
    pub key: DataSchemaImpl<'a>,
    pub target: DataSchemaImpl<'a>,
    pub optional: bool,
}

impl Simple<'_> {
    #[inline]
    pub fn new(schema: impl Schema) -> Self {
        let (key, target, optional) = schema.into_parts();
        let key = key.into_repr();
        let target = target.into_repr();
        Self {
            key,
            target,
            optional,
        }
    }
    fn into_owned(self) -> Simple<'static> {
        todo!()
    }
}

impl Schema for Simple<'_> {
    #[inline]
    fn key(&self) -> &impl DataValidator {
        &self.key
    }
    #[inline]
    fn target(&self) -> &impl DataValidator {
        &self.target
    }
    #[inline]
    fn optional(&self) -> bool {
        self.optional
    }
    #[inline]
    fn into_parts(self) -> (impl DataValidator, impl DataValidator, bool) {
        (self.key, self.target, self.optional)
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
    fn validate<L: Link>(&self, link: &L) -> bool {
        let _ = link;
        self.validate_type::<L>()
    }
    #[inline]
    fn validate_type<L: Link>(&self) -> bool {
        true
    }
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl Validator, impl Schema>;
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
    fn validate<L: crate::Link>(&self, link: &L) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(|s| s.validate(link)),
            Self::IntersectionBox(i) => i.iter().all(|s| s.validate(link)),
            Self::Union(u) => u.iter().any(|s| s.validate(link)),
            Self::UnionBox(u) => u.iter().any(|s| s.validate(link)),
            Self::Not(n) => !n.validate(link),
            Self::SimpleNot(s) => !s.validate(link),
            Self::Simple(s) => s.validate(link),
            Self::NotBox(n) => !n.validate(link),
        }
    }
    #[inline]
    fn validate_type<L: crate::Link>(&self) -> bool {
        match self {
            Self::Intersection(s) => s.iter().all(Validator::validate_type::<L>),
            Self::IntersectionBox(i) => i.iter().all(Validator::validate_type::<L>),
            Self::Union(u) => u.iter().any(Validator::validate_type::<L>),
            Self::UnionBox(u) => u.iter().any(Validator::validate_type::<L>),
            Self::Not(n) => !n.validate_type::<L>(),
            Self::SimpleNot(s) => !s.validate_type::<L>(),
            Self::Simple(s) => s.validate_type::<L>(),
            Self::NotBox(n) => !n.validate_type::<L>(),
        }
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl Validator, impl Schema> {
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
    fn validate<L: crate::Link>(&self, _link: &L) -> bool {
        self.key.validate_value_type(L::Key::default())
            && self.target.validate_value_type(L::Target::default())
    }
    #[inline]
    fn validate_type<L: crate::Link>(&self) -> bool {
        self.key.validate_value_type(L::Key::default())
            && self.target.validate_value_type(L::Target::default())
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl Validator, impl Schema> {
        Composition::simple(self)
    }
    #[inline]
    fn into_repr(self) -> Impl<'static>
where {
        Impl::Simple(self.into_owned())
    }
}

impl<V: Validator + Clone> Validator for std::borrow::Cow<'_, V> {
    #[inline]
    fn validate<L: Link>(&self, link: &L) -> bool {
        self.as_ref().validate(link)
    }
    #[inline]
    fn validate_type<L: Link>(&self) -> bool {
        self.as_ref().validate_type::<L>()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl Validator, impl Schema> {
        self.into_owned().decompose()
    }
}

impl<I, U, N, S> From<Composition<I, U, N, S>> for Impl<'static>
where
    I: LinkValidators,
    U: LinkValidators,
    N: Validator,
    S: Schema,
{
    #[inline]
    fn from(comp: Composition<I, U, N, S>) -> Self {
        match comp {
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

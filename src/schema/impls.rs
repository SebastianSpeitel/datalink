use core::any::TypeId;
use core::convert::Infallible;

use crate::{
    r#type::Type,
    util::{Bool, IntoTrait, Snek},
    Link,
};

use super::{
    prelude::*, And, DataMarker, DataSchemaImpl, LinkMarker, Not, Or, TypeMarker, ValueMarker,
};

impl ValueValidator for ValueMarker {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        Composition::r#true()
    }
}

// DataMarker
impl DataValidator for DataMarker {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        Composition::r#true()
    }
}

// LinkMarker
impl LinkValidator for LinkMarker {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        Composition::r#true()
    }
}

// TypeMarker
impl TypeValidator for TypeMarker {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        Composition::r#true()
    }
}

// Infallible
impl DataValidator for Infallible {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        Composition::r#true()
    }
}
impl DataSchema for Infallible {
    #[inline]
    fn value(&self) -> &impl ValueValidator {
        &TRUE
    }
    #[inline]
    fn links(&self) -> Option<&impl LinkValidators> {
        None::<&Self>
    }
    #[inline]
    fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
        (TRUE, None::<Self>)
    }
}
impl ValueValidator for Infallible {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        Composition::r#false()
    }
}
impl ValueSchema for Infallible {
    #[inline]
    fn r#type(&self) -> &impl TypeValidator {
        &TRUE
    }
    #[inline]
    fn constant(&self) -> Option<&(impl crate::Data + Send + Sync + '_ + ?Sized)> {
        None::<&Self>
    }
    #[inline]
    fn into_parts(
        self,
    ) -> (
        impl TypeValidator,
        Option<impl crate::Data + Send + Sync + 'static>,
    ) {
        (TRUE, None::<Self>)
    }
}
impl LinkValidator for Infallible {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        Composition::r#false()
    }
}
impl LinkSchema for Infallible {
    #[inline]
    fn key(&self) -> &impl DataValidator {
        &TRUE
    }
    #[inline]
    fn target(&self) -> &impl DataValidator {
        &TRUE
    }
    #[inline]
    fn optional(&self) -> bool {
        true
    }
    #[inline]
    fn into_parts(self) -> (impl DataValidator, impl DataValidator, bool) {
        (TRUE, TRUE, true)
    }
}
impl TypeValidator for Infallible {
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        Composition::r#false()
    }
}
impl TypeSchema for Infallible {
    #[inline]
    fn id(&self) -> Option<TypeId> {
        None
    }
    #[inline]
    fn name(&self) -> Option<&'static str> {
        None
    }
    #[inline]
    fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
        (None, None)
    }
}

// Bool
impl<M, T, F> Snek<M> for Bool<T, F>
where
    Infallible: IntoTrait<M>,
    T: IntoTrait<M>,
{
    type Head = T;
    type Tail = ();
    #[inline]
    fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
        match self {
            Self::True(t) => (Ok(t), ()),
            Self::False(f) => (Err(f), ()),
        }
    }
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::True(_) => 1,
            Self::False(_) => 0,
        }
    }
}
impl<T: Copy, F: Copy> DataValidator for Bool<T, F> {
    #[inline]
    fn validate_link<L: crate::Link>(&self, _link: &L) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_value<_T: 'static>(&self, _data: &_T) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_value_type(&self, _typ: impl Type) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        match self {
            Self::True(t) => {
                Composition::Intersection::<_, _, Infallible, Infallible>(Bool::False::<
                    Infallible,
                    _,
                >(t))
            }
            Self::False(f) => Composition::Union(Bool::False::<Infallible, _>(f)),
        }
    }
}
impl<T: Copy, F: Copy> DataSchema for Bool<T, F> {
    #[inline]
    fn value(&self) -> &impl ValueValidator {
        self
    }
    #[inline]
    fn links(&self) -> Option<&impl LinkValidators> {
        match self {
            Self::True(_) => Some(&(TRUE,)),
            Self::False(_) => None,
        }
    }
    #[inline]
    fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
        (self, Some((self,)))
    }
}
impl<T: Copy, F: Copy> ValueValidator for Bool<T, F> {
    #[inline]
    fn validate<_T: 'static>(&self, _value: &_T) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_type(&self, _typ: impl Type) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        match self {
            Self::True(t) => {
                Composition::Intersection::<_, _, Infallible, Infallible>(Bool::False::<
                    Infallible,
                    _,
                >(t))
            }
            Self::False(f) => Composition::Union(Bool::False::<Infallible, _>(f)),
        }
    }
}
impl<T: Copy, F: Copy> LinkValidator for Bool<T, F> {
    #[inline]
    fn validate<L: Link>(&self, _link: &L) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_type<L: Link>(&self) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        match self {
            Self::True(t) => {
                Composition::Intersection::<_, _, Infallible, Infallible>(Bool::False::<
                    Infallible,
                    _,
                >(t))
            }
            Self::False(f) => Composition::Union(Bool::False::<Infallible, _>(f)),
        }
    }
}
impl<T: Copy, F: Copy> TypeValidator for Bool<T, F> {
    #[inline]
    fn validate<_T: 'static>(&self) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn validate_type(&self, _typ: impl Type) -> bool {
        matches!(self, Self::True(_))
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        match self {
            Self::True(t) => {
                Composition::Intersection::<_, _, Infallible, Infallible>(Bool::False::<
                    Infallible,
                    _,
                >(t))
            }
            Self::False(f) => Composition::Union(Bool::False::<Infallible, _>(f)),
        }
    }
}

// And
#[deny(clippy::missing_trait_methods)]
impl<S0, S1> DataValidator for And<S0, S1>
where
    S0: DataValidator,
    S1: DataValidator,
{
    #[inline]
    fn validate_link<L: crate::Link>(&self, link: &L) -> bool {
        self.0.validate_link(link) && self.1.validate_link(link)
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        self.0.validate_link_type::<L>() && self.1.validate_link_type::<L>()
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        self.0.validate_value(data) && self.1.validate_value(data)
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        self.0.validate_value_type(typ) && self.1.validate_value_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        Composition::Intersection::<_, Infallible, Infallible, Infallible>((self.0, self.1))
    }
    #[inline]
    fn into_repr(self) -> DataSchemaImpl<'static> {
        self.decompose().into()
    }
}
impl<S0, S1> ValueValidator for And<S0, S1>
where
    S0: ValueValidator,
    S1: ValueValidator,
{
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        self.0.validate(value) && self.1.validate(value)
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        self.0.validate_type(typ) && self.1.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        Composition::Intersection::<_, Infallible, Infallible, Infallible>((self.0, self.1))
    }
}
impl<S0, S1> LinkValidator for And<S0, S1>
where
    S0: LinkValidator,
    S1: LinkValidator,
{
    #[inline]
    fn validate<L: crate::Link>(&self, link: &L) -> bool {
        self.0.validate(link) && self.1.validate(link)
    }
    #[inline]
    fn validate_type<L: crate::Link>(&self) -> bool {
        self.0.validate_type::<L>() && self.1.validate_type::<L>()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        Composition::Intersection::<_, Infallible, Infallible, Infallible>((self.0, self.1))
    }
}
impl<S0, S1> TypeValidator for And<S0, S1>
where
    S0: TypeValidator,
    S1: TypeValidator,
{
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        self.0.validate::<T>() && self.1.validate::<T>()
    }
    #[inline]
    fn validate_type(&self, typ: impl crate::r#type::Type) -> bool {
        self.0.validate_type(typ) && self.1.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        Composition::Intersection::<_, Infallible, Infallible, Infallible>((self.0, self.1))
    }
}

// Or
#[deny(clippy::missing_trait_methods)]
impl<S0, S1> DataValidator for Or<S0, S1>
where
    S0: DataValidator,
    S1: DataValidator,
{
    #[inline]
    fn validate_link<L: crate::Link>(&self, link: &L) -> bool {
        self.0.validate_link(link) || self.1.validate_link(link)
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        self.0.validate_link_type::<L>() || self.1.validate_link_type::<L>()
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        self.0.validate_value(data) || self.1.validate_value(data)
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        self.0.validate_value_type(typ) || self.1.validate_value_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        Composition::Union::<Infallible, _, Infallible, Infallible>((self.0, self.1))
    }
    #[inline]
    fn into_repr(self) -> DataSchemaImpl<'static> {
        self.decompose().into()
    }
}
impl<S0, S1> ValueValidator for Or<S0, S1>
where
    S0: ValueValidator,
    S1: ValueValidator,
{
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        self.0.validate(value) || self.1.validate(value)
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        self.0.validate_type(typ) || self.1.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        Composition::Union::<Infallible, _, Infallible, Infallible>((self.0, self.1))
    }
}
impl<S0, S1> LinkValidator for Or<S0, S1>
where
    S0: LinkValidator,
    S1: LinkValidator,
{
    #[inline]
    fn validate<L: crate::Link>(&self, link: &L) -> bool {
        self.0.validate(link) || self.1.validate(link)
    }
    #[inline]
    fn validate_type<L: crate::Link>(&self) -> bool {
        self.0.validate_type::<L>() || self.1.validate_type::<L>()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        Composition::Union::<Infallible, _, Infallible, Infallible>((self.0, self.1))
    }
}
impl<S0, S1> TypeValidator for Or<S0, S1>
where
    S0: TypeValidator,
    S1: TypeValidator,
{
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        self.0.validate::<T>() || self.1.validate::<T>()
    }
    #[inline]
    fn validate_type(&self, typ: impl crate::r#type::Type) -> bool {
        self.0.validate_type(typ) || self.1.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        Composition::Union::<Infallible, _, Infallible, Infallible>((self.0, self.1))
    }
}

// Not
impl<S> DataValidator for Not<S>
where
    S: DataValidator,
{
    #[inline]
    fn validate_link<L: crate::Link>(&self, link: &L) -> bool {
        !self.0.validate_link(link)
    }
    #[inline]
    fn validate_link_type<L: crate::Link>(&self) -> bool {
        !self.0.validate_link_type::<L>()
    }
    #[inline]
    fn validate_value<T: 'static>(&self, data: &T) -> bool {
        !self.0.validate_value(data)
    }
    #[inline]
    fn validate_value_type(&self, typ: impl Type) -> bool {
        !self.0.validate_value_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
    {
        Composition::Not::<Infallible, Infallible, _, Infallible>(self.0)
    }
}
impl<S> ValueValidator for Not<S>
where
    S: ValueValidator,
{
    #[inline]
    fn validate<T: 'static>(&self, value: &T) -> bool {
        !self.0.validate(value)
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        !self.0.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<
        impl ValueValidators,
        impl ValueValidators,
        impl ValueValidator,
        impl ValueSchema,
    > {
        Composition::Not::<Infallible, Infallible, _, Infallible>(self.0)
    }
}
impl<S> LinkValidator for Not<S>
where
    S: LinkValidator,
{
    #[inline]
    fn validate<L: crate::Link>(&self, link: &L) -> bool {
        !self.0.validate(link)
    }
    #[inline]
    fn validate_type<L: crate::Link>(&self) -> bool {
        !self.0.validate_type::<L>()
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
    {
        Composition::Not::<Infallible, Infallible, _, Infallible>(self.0)
    }
}
impl<S> TypeValidator for Not<S>
where
    S: TypeValidator,
{
    #[inline]
    fn validate<T: 'static>(&self) -> bool {
        !self.0.validate::<T>()
    }
    #[inline]
    fn validate_type(&self, typ: impl Type) -> bool {
        !self.0.validate_type(typ)
    }
    #[inline]
    fn decompose(
        self,
    ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
    {
        Composition::Not::<Infallible, Infallible, _, Infallible>(self.0)
    }
}

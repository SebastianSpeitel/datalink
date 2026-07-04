use core::convert::Infallible;

use crate::Link;
use crate::r#type::{Any, SimpleTypeSet, Type, TypeOf, TypeSet};
use crate::util::Bool;

pub use crate::util::{FALSE, TRUE};

// pub mod builder;
// mod data;
// pub mod debug;
// mod impls;
// mod link;
// mod r#type;
// mod value;

// pub use data::{Impl as DataSchemaImpl, Schema as DataSchema, Validator as DataValidator};
// pub use link::{Impl as LinkSchemaImpl, Schema as LinkSchema, Validator as LinkValidator};
// pub use r#type::{Impl as TypeSchemaImpl, Schema as TypeSchema, Validator as TypeValidator};
// pub use value::{Impl as ValueSchemaImpl, Schema as ValueSchema, Validator as ValueValidator};

pub mod prelude {
    // pub use super::{
    //     Composition, DataValidator, DataValidators, LinkValidator, LinkValidators, TypeValidator,
    //     TypeValidators, ValueValidator, ValueValidators, FALSE, TRUE,
    // };
    pub use crate::r#type::{Type, TypeOf};
}

pub trait Schema {
    #[inline]
    fn accepts_value<T: core::any::Any + ?Sized>(&self, _value: &T) -> bool {
        self.accepts_value_type(TypeOf::<T>::new())
    }

    #[inline]
    fn accepts_value_of<T: core::any::Any + ?Sized>(&self) -> bool {
        self.accepts_value_type(TypeOf::<T>::new())
    }

    #[inline]
    fn accepts_value_type<T: Type>(&self, typ: T) -> bool {
        self.accepted_value_types().contains(typ)
    }

    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        Any
    }

    #[inline]
    fn accepts_link<L: Link>(&self, link: &L) -> bool {
        let _ = link;
        self.accepts_link_type::<L>()
    }

    #[inline]
    fn accepts_link_type<L: Link>(&self) -> bool {
        // dbg!(
        //     "accepts_link_type",
        //     core::any::type_name_of_val(&self),
        //     core::any::type_name::<L>(),
        //     core::any::type_name::<L::Key>(),
        //     core::any::type_name::<L::Target>()
        // );

        let key_type = L::Key::default();
        let target_type = L::Target::default();

        self.accepted_link_types()
            .into_iter()
            // .inspect(|t| {
            //     dbg!(core::any::type_name_of_val(t));
            // })
            .any(|(k, t)| k.contains(key_type) && t.contains(target_type))
    }

    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [(Any, Any)]
    }
}

struct LinkSchema<'a> {
    key_type: SimpleTypeSet<'a>,
    target_type: SimpleTypeSet<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleSchema<'a> {
    value_types: SimpleTypeSet<'a>,
    link_types: &'a [(SimpleTypeSet<'a>, SimpleTypeSet<'a>)],
}

impl<'a> SimpleSchema<'a> {
    pub const ANY: Self = Self {
        value_types: SimpleTypeSet::ANY,
        link_types: &[(SimpleTypeSet::ANY, SimpleTypeSet::ANY)],
    };

    pub const ANY_VALUE: Self = Self {
        value_types: SimpleTypeSet::ANY,
        link_types: &[],
    };

    pub const ANY_LINK: Self = Self {
        value_types: SimpleTypeSet::empty(),
        link_types: &[(SimpleTypeSet::ANY, SimpleTypeSet::ANY)],
    };

    pub const EMPTY: Self = Self {
        value_types: SimpleTypeSet::empty(),
        link_types: &[],
    };

    #[inline]
    #[must_use]
    pub const fn values(value_types: SimpleTypeSet<'a>) -> Self {
        Self {
            value_types,
            link_types: &[],
        }
    }

    #[inline]
    #[must_use]
    pub const fn links(link_types: &'a [(SimpleTypeSet<'a>, SimpleTypeSet<'a>)]) -> Self {
        Self {
            value_types: SimpleTypeSet::empty(),
            link_types,
        }
    }
}

impl Schema for SimpleSchema<'_> {
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        self.value_types
    }

    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        self.link_types.iter().map(|(k, t)| (k, t))
    }
}

impl<T, F> Schema for Bool<T, F> {
    #[inline]
    fn accepts_value<TY: 'static + ?Sized>(&self, _value: &TY) -> bool {
        self == true
    }
    #[inline]
    fn accepts_value_of<TY: 'static + ?Sized>(&self) -> bool {
        self == true
    }
    #[inline]
    fn accepts_value_type<TY: Type>(&self, _typ: TY) -> bool {
        self == true
    }
    #[inline]
    fn accepts_link<L: Link>(&self, _link: &L) -> bool {
        self == true
    }
    #[inline]
    fn accepts_link_type<L: Link>(&self) -> bool {
        self == true
    }
}

// pub trait DataValidators: Snek<DataMarker> {
//     #[inline]
//     fn into_boxed_slice(self) -> Box<[DataSchemaImpl<'static>]>
//     where
//         Self: Sized,
//     {
//         struct Push<'a>(&'a mut Vec<DataSchemaImpl<'static>>);
//         impl crate::util::SnekFn<DataMarker> for Push<'_> {
//             type Output = ();
//             fn call(&mut self, item: impl IntoTrait<DataMarker>) -> Self::Output {
//                 self.0.push(item.into_data_schema().into_repr());
//             }
//         }
//         if self.is_empty() {
//             return Box::new([]);
//         }
//         let mut vec = Vec::with_capacity(self.len());
//         self.for_each(Push(&mut vec));
//         debug_assert_eq!(vec.len(), vec.capacity());
//         vec.into_boxed_slice()
//     }
// }
// pub trait LinkValidators: Snek<LinkMarker> {
//     #[inline]
//     fn into_boxed_slice(self) -> Box<[LinkSchemaImpl<'static>]>
//     where
//         Self: Sized,
//     {
//         struct Push<'a>(&'a mut Vec<LinkSchemaImpl<'static>>);
//         impl crate::util::SnekFn<LinkMarker> for Push<'_> {
//             type Output = ();
//             fn call(&mut self, item: impl IntoTrait<LinkMarker>) -> Self::Output {
//                 self.0.push(item.into_link_schema().into_repr());
//             }
//         }
//         if self.is_empty() {
//             return Box::new([]);
//         }
//         let mut vec = Vec::with_capacity(self.len());
//         self.for_each(Push(&mut vec));
//         debug_assert_eq!(vec.len(), vec.capacity());
//         vec.into_boxed_slice()
//     }
// }
// pub trait ValueValidators: Snek<ValueMarker> {
//     #[inline]
//     fn into_boxed_slice(self) -> Box<[ValueSchemaImpl<'static>]>
//     where
//         Self: Sized,
//     {
//         struct Push<'a>(&'a mut Vec<ValueSchemaImpl<'static>>);
//         impl crate::util::SnekFn<ValueMarker> for Push<'_> {
//             type Output = ();
//             fn call(&mut self, item: impl IntoTrait<ValueMarker>) -> Self::Output {
//                 self.0.push(item.into_value_schema().into_repr());
//             }
//         }
//         if self.is_empty() {
//             return Box::new([]);
//         }
//         let mut vec = Vec::with_capacity(self.len());
//         self.for_each(Push(&mut vec));
//         debug_assert_eq!(vec.len(), vec.capacity());
//         vec.into_boxed_slice()
//     }
// }
// pub trait TypeValidators: Snek<TypeMarker> {
//     #[inline]
//     fn into_boxed_slice(self) -> Box<[TypeSchemaImpl<'static>]>
//     where
//         Self: Sized,
//     {
//         struct Push<'a>(&'a mut Vec<TypeSchemaImpl<'static>>);
//         impl crate::util::SnekFn<TypeMarker> for Push<'_> {
//             type Output = ();
//             fn call(&mut self, item: impl IntoTrait<TypeMarker>) -> Self::Output {
//                 self.0.push(item.into_type_schema().into_repr());
//             }
//         }
//         if self.is_empty() {
//             return Box::new([]);
//         }
//         let mut vec = Vec::with_capacity(self.len());
//         self.for_each(Push(&mut vec));
//         debug_assert_eq!(vec.len(), vec.capacity());
//         vec.into_boxed_slice()
//     }
// }

// impl<S: Snek<TypeMarker>> TypeValidators for S {}
// impl<S: Snek<DataMarker>> DataValidators for S {}
// impl<S: Snek<LinkMarker>> LinkValidators for S {}
// impl<S: Snek<ValueMarker>> ValueValidators for S {}

// #[derive(Debug, Clone, Copy)]
// pub enum ValueMarker {}
// #[derive(Debug, Clone, Copy)]
// pub enum DataMarker {}
// #[derive(Debug, Clone, Copy)]
// pub enum LinkMarker {}
// #[derive(Debug, Clone, Copy)]
// pub enum TypeMarker {}

#[derive(Debug, Clone, Copy)]
pub enum Composition<I, U, N, S> {
    Intersection(I),
    Union(U),
    Not(N),
    Simple(S),
}

impl<S> Composition<Infallible, Infallible, Infallible, S> {
    #[inline]
    pub const fn simple(value: S) -> Self {
        Self::Simple(value)
    }
}

impl Composition<(), Infallible, Infallible, Infallible> {
    #[inline]
    #[must_use]
    pub const fn r#true() -> Self {
        Self::Intersection(())
    }
}

impl Composition<Infallible, (), Infallible, Infallible> {
    #[inline]
    #[must_use]
    pub const fn r#false() -> Self {
        Self::Union(())
    }
}

impl<I> Composition<I, Infallible, Infallible, Infallible> {
    #[inline]
    pub const fn intersection(value: I) -> Self {
        Self::Intersection(value)
    }
}

impl<U> Composition<Infallible, U, Infallible, Infallible> {
    #[inline]
    pub const fn union(value: U) -> Self {
        Self::Union(value)
    }
}

impl<N> Composition<Infallible, Infallible, N, Infallible> {
    #[inline]
    pub const fn not(value: N) -> Self {
        Self::Not(value)
    }
}

// unsafe impl<S: ValueValidator> IntoTrait<ValueMarker> for S {
//     #[inline]
//     fn into_value_schema(self) -> impl crate::schema::ValueValidator {
//         self
//     }
// }

// unsafe impl<S: LinkValidator> IntoTrait<LinkMarker> for S {
//     #[inline]
//     fn into_link_schema(self) -> impl crate::schema::LinkValidator {
//         self
//     }
// }

// unsafe impl<S: DataValidator> IntoTrait<DataMarker> for S {
//     #[inline]
//     fn into_data_schema(self) -> impl crate::schema::DataValidator {
//         self
//     }
// }

// unsafe impl<S: TypeValidator> IntoTrait<TypeMarker> for S {
//     #[inline]
//     fn into_type_schema(self) -> impl crate::schema::TypeValidator {
//         self
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct And<A, B>(pub A, pub B);

#[derive(Debug, Clone, Copy)]
pub struct Or<A, B>(pub A, pub B);

#[derive(Debug, Clone, Copy)]
pub struct Not<S>(pub S);

#[derive(Debug, Clone, Copy)]
pub struct OnlyLinks;

pub const ONLY_LINKS: OnlyLinks = OnlyLinks;

impl OnlyLinks {
    pub(crate) const SCHEMA: SimpleSchema<'static> =
        SimpleSchema::links(&[(SimpleTypeSet::ANY, SimpleTypeSet::ANY)]);
}

impl Schema for OnlyLinks {
    #[inline]
    fn accepts_value<T: core::any::Any + ?Sized>(&self, _value: &T) -> bool {
        false
    }
    #[inline]
    fn accepts_value_of<T: core::any::Any + ?Sized>(&self) -> bool {
        false
    }
    #[inline]
    fn accepts_value_type<T: Type>(&self, _typ: T) -> bool {
        false
    }
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        [] as [Infallible; 0]
    }
    #[inline]
    fn accepts_link_type<L: Link>(&self) -> bool {
        true
    }
    #[inline]
    fn accepts_link<L: Link>(&self, _link: &L) -> bool {
        true
    }
    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [(crate::r#type::Any, crate::r#type::Any)]
    }
}

// impl DataSchema for OnlyLinks {
//     #[inline]
//     fn value(&self) -> &impl ValueValidator {
//         &FALSE
//     }
//     #[inline]
//     fn links(&self) -> Option<&impl LinkValidators> {
//         Some(&(TRUE,))
//     }
//     #[inline]
//     fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
//         (FALSE, Some((TRUE,)))
//     }
// }
// impl DataValidator for OnlyLinks {
//     #[inline]
//     fn validate_value<T: 'static>(&self, _data: &T) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_value_type(&self, typ: impl prelude::Type) -> bool {
//         let _ = typ;
//         false
//     }
//     #[inline]
//     fn validate_link<L: Link>(&self, _link: &L) -> bool {
//         true
//     }
//     #[inline]
//     fn validate_link_type<L: Link>(&self) -> bool {
//         true
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct OnlyValues;

pub const ONLY_VALUES: OnlyValues = OnlyValues;

impl OnlyValues {
    pub(crate) const SCHEMA: SimpleSchema<'static> = SimpleSchema::ANY_VALUE;
}

impl Schema for OnlyValues {
    #[inline]
    fn accepted_value_types(&self) -> impl TypeSet {
        crate::r#type::Any
    }
    #[inline]
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)> {
        [] as [([Infallible; 0], [Infallible; 0]); 0]
    }
}

// impl DataSchema for OnlyValues {
//     #[inline]
//     fn value(&self) -> &impl ValueValidator {
//         &TRUE
//     }
//     #[inline]
//     fn links(&self) -> Option<&impl LinkValidators> {
//         None::<&Infallible>
//     }
//     #[inline]
//     fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
//         (TRUE, None::<Infallible>)
//     }
// }

// impl DataValidator for OnlyValues {
//     #[inline]
//     fn validate_link<L: Link>(&self, _link: &L) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_link_type<L: Link>(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_value<T: 'static>(&self, _data: &T) -> bool {
//         true
//     }
//     #[inline]
//     fn validate_value_type(&self, typ: impl prelude::Type) -> bool {
//         let _ = typ;
//         true
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    // use debug::DebugSchema;

    #[test]
    fn test_only_links() {
        let _schema = ONLY_LINKS;

        // dbg!(DebugSchema::<DataMarker>::new(schema, 6));

        // assert!(schema.links().len() == 1);

        // assert!(false);
    }

    #[test]
    fn test_only_values() {
        let _schema = ONLY_VALUES;
        // assert!(schema.links().len() == 0);
    }
}

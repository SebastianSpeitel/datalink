use core::any::TypeId;
use core::convert::Infallible;
use core::marker::PhantomData;
use core::ops::ControlFlow;

use crate::util::{IntoTrait, Snek, SnekFn};

use super::{prelude::*, And, DataMarker, LinkMarker, Not, Or, TypeMarker, ValueMarker};

pub trait SchemaExt: Sized {
    #[inline]
    fn and<S>(self, other: S) -> And<Self, S> {
        And(self, other)
    }
    #[inline]
    fn or<S>(self, other: S) -> Or<Self, S> {
        Or(self, other)
    }
    #[inline]
    fn invert(self) -> Not<Self> {
        Not(self)
    }
    #[inline]
    fn has_links<L>(self, links: L) -> HasLinks<L>
    where
        L: Snek<LinkMarker>,
    {
        HasLinks(links)
    }
    #[inline]
    fn has_value<T>(self, value: T) -> HasValue<T>
    where
        T: ValueValidator,
    {
        HasValue(value)
    }
    #[inline]
    fn has_type<T>(self, typ: T) -> HasType<T>
    where
        T: TypeValidator,
    {
        HasType(typ)
    }
    #[inline]
    fn equals<T>(self, value: T) -> Equals<T>
    where
        T: crate::Data + Send + Sync + Clone + 'static,
    {
        Equals(value)
    }
    #[inline]
    fn has_target<T>(self, target: T) -> HasTarget<T>
    where
        T: DataValidator,
    {
        HasTarget(target)
    }
    #[inline]
    fn has_key<T>(self, key: T) -> HasKey<T>
    where
        T: DataValidator,
    {
        HasKey(key)
    }
}

impl<S: super::Schema> SchemaExt for S {}

// impl<S: DataValidator> SchemaExt<DataMarker> for S {}
// impl<S: ValueValidator> SchemaExt<ValueMarker> for S {}
// impl<S: TypeValidator> SchemaExt<TypeMarker> for S {}
// impl<S: LinkValidator> SchemaExt<LinkMarker> for S {}

#[derive(Debug, Default, Clone, Copy)]
pub struct HasValue<T>(T);
impl<T> HasValue<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

// impl<T: ValueValidator> DataSchema for HasValue<T> {
//     #[inline]
//     fn value(&self) -> &impl ValueValidator {
//         &self.0
//     }
//     #[inline]
//     fn links(&self) -> Option<&impl LinkValidators> {
//         None::<&Infallible>
//     }
//     #[inline]
//     fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
//         (self.0, None::<Infallible>)
//     }
// }
// impl<T: ValueValidator> DataValidator for HasValue<T> {
//     #[inline]
//     fn validate_value<_T: 'static>(&self, data: &_T) -> bool {
//         self.0.validate(data)
//     }
//     #[inline]
//     fn validate_value_type(&self, typ: impl Type) -> bool {
//         self.0.validate_type(typ)
//     }
//     #[inline]
//     fn validate_link<L: crate::Link>(&self, _link: &L) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_link_type<L: crate::Link>(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct HasLinks<T>(T);
impl<T> HasLinks<T> {
    #[inline]
    pub const fn new(links: T) -> Self {
        Self(links)
    }
}
// impl<T: Snek<LinkMarker>> DataSchema for HasLinks<T> {
//     #[inline]
//     fn value(&self) -> &impl ValueValidator {
//         &FALSE
//     }
//     #[inline]
//     fn links(&self) -> Option<&impl LinkValidators> {
//         Some(&self.0)
//     }
//     #[inline]
//     fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
//         (FALSE, Some(self.0))
//     }
// }
// impl<T: Snek<LinkMarker> + Clone> DataValidator for HasLinks<T> {
//     #[inline]
//     fn validate_link<L: crate::Link>(&self, _link: &L) -> bool {
//         self.validate_link_type::<L>()
//     }
//     #[inline]
//     fn validate_link_type<L: crate::Link>(&self) -> bool {
//         struct Matches<L>(PhantomData<L>);
//         impl<L: crate::Link> SnekFn<LinkMarker> for Matches<L> {
//             type Output = ControlFlow<bool>;
//             #[inline]
//             fn call(&mut self, item: impl IntoTrait<LinkMarker>) -> Self::Output {
//                 let schema = item.into_link_schema();
//                 if schema.validate_type::<L>() {
//                     ControlFlow::Break(true)
//                 } else {
//                     ControlFlow::Continue(())
//                 }
//             }
//         }
//         if self.0.is_empty() {
//             return false;
//         }
//         // Todo: prevent this clone
//         self.0
//             .clone()
//             .try_for_each(Matches::<L>(PhantomData))
//             .break_value()
//             .unwrap_or_default()
//     }
//     #[inline]
//     fn validate_value<Ty: 'static>(&self, _data: &Ty) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_value_type(&self, typ: impl Type) -> bool {
//         let _ = typ;
//         false
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct HasType<T>(T);
impl<T> HasType<T> {
    #[inline]
    pub const fn new(typ: T) -> Self {
        Self(typ)
    }
}
impl<T: crate::r#type::TypeSet> super::Schema for HasType<T> {
    #[inline]
    fn accepted_value_types(&self) -> impl crate::r#type::TypeSet {
        self.0
    }
    #[inline]
    fn accepted_link_types(
        &self,
    ) -> impl IntoIterator<Item = (impl crate::r#type::TypeSet, impl crate::r#type::TypeSet)> {
        [((), ())]
    }
}

// impl<T: TypeValidator> ValueSchema for HasType<T> {
//     #[inline]
//     fn r#type(&self) -> &impl TypeValidator {
//         &self.0
//     }
//     #[inline]
//     fn constant(&self) -> Option<&(impl crate::Data + Send + Sync + '_ + ?Sized)> {
//         None::<&Infallible>
//     }
//     #[inline]
//     fn into_parts(
//         self,
//     ) -> (
//         impl TypeValidator,
//         Option<impl crate::Data + Send + Sync + 'static>,
//     ) {
//         (self.0, None::<Infallible>)
//     }
// }
// impl<T: TypeValidator> ValueValidator for HasType<T> {
//     #[inline]
//     fn validate<Ty: 'static>(&self, _value: &Ty) -> bool {
//         self.0.validate::<Ty>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         self.0.validate_type(typ)
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<
//         impl ValueValidators,
//         impl ValueValidators,
//         impl ValueValidator,
//         impl ValueSchema,
//     > {
//         Composition::simple(self)
//     }
// }
// impl<T: TypeValidator> DataSchema for HasType<T> {
//     #[inline]
//     fn value(&self) -> &impl ValueValidator {
//         self
//     }
//     #[inline]
//     fn links(&self) -> Option<&impl LinkValidators> {
//         None::<&Infallible>
//     }
//     #[inline]
//     fn into_parts(self) -> (impl ValueValidator, Option<impl LinkValidators>) {
//         (self, None::<Infallible>)
//     }
// }
// impl<T: TypeValidator> DataValidator for HasType<T> {
//     #[inline]
//     fn validate_value<Ty: 'static>(&self, _data: &Ty) -> bool {
//         self.0.validate::<Ty>()
//     }
//     #[inline]
//     fn validate_value_type(&self, typ: impl Type) -> bool {
//         self.0.validate_type(typ)
//     }
//     #[inline]
//     fn validate_link<L: crate::Link>(&self, _link: &L) -> bool {
//         false
//     }
//     #[inline]
//     fn validate_link_type<L: crate::Link>(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl DataValidators, impl DataValidators, impl DataValidator, impl DataSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct Equals<T>(T);
// impl<T: 'static> TypeSchema for Equals<T> {
//     #[inline]
//     fn id(&self) -> Option<TypeId> {
//         Some(TypeId::of::<T>())
//     }
//     #[inline]
//     fn name(&self) -> Option<&str> {
//         Some(core::any::type_name::<T>())
//     }
//     #[inline]
//     fn into_parts(self) -> (Option<TypeId>, Option<&'static str>) {
//         (Some(TypeId::of::<T>()), Some(core::any::type_name::<T>()))
//     }
// }
// impl<T: 'static> TypeValidator for Equals<T> {
//     #[inline]
//     fn validate<Ty: 'static>(&self) -> bool {
//         TypeId::of::<Ty>() == TypeId::of::<T>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         typ.is::<T>()
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl TypeValidators, impl TypeValidators, impl TypeValidator, impl TypeSchema>
//     {
//         Composition::simple(self)
//     }
// }
// impl<T: crate::Data + Send + Sync + Clone + 'static> ValueSchema for Equals<T> {
//     #[inline]
//     fn r#type(&self) -> &impl TypeValidator {
//         self
//     }
//     #[inline]
//     fn constant(&self) -> Option<&(impl crate::Data + Send + Sync + '_ + ?Sized)> {
//         Some(&self.0)
//     }
//     #[inline]
//     fn into_parts(
//         self,
//     ) -> (
//         impl TypeValidator,
//         Option<impl crate::Data + Send + Sync + 'static>,
//     ) {
//         (TypeOf::<T>::new(), Some(self.0))
//     }
// }
// impl<T: crate::Data + Send + Sync + Clone + 'static> ValueValidator for Equals<T> {
//     #[inline]
//     fn validate<T2: 'static>(&self, _value: &T2) -> bool {
//         TypeId::of::<T>() == TypeId::of::<T2>()
//     }
//     #[inline]
//     fn validate_type(&self, typ: impl Type) -> bool {
//         typ.is::<T>()
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<
//         impl ValueValidators,
//         impl ValueValidators,
//         impl ValueValidator,
//         impl ValueSchema,
//     > {
//         Composition::simple(self)
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct HasTarget<T>(T);
impl<T> HasTarget<T> {
    #[inline]
    pub const fn new(target: T) -> Self {
        Self(target)
    }
}
// impl<T: DataValidator> LinkSchema for HasTarget<T> {
//     #[inline]
//     fn key(&self) -> &impl DataValidator {
//         &TRUE
//     }
//     #[inline]
//     fn target(&self) -> &impl DataValidator {
//         &self.0
//     }
//     #[inline]
//     fn optional(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn into_parts(self) -> (impl DataValidator, impl DataValidator, bool) {
//         (TRUE, self.0, false)
//     }
// }
// impl<T: DataValidator> LinkValidator for HasTarget<T> {
//     #[inline]
//     fn validate<L: crate::Link>(&self, _link: &L) -> bool {
//         self.0.validate_value_type(L::Target::default())
//     }
//     #[inline]
//     fn validate_type<L: crate::Link>(&self) -> bool {
//         self.0.validate_value_type(L::Target::default())
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
//     {
//         Composition::simple(self)
//     }
// }
#[derive(Debug, Default, Clone, Copy)]
pub struct HasKey<T>(T);
impl<T> HasKey<T> {
    #[inline]
    pub const fn new(key: T) -> Self {
        Self(key)
    }
}
// impl<T: DataValidator> LinkSchema for HasKey<T> {
//     #[inline]
//     fn key(&self) -> &impl DataValidator {
//         &self.0
//     }
//     #[inline]
//     fn target(&self) -> &impl DataValidator {
//         &TRUE
//     }
//     #[inline]
//     fn optional(&self) -> bool {
//         false
//     }
//     #[inline]
//     fn into_parts(self) -> (impl DataValidator, impl DataValidator, bool) {
//         (self.0, TRUE, false)
//     }
// }
// impl<T: DataValidator> LinkValidator for HasKey<T> {
//     #[inline]
//     fn validate<L: crate::Link>(&self, _link: &L) -> bool {
//         self.0.validate_value_type(L::Key::default())
//     }
//     #[inline]
//     fn validate_type<L: crate::Link>(&self) -> bool {
//         self.0.validate_value_type(L::Key::default())
//     }
//     #[inline]
//     fn decompose(
//         self,
//     ) -> Composition<impl LinkValidators, impl LinkValidators, impl LinkValidator, impl LinkSchema>
//     {
//         Composition::simple(self)
//     }
// }

#[cfg(test)]
mod tests {
    use core::any::type_name_of_val;

    use super::*;
    use crate::{
        // schema::debug::DebugSchema,
        r#type::{TypeOf, STRING_LIKE},
    };

    // fn make_complex() -> Builder<impl DataSchema, DataMarker> {
    //     Builder::any()
    //         .has_type::<i32>()
    //         .or(Builder::any().has_type::<u32>().build())
    //         .has_key(TypeSchema(types::STRING_LIKE))
    //         .has_target(Builder::any().has_type::<i32>().build())
    //         .or(Builder::any().has_type::<String>().build())
    // }

    // #[test]
    // // #[ignore]
    // /// This test is ignored because it takes a long time to run.
    // fn very_complex() {
    //     let schema = make_complex()
    //         .has_key(make_complex().build())
    //         .has_target(Builder::any().has_target(make_complex().build()).build())
    //         .build();

    //     let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     // let schema = Builder::from(schema.clone()).has_key(schema).build();

    //     let types = schema.r#type();
    //     let iter = types.try_iter();

    //     dbg!(type_name_of_val(&iter));
    //     dbg!(size_of_val(&iter));

    //     if let Ok(iter) = iter {
    //         for ty in iter {
    //             dbg!(ty);
    //         }
    //     }

    //     dbg!(type_name_of_val(&schema));
    //     assert_eq!(size_of_val(&schema), 0);

    //     dbg!(schema.debug(6));

    //     assert!(false);
    // }

    fn make_complex() -> impl DataValidator + core::fmt::Debug {
        // Builder::value(Builder::r#type().build())
        //     .or(Builder::value(Builder::r#type(TypeOf::<u32>::new()).build()).build())
        //     .and(
        //         Builder::links((Builder::key(
        //             Builder::value(Builder::r#type(STRING_LIKE).build()).build(),
        //         )
        //         .with_target(Builder::value(Builder::r#type(TypeOf::<i32>::new()).build()).build())
        //         .build(),))
        //         .build(),
        //     )
        //     .or(Builder::value(Builder::r#type(TypeOf::<String>::new()).build()).build())

        HasValue::new(HasType::new(TypeOf::<i32>::new().or(TypeOf::<u32>::new()))).and(
            HasLinks::new((HasKey::new(HasValue::new(HasType::new(STRING_LIKE))).and(
                HasTarget::new(HasValue::new(HasType::new(TypeOf::<i32>::new()))),
            ),)),
        )
    }

    #[test]
    fn very_complex() {
        let schema = make_complex();

        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);
        let schema = make_complex().and(schema);

        dbg!(type_name_of_val(&schema));
        assert_eq!(size_of_val(&schema), 0);
        dbg!(&schema);

        // dbg!(super::super::repr::DataSchemaRepr::new(schema));

        // dbg!(DebugSchema::<DataMarker>::new(schema, 5));

        // assert!(false);
    }
}

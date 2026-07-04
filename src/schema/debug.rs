use core::cell::Cell;
use core::fmt::{self, Debug};
use core::marker::PhantomData;

use crate::util::{IntoTrait, Snek, SnekFn};

use super::{prelude::*, DataMarker, LinkMarker, TypeMarker, ValueMarker};

#[derive(Clone, Copy)]
pub struct DebugSchema<M = DataMarker, S = ()> {
    schema: S,
    depth: u8,
    marker: PhantomData<M>,
}

struct DebugSchemaOnce<M = DataMarker, S = ()> {
    schema: Cell<Option<S>>,
    depth: u8,
    marker: PhantomData<M>,
}

impl DebugSchema<DataMarker> {
    #[inline]
    pub fn new(
        schema: impl IntoTrait<DataMarker>,
        depth: u8,
    ) -> DebugSchema<DataMarker, impl DataValidator> {
        DebugSchema {
            schema: schema.into_data_schema(),
            depth,
            marker: PhantomData,
        }
    }
}
impl DebugSchema<ValueMarker> {
    #[inline]
    pub fn new(
        schema: impl IntoTrait<ValueMarker>,
        depth: u8,
    ) -> DebugSchema<ValueMarker, impl ValueValidator> {
        DebugSchema {
            schema: schema.into_value_schema(),
            depth,
            marker: PhantomData,
        }
    }
}
impl DebugSchema<TypeMarker> {
    #[inline]
    pub fn new(
        schema: impl IntoTrait<TypeMarker>,
        depth: u8,
    ) -> DebugSchema<TypeMarker, impl TypeValidator> {
        DebugSchema {
            schema: schema.into_type_schema(),
            depth,
            marker: PhantomData,
        }
    }
}
impl DebugSchema<LinkMarker> {
    #[inline]
    pub fn new(
        schema: impl IntoTrait<LinkMarker>,
        depth: u8,
    ) -> DebugSchema<LinkMarker, impl LinkValidator> {
        DebugSchema {
            schema: schema.into_link_schema(),
            depth,
            marker: PhantomData,
        }
    }
}

impl<S: DataValidator> Debug for DebugSchema<DataMarker, S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Some(depth) = self.depth.checked_sub(1) else {
            return f.debug_struct("DataSchema").finish_non_exhaustive();
        };

        match self.schema.clone().decompose() {
            Composed::Intersection(i) => {
                if i.is_empty() {
                    return f.debug_struct("TRUE").finish();
                }
                let mut tup = f.debug_tuple("Intersection");
                i.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Not(n) => {
                f.write_str("!")?;
                let n = DebugSchema::<DataMarker>::new(n, depth);
                Debug::fmt(&n, f)
            }
            Composed::Union(u) => {
                if u.is_empty() {
                    return f.debug_struct("FALSE").finish();
                }
                let mut tup = f.debug_tuple("Union");
                u.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Simple(s) => {
                let mut st = f.debug_struct("DataSchema");
                let value = DebugSchema::<ValueMarker>::new(s.value(), depth);
                st.field("value", &value as &dyn Debug);
                let links = s.links();
                st.field("links", &DebugSnek::new(links, depth));
                st.finish()
            }
        }
    }
}

impl<S: ValueValidator> Debug for DebugSchema<ValueMarker, S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::data::{ext::DataExt, format::DEBUG};

        let Some(depth) = self.depth.checked_sub(1) else {
            return f.debug_struct("DataSchema").finish_non_exhaustive();
        };

        match self.schema.clone().decompose() {
            Composed::Intersection(i) => {
                if i.is_empty() {
                    return f.debug_struct("TRUE").finish();
                }
                let mut tup = f.debug_tuple("Intersection");
                i.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Not(n) => {
                f.write_str("!")?;
                let n = DebugSchema::<ValueMarker>::new(n, depth);
                Debug::fmt(&n, f)
            }
            Composed::Union(u) => {
                if u.is_empty() {
                    return f.debug_struct("FALSE").finish();
                }
                let mut tup = f.debug_tuple("Union");
                u.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Simple(s) => {
                let mut st = f.debug_struct("ValueSchema");
                let r#type = DebugSchema::<TypeMarker>::new(s.r#type(), depth);
                st.field("type", &r#type as &dyn Debug);
                if let Some(constant) = s.constant() {
                    st.field("constant", &constant.format::<DEBUG>());
                }
                st.finish()
            }
        }
    }
}
impl<S: TypeValidator> Debug for DebugSchema<TypeMarker, S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Some(depth) = self.depth.checked_sub(1) else {
            return f.debug_struct("TypeSchema").finish_non_exhaustive();
        };

        match self.schema.clone().decompose() {
            Composed::Intersection(i) => {
                if i.is_empty() {
                    return f.debug_struct("TRUE").finish();
                }
                let mut tup = f.debug_tuple("Intersection");
                i.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Not(n) => {
                f.write_str("!")?;
                let n = DebugSchema::<TypeMarker>::new(n, depth);
                Debug::fmt(&n, f)
            }
            Composed::Union(u) => {
                if u.is_empty() {
                    return f.debug_struct("FALSE").finish();
                }
                let mut tup = f.debug_tuple("Union");
                u.for_each(DebugTuple(&mut tup, depth));
                tup.finish()
            }
            Composed::Simple(s) => {
                let mut st = f.debug_struct("TypeSchema");
                st.field("id", &s.id());
                st.field("name", &s.name());
                st.finish()
            }
        }
    }
}
impl<S: LinkValidator + Clone> Debug for DebugSchema<LinkMarker, S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Some(depth) = self.depth.checked_sub(1) else {
            return f.debug_struct("LinkSchema").finish_non_exhaustive();
        };

        fmt_link_schema(f, self.schema.clone(), depth)
    }
}
impl<S: LinkValidator> Debug for DebugSchemaOnce<LinkMarker, S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Some(depth) = self.depth.checked_sub(1) else {
            return f.debug_struct("LinkSchema").finish_non_exhaustive();
        };

        let schema = self.schema.take().ok_or(fmt::Error)?;

        fmt_link_schema(f, schema, depth)
    }
}

fn fmt_link_schema(f: &mut fmt::Formatter, schema: impl LinkValidator, depth: u8) -> fmt::Result {
    match schema.decompose() {
        Composed::Intersection(i) => {
            if i.is_empty() {
                return f.debug_struct("TRUE").finish();
            }
            let mut tup = f.debug_tuple("Intersection");
            i.for_each(DebugTuple(&mut tup, depth));
            tup.finish()
        }
        Composed::Not(n) => {
            f.write_str("!")?;
            fmt_link_schema(f, n.into_link_schema(), depth)
        }
        Composed::Union(u) => {
            if u.is_empty() {
                return f.debug_struct("FALSE").finish();
            }
            let mut tup = f.debug_tuple("Union");
            u.for_each(DebugTuple(&mut tup, depth));
            tup.finish()
        }
        Composed::Simple(s) => {
            let mut st = f.debug_struct("LinkSchema");
            let key = DebugSchema::<DataMarker>::new(s.key(), depth);
            st.field("key", &key as &dyn Debug);
            let target = DebugSchema::<DataMarker>::new(s.target(), depth);
            st.field("target", &target as &dyn Debug);
            st.field("optional", &s.optional());
            st.finish()
        }
    }
}

struct DebugSnek<T>(core::cell::Cell<Option<T>>, u8);
impl<T> DebugSnek<T> {
    #[inline]
    const fn new(snek: T, depth: u8) -> Self {
        DebugSnek(core::cell::Cell::new(Some(snek)), depth)
    }
}
impl<T> Debug for DebugSnek<T>
where
    T: Snek<LinkMarker>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();
        if let Some(snek) = self.0.take() {
            snek.for_each(DebugList(&mut list, self.1));
        } else {
            return list.finish_non_exhaustive();
        }
        list.finish()
    }
}

struct DebugList<'a, 'b, 'c>(&'a mut fmt::DebugList<'b, 'c>, u8);
impl SnekFn<LinkMarker> for DebugList<'_, '_, '_> {
    type Output = ();
    fn call(&mut self, item: impl IntoTrait<LinkMarker>) -> Self::Output {
        let schema = DebugSchemaOnce::<LinkMarker, _> {
            schema: Cell::new(Some(item.into_link_schema())),
            depth: self.1,
            marker: PhantomData,
        };
        self.0.entry(&schema as &_);
    }
}

struct DebugTuple<'a, 'b, 'c>(&'a mut fmt::DebugTuple<'b, 'c>, u8);
impl SnekFn<DataMarker> for DebugTuple<'_, '_, '_> {
    type Output = ();
    fn call(&mut self, item: impl IntoTrait<DataMarker>) -> Self::Output {
        let schema = DebugSchema::<DataMarker>::new(item, self.1);
        self.0.field(&schema as &_);
    }
}
impl SnekFn<ValueMarker> for DebugTuple<'_, '_, '_> {
    type Output = ();
    fn call(&mut self, item: impl IntoTrait<ValueMarker>) -> Self::Output {
        let schema = DebugSchema::<ValueMarker>::new(item, self.1);
        self.0.field(&schema as &_);
    }
}
impl SnekFn<TypeMarker> for DebugTuple<'_, '_, '_> {
    type Output = ();
    fn call(&mut self, item: impl IntoTrait<TypeMarker>) -> Self::Output {
        let schema = DebugSchema::<TypeMarker>::new(item, self.1);
        self.0.field(&schema as &dyn Debug);
    }
}
impl SnekFn<LinkMarker> for DebugTuple<'_, '_, '_> {
    type Output = ();
    fn call(&mut self, item: impl IntoTrait<LinkMarker>) -> Self::Output {
        let schema = DebugSchemaOnce::<LinkMarker, _> {
            schema: Cell::new(Some(item.into_link_schema())),
            depth: self.1,
            marker: PhantomData,
        };
        self.0.field(&schema as &dyn Debug);
    }
}

struct DebugOnce<T>(Cell<Option<T>>);
impl<T: Debug> Debug for DebugOnce<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.take().ok_or(fmt::Error)?.fmt(f)
    }
}

// struct CloneOnce<T>(Cell<Option<T>>);
// impl<T> CloneOnce<T> {
//     #[inline]
//     pub const fn new(value: T) -> Self {
//         Self(Cell::new(Some(value)))
//     }
// }
// impl<T> Clone for CloneOnce<T> {
//     #[inline]
//     fn clone(&self) -> Self {
//         Self(Cell::new(self.0.take()))
//     }
// }

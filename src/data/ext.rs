#[cfg(feature = "std")]
use crate::data::BoxedData;

use crate::data::{format, Data, Provided};
use crate::links::{LinkError, Links, Result};
use crate::query::Query;
use crate::rr::{Receiver, Request};

pub trait DataExt: Data {
    #[inline]
    #[must_use]
    fn as_<T>(&self) -> Option<T>
    where
        Self: Sized,
        Option<T>: Receiver + 'static,
    {
        let mut request = Request::<Option<T>>::default();

        if !self.provide_requested(&mut request).was_provided() {
            self.provide_value(&mut request.as_erased());
        }

        request.take()
    }

    #[inline]
    #[must_use]
    fn as_bool(&self) -> Option<bool>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u8(&self) -> Option<u8>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i8(&self) -> Option<i8>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u16(&self) -> Option<u16>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i16(&self) -> Option<i16>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u32(&self) -> Option<u32>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i32(&self) -> Option<i32>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u64(&self) -> Option<u64>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i64(&self) -> Option<i64>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u128(&self) -> Option<u128>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i128(&self) -> Option<i128>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_f32(&self) -> Option<f32>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_f64(&self) -> Option<f64>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_str(&self) -> Option<String>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_bytes(&self) -> Option<Vec<u8>>
    where
        Self: Sized,
    {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_number(&self) -> Option<isize>
    where
        Self: Sized,
    {
        #[derive(Debug, Default)]
        struct Number(Option<isize>);

        impl Number {
            #[inline]
            fn set(&mut self, val: impl TryInto<isize>) {
                if let Ok(val) = val.try_into() {
                    self.0.replace(val);
                }
            }
        }

        impl crate::rr::Receiver for Number {
            #[inline]
            fn bool(&mut self, value: bool) {
                self.set(value);
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)]
            fn f32(&mut self, value: f32) {
                self.set(value as isize);
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)]
            fn f64(&mut self, value: f64) {
                self.set(value as isize);
            }
            #[inline]
            fn i128(&mut self, value: i128) {
                self.set(value);
            }
            #[inline]
            fn i16(&mut self, value: i16) {
                self.set(value);
            }
            #[inline]
            fn i32(&mut self, value: i32) {
                self.set(value);
            }
            #[inline]
            fn i64(&mut self, value: i64) {
                self.set(value);
            }
            #[inline]
            fn i8(&mut self, value: i8) {
                self.set(value);
            }
            #[inline]
            fn u16(&mut self, value: u16) {
                self.set(value);
            }
            #[inline]
            fn u32(&mut self, value: u32) {
                self.set(value);
            }
            #[inline]
            fn u64(&mut self, value: u64) {
                self.set(value);
            }
            #[inline]
            fn u8(&mut self, value: u8) {
                self.set(value);
            }
            #[inline]
            fn u128(&mut self, value: u128) {
                self.set(value);
            }
            #[inline]
            fn char(&mut self, value: char) {
                if let Some(val) = value.to_digit(10) {
                    self.set(val);
                }
            }
            #[inline]
            fn str(&mut self, value: &str) {
                if let Ok(val) = value.parse::<isize>() {
                    self.set(val);
                }
            }
            #[inline]
            fn str_owned(&mut self, value: String) {
                if let Ok(val) = value.parse::<isize>() {
                    self.set(val);
                }
            }
            #[inline]
            fn accepting() -> impl crate::rr::typeset::TypeSet + 'static
            where
                Self: Sized,
            {
                crate::rr::typeset::AnyOf::<(
                    bool,
                    f32,
                    f64,
                    i128,
                    i16,
                    i32,
                    i64,
                    i8,
                    u16,
                    u32,
                    u64,
                    u8,
                    u128,
                    char,
                    &str,
                    String,
                )>::default()
            }
        }

        let mut request = Request::<Number>::default();
        if !self.provide_requested(&mut request).was_provided() {
            self.provide_value(&mut request.as_erased());
        }

        request.take().0
    }

    #[inline]
    fn query<L: Links + Default>(&self, query: &Query) -> Result<L, LinkError> {
        let mut links = L::default();

        self.query_links(&mut links, query)?;

        Ok(links)
    }

    /// Collects all links without a key into a vec.
    ///
    /// Note:
    /// There is no guarantee that the order of the links is preserved.
    ///
    /// ```rust
    /// use datalink::prelude::*;
    /// use datalink::data::DataExt;
    ///
    /// let v = vec![1i32];
    ///
    /// let list = DataExt::as_list(&v).unwrap();
    /// assert_eq!(list.len(), 1);
    /// let item = &list[0];
    /// assert_eq!(DataExt::as_i32(item), Some(1));
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    fn as_list(&self) -> Result<Vec<BoxedData>, LinkError> {
        use crate::links::CONTINUE;

        #[derive(Default)]
        struct ListLinks(Vec<BoxedData>);

        impl Links for ListLinks {
            #[inline]
            fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
                if key.is_none() {
                    self.0.push(target);
                }
                CONTINUE
            }
            #[inline]
            fn push_unkeyed(&mut self, target: BoxedData) -> Result {
                self.0.push(target);
                CONTINUE
            }
            #[inline]
            fn push_keyed(&mut self, _target: BoxedData, _key: BoxedData) -> Result {
                CONTINUE
            }
        }

        let mut links = ListLinks::default();
        self.provide_links(&mut links)?;

        Ok(links.0)
    }

    /// Collects all links with a key into a vec.
    ///
    /// Note:
    /// There is no guarantee that the order of the links is preserved.
    ///
    /// ```rust
    /// use datalink::prelude::*;
    /// use datalink::data::DataExt;
    ///
    /// let mut m = std::collections::HashMap::new();
    /// m.insert("Hello", "world!");
    ///
    /// let items = DataExt::as_items(&m).unwrap();
    /// assert_eq!(items.len(), 1);
    /// let (key, value) = &items[0];
    /// assert_eq!(DataExt::as_str(key), Some("Hello".into()));
    /// assert_eq!(DataExt::as_str(value), Some("world!".into()));
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    fn as_items(&self) -> Result<Vec<(BoxedData, BoxedData)>, LinkError> {
        self.collect_links()
    }

    #[inline]
    fn collect_links<L: Links + Default>(&self) -> Result<L, LinkError> {
        let mut links = L::default();
        self.provide_links(&mut links)?;
        Ok(links)
    }

    #[cfg(feature = "well_known")]
    #[inline]
    fn tags(&self) -> Result<Vec<BoxedData>, LinkError> {
        use crate::{query::prelude::*, well_known::WellKnown};
        const TAG_QUERY: Query = {
            use crate::well_known::TagType;
            Query::new(Link::Key(Data::Id(TagType::ID)))
        };
        self.query(&TAG_QUERY)
    }

    #[cfg(all(feature = "well_known", feature = "unique"))]
    #[inline]
    fn is_tagged_with(&self, tag: &impl crate::data::unique::Unique) -> Result<bool, LinkError> {
        use crate::links::impls::Linked;
        let query = {
            use crate::query::prelude::*;
            use crate::well_known::{TagType, WellKnown};
            Query::new(Link::Key(Data::Id(TagType::ID)) & Link::target(Data::eq(tag)))
                .with_limit(1)
                .build()
        };

        Ok(self.query::<Linked>(&query)? == Linked::Yes)
    }

    #[inline]
    #[must_use]
    fn all_values(&self) -> crate::value::AllValues
    where
        Self: Sized,
    {
        use crate::value::AllValues;
        let mut request = Request::<AllValues>::default();
        if !self.provide_requested(&mut request).was_provided() {
            self.provide_value(&mut request.as_erased());
        }

        request.take()
    }

    #[inline]
    fn has_links(&self) -> Result<bool, LinkError> {
        use crate::links::impls::Linked;
        const FIRST: Query = Query::new(crate::query::LinkFilter::Any)
            .with_limit(1)
            .build_unoptimized();
        let linked = self.query::<Linked>(&FIRST)?;
        Ok(linked == Linked::Yes)
    }

    #[allow(unused_variables)]
    #[inline]
    #[must_use]
    fn format<F: format::Format>(&self) -> format::FormattableData<F, Self> {
        self.into()
    }
}

impl<T: Data + ?Sized> DataExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_list() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("Hello", "world!");

        assert_eq!(DataExt::as_str(&m), None);

        let as_vec = DataExt::as_list(&m).unwrap();
        assert_eq!(as_vec.len(), 0);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_items() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("Hello", "world!");

        assert_eq!(DataExt::as_str(&m), None);

        let as_items = DataExt::as_items(&m).unwrap();
        assert_eq!(as_items.len(), 1);
        let (key, value) = &as_items[0];
        assert_eq!(DataExt::as_str(key), Some("Hello".into()));
        assert_eq!(DataExt::as_str(value), Some("world!".into()));
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_list() {
        let v = vec!["Hello, world!"];

        let vec = DataExt::as_list(&v).unwrap();

        assert_eq!(vec.len(), 1);
        let item = &vec[0];
        assert_eq!(DataExt::as_str(item), Some("Hello, world!".into()));
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_items() {
        let v = vec!["Hello, world!"];

        let items = DataExt::as_items(&v).unwrap();

        assert_eq!(items.len(), 0);
    }

    #[test]
    #[cfg(feature = "well_known")]
    fn tagged() {
        use crate::{data::constant::Const, links::LinksExt, well_known::TAG};

        const IS_TAGGED: Const<12345> = Const::<12345>::empty();

        struct Tagged;
        impl Data for Tagged {
            fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
                links.push_link((TAG, IS_TAGGED))?;
                Ok(())
            }
        }

        let tagged = Tagged.is_tagged_with(&IS_TAGGED).unwrap();
        assert!(tagged);

        let tags = Tagged.tags().unwrap();
        assert_eq!(tags.len(), 1);
    }
}

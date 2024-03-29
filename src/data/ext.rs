use std::borrow::Cow;

#[cfg(feature = "std")]
use crate::data::BoxedData;

use crate::data::{format, Data};
use crate::links::{LinkError, Links, Result};
use crate::query::Query;

pub trait DataExt: Data {
    #[inline]
    #[must_use]
    fn as_bool(&self) -> Option<bool> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_u8(&self) -> Option<u8> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_i8(&self) -> Option<i8> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_u16(&self) -> Option<u16> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_i16(&self) -> Option<i16> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_u32(&self) -> Option<u32> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_i32(&self) -> Option<i32> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_u64(&self) -> Option<u64> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_i64(&self) -> Option<i64> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_u128(&self) -> Option<u128> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_i128(&self) -> Option<i128> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_f32(&self) -> Option<f32> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_f64(&self) -> Option<f64> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_str(&self) -> Option<Cow<str>> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_bytes(&self) -> Option<Cow<[u8]>> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    fn as_number(&self) -> Option<usize> {
        enum NumberBuilder {
            NotFound,
            Found(usize),
            Invalid,
        }

        impl NumberBuilder {
            fn try_set(&mut self, val: usize) {
                match self {
                    Self::NotFound => *self = Self::Found(val),
                    Self::Found(before) if *before == val => {}
                    Self::Found(_) => *self = Self::Invalid,
                    Self::Invalid => {}
                }
            }
        }

        impl crate::value::ValueBuiler<'_> for NumberBuilder {
            fn bool(&mut self, value: bool) {
                self.try_set(value as usize);
            }
            fn bytes(&mut self, _value: Cow<'_, [u8]>) {
                *self = Self::Invalid;
            }
            fn f32(&mut self, value: f32) {
                self.try_set(value as usize);
            }
            fn f64(&mut self, value: f64) {
                self.try_set(value as usize);
            }
            fn i128(&mut self, value: i128) {
                self.try_set(value as usize);
            }
            fn i16(&mut self, value: i16) {
                self.try_set(value as usize);
            }
            fn i32(&mut self, value: i32) {
                self.try_set(value as usize);
            }
            fn i64(&mut self, value: i64) {
                self.try_set(value as usize);
            }
            fn i8(&mut self, value: i8) {
                self.try_set(value as usize);
            }
            fn u16(&mut self, value: u16) {
                self.try_set(value as usize);
            }
            fn u32(&mut self, value: u32) {
                self.try_set(value as usize);
            }
            fn u64(&mut self, value: u64) {
                self.try_set(value as usize);
            }
            fn u8(&mut self, value: u8) {
                self.try_set(value as usize);
            }
            fn u128(&mut self, value: u128) {
                self.try_set(value as usize);
            }
            fn str(&mut self, value: Cow<'_, str>) {
                if let Ok(val) = value.parse() {
                    self.try_set(val);
                } else {
                    *self = Self::Invalid;
                }
            }
        }

        let mut num = NumberBuilder::NotFound;
        self.provide_value(&mut num);

        match num {
            NumberBuilder::Found(val) => Some(val),
            _ => None,
        }
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
    fn is_tagged_with(&self, tag: &impl crate::data::unique::Unique) -> Result<bool, LinkError> {
        let query = {
            use crate::query::prelude::*;
            use crate::well_known::{TagType, WellKnown};
            Query::new(Link::Key(Data::Id(TagType::ID)) & Link::target(Data::eq(tag))).build()
        };

        Ok(self.query::<Option<BoxedData>>(&query)?.is_some())
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
}

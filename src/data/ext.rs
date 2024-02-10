#[cfg(feature = "std")]
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
    #[cfg(feature = "std")]
    fn as_str(&self) -> Option<Cow<str>> {
        let mut value = None;
        self.provide_value(&mut value);
        value
    }

    #[inline]
    #[must_use]
    #[cfg(feature = "std")]
    fn as_bytes(&self) -> Option<Cow<[u8]>> {
        let mut value = None;
        self.provide_value(&mut value);
        value
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
    #[must_use]
    fn collect_links<L: Links + Default>(&self) -> Result<L, LinkError> {
        let mut links = L::default();
        self.provide_links(&mut links)?;
        Ok(links)
    }

    #[allow(unused_variables)]
    #[inline]
    #[must_use]
    fn format<F: format::DataFormatter>(&self) -> format::FormattableData<F, Self> {
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

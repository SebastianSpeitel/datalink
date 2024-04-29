#[cfg(feature = "std")]
use crate::data::BoxedData;

use crate::data::{format, Data, Provided};
use crate::links::{LinkError, Links, Result};
use crate::query::Query;
use crate::rr::{IgnoreMeta, Receiver, RefOption, Req, Request};

pub trait DataExt: Data {
    #[inline]
    #[must_use]
    fn as_<T>(&self) -> Option<T>
    where
        Self: Sized,
        RefOption<T>: for<'d> Req<Receiver<'d> = &'d mut Option<T>>,
        Option<T>: Receiver,
    {
        let mut value = None;
        let mut request: Request<'_, IgnoreMeta<RefOption<T>>> = Request::new(&mut value);
        if self.provide_requested(&mut request).was_provided() {
            return value;
        }
        let request = Request::new(&mut value as &mut dyn Receiver);
        self.provide_value(request);
        value
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

    // #[inline]
    // #[must_use]
    // fn as_number(&self) -> Option<isize> {
    //     enum NumberBuilder {
    //         NotFound,
    //         Found(isize),
    //         Invalid,
    //     }

    //     impl NumberBuilder {
    //         #[inline]
    //         fn try_set(&mut self, val: impl TryInto<isize>) {
    //             match (&self, val.try_into()) {
    //                 (Self::NotFound, Ok(v)) => *self = Self::Found(v),
    //                 (Self::Found(before), Ok(v)) if *before == v => {}
    //                 _ => *self = Self::Invalid,
    //             }
    //         }
    //     }

    //     impl crate::value::ValueBuiler<'_> for NumberBuilder {
    //         fn bool(&mut self, value: bool) {
    //             self.try_set(value);
    //         }
    //         fn bytes(&mut self, _value: Cow<'_, [u8]>) {
    //             *self = Self::Invalid;
    //         }
    //         #[allow(clippy::cast_possible_truncation)]
    //         fn f32(&mut self, value: f32) {
    //             self.try_set(value as isize);
    //         }
    //         #[allow(clippy::cast_possible_truncation)]
    //         fn f64(&mut self, value: f64) {
    //             self.try_set(value as isize);
    //         }
    //         fn i128(&mut self, value: i128) {
    //             self.try_set(value);
    //         }
    //         fn i16(&mut self, value: i16) {
    //             self.try_set(value);
    //         }
    //         fn i32(&mut self, value: i32) {
    //             self.try_set(value);
    //         }
    //         fn i64(&mut self, value: i64) {
    //             self.try_set(value);
    //         }
    //         fn i8(&mut self, value: i8) {
    //             self.try_set(value);
    //         }
    //         fn u16(&mut self, value: u16) {
    //             self.try_set(value);
    //         }
    //         fn u32(&mut self, value: u32) {
    //             self.try_set(value);
    //         }
    //         fn u64(&mut self, value: u64) {
    //             self.try_set(value);
    //         }
    //         fn u8(&mut self, value: u8) {
    //             self.try_set(value);
    //         }
    //         fn u128(&mut self, value: u128) {
    //             self.try_set(value);
    //         }
    //         fn str(&mut self, value: Cow<'_, str>) {
    //             if let Ok(val) = value.parse::<isize>() {
    //                 self.try_set(val);
    //             } else {
    //                 *self = Self::Invalid;
    //             }
    //         }
    //     }

    //     let mut num = NumberBuilder::NotFound;
    //     self.provide_value(&mut num);

    //     match num {
    //         NumberBuilder::Found(val) => Some(val),
    //         _ => None,
    //     }
    // }

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
        let query = {
            use crate::query::prelude::*;
            use crate::well_known::{TagType, WellKnown};
            Query::new(Link::Key(Data::Id(TagType::ID)) & Link::target(Data::eq(tag))).build()
        };

        Ok(self.query::<Option<BoxedData>>(&query)?.is_some())
    }

    #[inline]
    #[must_use]
    fn all_values(&self) -> crate::value::AllValues
    where
        Self: Sized,
    {
        let mut values = crate::value::AllValues::default();
        if self
            .provide_requested::<crate::value::AllValues>(&mut Request::new(&mut values))
            .was_provided()
        {
            return values;
        }
        let mut values = crate::value::AllValues::default();
        self.provide_value(Request::new(&mut values as &mut dyn Receiver));
        values
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

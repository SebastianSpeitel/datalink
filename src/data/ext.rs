use crate::{Data, ErasedData, Request, Visitor};

pub trait DataExt: Data {
    #[inline]
    #[must_use]
    fn as_<T>(&self) -> Option<T>
    where
        Option<T>: Request,
    {
        let mut request = None;
        self.query(request.by_ref());
        request
    }

    #[inline]
    #[must_use]
    fn as_bool(&self) -> Option<bool> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u8(&self) -> Option<u8> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i8(&self) -> Option<i8> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u16(&self) -> Option<u16> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i16(&self) -> Option<i16> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u32(&self) -> Option<u32> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i32(&self) -> Option<i32> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u64(&self) -> Option<u64> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i64(&self) -> Option<i64> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_u128(&self) -> Option<u128> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_i128(&self) -> Option<i128> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_f32(&self) -> Option<f32> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_f64(&self) -> Option<f64> {
        self.as_()
    }

    /// Tries to convert the data into a string.
    ///
    /// # Example
    /// ```
    /// use datalink::prelude::*;
    /// use datalink::DataExt;
    ///
    /// let s = "Hello, world!";
    /// assert_eq!(DataExt::as_string(&s), Some("Hello, world!".into()));
    ///
    /// let s = Box::new(s) as Box<ErasedData>;
    /// assert_eq!(DataExt::as_string(&s), Some("Hello, world!".into()));
    /// ```
    #[inline]
    #[must_use]
    fn as_string(&self) -> Option<String> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_bytes(&self) -> Option<Vec<u8>> {
        self.as_()
    }

    #[inline]
    #[must_use]
    fn as_number(&self) -> Option<isize> {
        type NumberTypes = crate::r#type::Types<(
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
            &'static str,
            String,
        )>;
        const TYPE_SET: NumberTypes = NumberTypes::new();

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

        impl Visitor for Number {
            #[inline]
            fn visit_bool(&mut self, value: bool) {
                self.set(value);
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)]
            fn visit_f32(&mut self, value: f32) {
                self.set(value as isize);
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)]
            fn visit_f64(&mut self, value: f64) {
                self.set(value as isize);
            }
            #[inline]
            fn visit_i128(&mut self, value: i128) {
                self.set(value);
            }
            #[inline]
            fn visit_i16(&mut self, value: i16) {
                self.set(value);
            }
            #[inline]
            fn visit_i32(&mut self, value: i32) {
                self.set(value);
            }
            #[inline]
            fn visit_i64(&mut self, value: i64) {
                self.set(value);
            }
            #[inline]
            fn visit_i8(&mut self, value: i8) {
                self.set(value);
            }
            #[inline]
            fn visit_u16(&mut self, value: u16) {
                self.set(value);
            }
            #[inline]
            fn visit_u32(&mut self, value: u32) {
                self.set(value);
            }
            #[inline]
            fn visit_u64(&mut self, value: u64) {
                self.set(value);
            }
            #[inline]
            fn visit_u8(&mut self, value: u8) {
                self.set(value);
            }
            #[inline]
            fn visit_u128(&mut self, value: u128) {
                self.set(value);
            }
            #[inline]
            fn visit_char(&mut self, value: char) {
                if let Some(val) = value.to_digit(10) {
                    self.set(val);
                }
            }
            #[inline]
            fn visit_str(&mut self, value: &str) {
                if let Ok(val) = value.parse::<isize>() {
                    self.set(val);
                }
            }
            #[inline]
            fn visit_str_owned(&mut self, value: Box<str>) {
                if let Ok(val) = value.parse::<isize>() {
                    self.set(val);
                }
            }
        }

        impl Request for Number {
            fn schema(&self) -> impl crate::schema::Schema {
                TYPE_SET
            }
            fn visitor(&mut self) -> impl Visitor {
                self
            }
            fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
                self
            }
        }

        impl crate::request::ErasableRequest for Number {
            fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
                crate::schema::SimpleSchema::values(NumberTypes::SET)
            }
            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                Some(self)
            }
        }

        let mut req = Number::default();

        self.query(req.by_ref());

        req.0
    }

    /// Collects all links into a vec discarding possible keys.
    ///
    /// Note:
    /// There is no guarantee that the order of the links is preserved.
    ///
    /// ```rust
    /// use datalink::prelude::*;
    /// use datalink::DataExt;
    ///
    /// let v = vec![1i32];
    ///
    /// let list = DataExt::as_list(&v);
    /// assert_eq!(list.len(), 1);
    /// let item = &list[0];
    /// assert_eq!(DataExt::as_i32(item), Some(1));
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    fn as_list(&self) -> Vec<Box<ErasedData>> {
        use crate::Request;
        #[derive(Default)]
        struct Items {
            items: Vec<Box<ErasedData>>,
        }

        impl Request for Items {
            fn schema(&self) -> impl crate::schema::Schema {
                crate::schema::ONLY_LINKS
            }

            fn visitor(&mut self) -> impl Visitor {}

            fn provide_link_unchecked<L: crate::Link>(&mut self, link: L) {
                self.items.push(Box::new(link.into_target()));
            }

            fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
                self
            }
        }

        impl crate::request::ErasableRequest for Items {
            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                None
            }
            fn erased_link_request(
                &mut self,
            ) -> (
                Option<crate::request::ErasedRequest>,
                Option<crate::request::ErasedRequest>,
            ) {
                (
                    None,
                    Some(crate::request::ErasedRequest::Owned(Box::new(Appender(
                        &mut self.items,
                    )))),
                )
            }
        }

        struct Appender<'a>(&'a mut Vec<Box<ErasedData>>);
        impl crate::request::ErasableRequest for Appender<'_> {
            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                None
            }
            #[inline]
            fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
                crate::schema::OnlyLinks::SCHEMA
            }
            #[inline]
            fn erased_self_receiver(&mut self) -> Option<crate::request::SelfReceiver> {
                Some(Box::new(move |data| {
                    self.0.push(data);
                }))
            }
        }

        let mut req = Items::default();
        self.query(req.by_ref());

        req.items
    }

    /// Collects all links into a vec of key-value pairs.
    ///
    /// Note:
    /// There is no guarantee that the order of the links is preserved.
    ///
    /// ```
    /// use datalink::prelude::*;
    /// use datalink::DataExt;
    ///
    /// let mut m = std::collections::HashMap::new();
    /// m.insert("Hello", "world!");
    ///
    /// let items = DataExt::as_items(&m);
    /// assert_eq!(items.len(), 1);
    /// let (key, target) = &items[0];
    /// assert_eq!(DataExt::as_string(key), Some("Hello".into()));
    /// assert_eq!(DataExt::as_string(target), Some("world!".into()));
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    fn as_items(&self) -> Vec<(Box<ErasedData>, Box<ErasedData>)> {
        use crate::Request;

        #[derive(Debug, Default)]
        struct Items {
            items: Vec<(Box<ErasedData>, Box<ErasedData>)>,
            key: Option<Box<ErasedData>>,
            target: Option<Box<ErasedData>>,
        }

        impl Items {
            fn try_push(&mut self) {
                dbg!("try_push", &self);
                if let (Some(k), Some(t)) = (self.key.take(), self.target.take()) {
                    self.items.push((k, t));
                }
            }
        }

        impl Request for Items {
            fn schema(&self) -> impl crate::schema::Schema {
                crate::schema::ONLY_LINKS
            }

            fn visitor(&mut self) -> impl Visitor {}

            fn provide_link_unchecked<L: crate::Link>(&mut self, link: L) {
                dbg!("provide_link_unchecked", core::any::type_name::<L>());
                let (k, t) = link.into_tuple();
                self.items.push((Box::new(k), Box::new(t)));
            }

            fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
                self
            }
        }

        impl crate::request::ErasableRequest for Items {
            fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
                crate::schema::OnlyLinks::SCHEMA
            }

            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                None
            }
            fn erased_link_request(
                &mut self,
            ) -> (
                Option<crate::request::ErasedRequest>,
                Option<crate::request::ErasedRequest>,
            ) {
                use crate::request::ErasedRequest;
                self.try_push();
                (
                    Some(ErasedRequest::Borrowed(&mut self.key)),
                    Some(ErasedRequest::Borrowed(&mut self.target)),
                )
            }
        }

        let mut req = Items::default();
        self.query(req.by_ref());
        req.try_push();
        req.items
    }

    // #[inline]
    // fn collect_links<L: Links + Default>(&self) -> Result<L, LinkError> {
    //     let mut links = L::default();
    //     self.provide_links(&mut links)?;
    //     Ok(links)
    // }

    // #[cfg(feature = "well_known")]
    // #[inline]
    // fn tags(&self) -> Result<Vec<BoxedData>, LinkError> {
    //     use crate::{query::prelude::*, well_known::WellKnown};
    //     const TAG_QUERY: Query = {
    //         use crate::well_known::TagType;
    //         Query::new(Link::Key(Data::Id(TagType::ID)))
    //     };
    //     self.query(&TAG_QUERY)
    // }

    // #[cfg(all(feature = "well_known", feature = "unique"))]
    // #[inline]
    // fn is_tagged_with(&self, tag: &impl crate::data::unique::Unique) -> Result<bool, LinkError> {
    //     use crate::links::impls::Linked;
    //     let query = {
    //         use crate::query::prelude::*;
    //         use crate::well_known::{TagType, WellKnown};
    //         Query::new(Link::Key(Data::Id(TagType::ID)) & Link::target(Data::eq(tag)))
    //             .with_limit(1)
    //             .build()
    //     };

    //     Ok(self.query::<Linked>(&query)? == Linked::Yes)
    // }

    #[inline]
    #[must_use]
    fn all_values(&self) -> crate::value::AllValues
    where
        Self: Sized,
    {
        use crate::value::AllValues;
        let mut req = AllValues::default();
        self.query(req.by_ref());

        req
    }

    #[inline]
    fn has_links(&self) -> bool {
        struct HasLinks(bool);

        impl Request for HasLinks {
            #[inline]
            fn schema(&self) -> impl crate::schema::Schema {
                crate::schema::ONLY_LINKS
            }
            #[inline]
            fn visitor(&mut self) -> impl Visitor {}
            #[inline]
            fn provide_link_unchecked<L: crate::Link>(&mut self, _link: L) {
                self.0 = true;
            }
            #[inline]
            fn as_erased(&mut self) -> impl crate::request::ErasableRequest {
                self
            }
        }

        impl crate::request::ErasableRequest for HasLinks {
            fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
                crate::schema::OnlyLinks::SCHEMA
            }
            fn erased_visitor(&mut self) -> crate::request::erased::ErasedVisitor {
                None
            }
            fn erased_link_request(
                &mut self,
            ) -> (
                Option<crate::request::ErasedRequest>,
                Option<crate::request::ErasedRequest>,
            ) {
                self.0 = true;
                (None, None)
            }
        }

        let mut req = HasLinks(false);

        self.query(req.by_ref());

        req.0
    }

    #[allow(unused_variables)]
    #[inline]
    #[must_use]
    fn format<F: crate::data::format::Format>(
        &self,
    ) -> crate::data::format::FormattableData<F, Self> {
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

        assert_eq!(DataExt::as_string(&m), None);

        let as_vec = DataExt::as_list(&m);
        dbg!(&as_vec);
        assert_eq!(as_vec.len(), 1);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_items() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("Hello", "world!");

        assert_eq!(DataExt::as_string(&m), None);

        let as_items = DataExt::as_items(&m);

        assert_eq!(as_items.len(), 1);
        let (key, value) = &as_items[0];

        dbg!(&key);

        assert_eq!(DataExt::as_string(key).unwrap(), "Hello");
        assert_eq!(DataExt::as_string(value).unwrap(), "world!");
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_items_dyn() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("Hello", "world!");
        let m = &m as &ErasedData;

        assert_eq!(DataExt::as_string(&m), None);

        let as_items = DataExt::as_items(&m);

        assert_eq!(as_items.len(), 1);
        let (key, value) = &as_items[0];

        dbg!(&key);

        assert_eq!(DataExt::as_string(key).unwrap(), "Hello");
        assert_eq!(DataExt::as_string(value).unwrap(), "world!");
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_list() {
        let v = vec!["Hello, world!"];

        let vec = DataExt::as_list(&v);

        assert_eq!(vec.len(), 1);
        let item = &vec[0];
        assert_eq!(DataExt::as_string(item).unwrap(), "Hello, world!");
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_list_dyn() {
        let v = vec!["Hello, world!"];
        let v = &v as &ErasedData;

        let vec = DataExt::as_list(&v);

        assert_eq!(vec.len(), 1);
        let item = &vec[0];
        assert_eq!(DataExt::as_string(item).unwrap(), "Hello, world!");
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_items() {
        let v = vec!["Hello, world!"];

        let items = DataExt::as_items(&v);

        assert_eq!(items.len(), 1);
    }

    // #[test]
    // #[cfg(feature = "well_known")]
    // fn tagged() {
    //     use crate::{data::constant::Const, links::LinksExt, well_known::TAG};

    //     const IS_TAGGED: Const<12345> = Const::<12345>::empty();

    //     struct Tagged;
    //     impl Data for Tagged {
    //         fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
    //             links.push_link((TAG, IS_TAGGED))?;
    //             Ok(())
    //         }
    //     }

    //     let tagged = Tagged.is_tagged_with(&IS_TAGGED).unwrap();
    //     assert!(tagged);

    //     let tags = Tagged.tags().unwrap();
    //     assert_eq!(tags.len(), 1);
    // }
}

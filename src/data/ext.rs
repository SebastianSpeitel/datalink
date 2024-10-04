use crate::{Data, ErasedData, Query};

pub trait DataExt: Data {
    #[inline]
    #[must_use]
    fn as_<T>(&self) -> Option<T>
    where
        Self: Sized,
        Option<T>: Query,
    {
        let mut query = None;
        self.query(&mut query);
        query
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
    fn as_string(&self) -> Option<String>
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

        impl crate::Receiver for Number {
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
            fn str_owned(&mut self, value: Box<str>) {
                if let Ok(val) = value.parse::<isize>() {
                    self.set(val);
                }
            }
            #[inline]
            fn accepting() -> impl crate::query::TypeFilter + 'static
            where
                Self: Sized,
            {
                crate::filter::AnyOf::<(
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

        impl Query for Number {
            type KeyQuery<'q> = ();
            type TargetQuery<'q> = ();
            type Receiver<'q> = &'q mut Self;
            type Filter<'q> = crate::filter::AnyOf<(
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

            fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
                ((), ())
            }

            fn receiver(&mut self) -> Self::Receiver<'_> {
                self
            }

            fn filter(&self) -> Self::Filter<'_> {
                Default::default()
            }
        }

        let mut query = Number::default();
        self.query(&mut query);

        query.0
    }

    /// Collects all links into a vec discarding possible keys.
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
    /// let list = DataExt::as_list(&v);
    /// assert_eq!(list.len(), 1);
    /// let item = &list[0];
    /// assert_eq!(DataExt::as_i32(item), Some(1));
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    fn as_list(&self) -> Vec<Box<ErasedData>> {
        #[derive(Default)]
        struct Items {
            items: Vec<Box<ErasedData>>,
            next_target: Option<Box<ErasedData>>,
        }

        impl Items {
            fn try_push(&mut self) {
                if let Some(t) = self.next_target.take() {
                    self.items.push(t);
                }
            }
        }

        impl Query for Items {
            type Receiver<'q> = () where Self:'q;
            type KeyQuery<'q> = () where Self:'q;
            type TargetQuery<'q> = &'q mut Option<Box<ErasedData>> where Self:'q;
            type Filter<'q> = crate::filter::AnyOf<()> where Self:'q;

            fn receiver(&mut self) -> Self::Receiver<'_> {}

            fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
                self.try_push();
                (&mut self.next_target, ())
            }

            fn filter(&self) -> Self::Filter<'_> {
                Default::default()
            }
        }

        let mut req = Items::default();
        self.query(&mut req);
        req.try_push();

        dbg!(req.items.len());
        dbg!(req.next_target.is_some());

        req.items
    }

    /// Collects all links *with* a key into a vec.
    ///
    /// Note:
    /// There is no guarantee that the order of the links is preserved.
    ///
    /// ```
    /// use datalink::prelude::*;
    /// use datalink::data::DataExt;
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
        #[derive(Default)]
        struct Items {
            items: Vec<(Box<ErasedData>, Box<ErasedData>)>,
            next_key: Option<Box<ErasedData>>,
            next_target: Option<Box<ErasedData>>,
        }

        impl Items {
            fn try_push(&mut self) {
                if let (Some(k), Some(t)) = (self.next_key.take(), self.next_target.take()) {
                    self.items.push((k, t));
                }
            }
        }

        impl Query for Items {
            type Receiver<'q> = () where Self:'q;
            type KeyQuery<'q> = &'q mut Option<Box<ErasedData>> where Self:'q;
            type TargetQuery<'q> = &'q mut Option<Box<ErasedData>> where Self:'q;
            type Filter<'q> = crate::filter::AnyOf<()> where Self:'q;

            fn receiver(&mut self) -> Self::Receiver<'_> {}

            fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
                self.try_push();
                (&mut self.next_target, &mut self.next_key)
            }

            fn filter(&self) -> Self::Filter<'_> {
                Default::default()
            }
        }

        let mut req = Items::default();
        self.query(&mut req);
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
        let mut query = AllValues::default();
        self.query(&mut query);

        query
    }

    #[inline]
    fn has_links(&self) -> bool {
        struct HasLinks(bool);

        impl Query for HasLinks {
            type Receiver<'q> = ();
            type KeyQuery<'q> = ();
            type TargetQuery<'q> = ();
            type Filter<'q> = crate::filter::AnyOf<()>;

            fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
                self.0 = true;
                ((), ())
            }

            fn receiver(&mut self) -> Self::Receiver<'_> {}

            fn filter(&self) -> Self::Filter<'_> {
                Default::default()
            }
        }

        let mut query = HasLinks(false);

        self.query(&mut query);

        query.0
    }

    #[inline]
    fn ensure_erasablity(self) -> impl Data
    where
        Self: Sized + 'static,
    {
        crate::data::EnsuredErasable(self)
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
        assert_eq!(DataExt::as_string(key), Some("Hello".into()));
        assert_eq!(DataExt::as_string(value), Some("world!".into()));
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_list() {
        let v = vec!["Hello, world!"];

        let vec = DataExt::as_list(&v);

        assert_eq!(vec.len(), 1);
        let item = &vec[0];
        assert_eq!(DataExt::as_string(item), Some("Hello, world!".into()));
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_items() {
        let v = vec!["Hello, world!"];

        let items = DataExt::as_items(&v);

        assert_eq!(items.len(), 0);
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

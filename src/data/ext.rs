use std::borrow::Cow;

use crate::data::{format, BoxedData, Data};
use crate::link_builder::{LinkBuilder, LinkBuilderError as LBE};
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
    fn query(&self, query: &Query) -> Result<Vec<(Option<BoxedData>, BoxedData)>, LBE> {
        #[derive(Default)]
        struct Builder {
            links: Vec<(Option<BoxedData>, BoxedData)>,
            next_key: Option<BoxedData>,
            next_target: Option<BoxedData>,
        }
        impl LinkBuilder for Builder {
            fn set_key(&mut self, key: BoxedData) {
                self.next_key.replace(key);
            }
            fn set_target(&mut self, target: BoxedData) {
                self.next_target.replace(target);
            }
            fn build(&mut self) -> Result<(), LBE> {
                let link = (
                    self.next_key.take(),
                    self.next_target.take().ok_or(LBE::MissingTarget)?,
                );
                self.links.push(link);
                Ok(())
            }
            fn end(&mut self) -> Result<(), LBE> {
                debug_assert!(self.next_key.is_none());
                debug_assert!(self.next_target.is_none());
                Ok(())
            }
        }

        let mut builder = Builder::default();
        self.query_links(&mut builder, query)?;

        Ok(builder.links)
    }

    // #[inline]
    // #[must_use]
    // fn iter_links(&self) -> std::sync::mpsc::Receiver<(Option<BoxedData>, BoxedData)> {
    //     use crate::link_builder::LinkBuilderError as LBE;

    //     struct SyncBuilder {
    //         next_key: Option<BoxedData>,
    //         next_target: Option<BoxedData>,
    //         sender: Option<std::sync::mpsc::SyncSender<(Option<BoxedData>, BoxedData)>>,
    //     }

    //     impl LinkBuilder for SyncBuilder {
    //         fn set_key(&mut self, key: BoxedData) {
    //             self.next_key.replace(key);
    //         }
    //         fn set_target(&mut self, target: BoxedData) {
    //             self.next_target.replace(target);
    //         }
    //         fn build(&mut self) -> Result<(), LBE> {
    //             let key = self.next_key.take();
    //             let target = self.next_target.take();

    //             let target = target.ok_or(LBE::MissingTarget)?;

    //             let sender = self.sender.as_mut().ok_or(LBE::AlreadyEnded)?;

    //             sender
    //                 .send((key, target))
    //                 .map_err(|_| LBE::Other("send failed"))?;

    //             Ok(())
    //         }

    //         fn end(&mut self) -> Result<(), LBE> {
    //             if self.sender.take().is_none() {
    //                 return Err(LBE::AlreadyEnded);
    //             }
    //             Ok(())
    //         }
    //     }

    //     let (sender, receiver) = std::sync::mpsc::sync_channel(0);

    //     let mut builder = SyncBuilder {
    //         next_key: None,
    //         next_target: None,
    //         sender: Some(sender),
    //     };

    //     std::thread::spawn(move || self.provide_links(&mut builder));

    //     receiver
    // }

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
    fn as_list(&self) -> Result<Vec<BoxedData>, LBE> {
        #[derive(Default)]
        struct Builder {
            has_key: bool,
            next: Option<BoxedData>,
            items: Vec<BoxedData>,
        }

        impl LinkBuilder for Builder {
            fn set_key(&mut self, _key: BoxedData) {
                self.has_key = true;
            }
            fn set_target(&mut self, target: BoxedData) {
                self.next.replace(target);
            }
            fn build(&mut self) -> Result<(), LBE> {
                let has_key = std::mem::take(&mut self.has_key);
                let item = self.next.take();

                match (item, has_key) {
                    (Some(item), false) => {
                        self.items.push(item);
                    }
                    (_, true) => {
                        // if link has a key it is not a list item but a map item
                    }
                    (None, _) => {
                        return Err(LBE::MissingTarget);
                    }
                }

                Ok(())
            }
            fn end(&mut self) -> Result<(), LBE> {
                debug_assert!(self.next.is_none());
                Ok(())
            }
        }

        let mut builder = Builder::default();
        self.provide_links(&mut builder)?;

        Ok(builder.items)
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
    fn as_items(&self) -> Result<Vec<(BoxedData, BoxedData)>, LBE> {
        #[derive(Default)]
        struct Builder {
            next_key: Option<BoxedData>,
            next_target: Option<BoxedData>,
            items: Vec<(BoxedData, BoxedData)>,
        }

        impl LinkBuilder for Builder {
            fn set_key(&mut self, key: BoxedData) {
                self.next_key.replace(key);
            }
            fn set_target(&mut self, target: BoxedData) {
                self.next_target.replace(target);
            }
            fn build(&mut self) -> Result<(), LBE> {
                let key = self.next_key.take();
                let target = self.next_target.take();

                match (key, target) {
                    (Some(key), Some(target)) => {
                        self.items.push((key, target));
                    }
                    (_, Some(..)) => {
                        // if link has no key it is not a map item but a list item
                    }
                    (_, None) => {
                        return Err(LBE::MissingTarget);
                    }
                }
                Ok(())
            }
            fn end(&mut self) -> Result<(), LBE> {
                debug_assert!(self.next_key.is_none());
                debug_assert!(self.next_target.is_none());
                Ok(())
            }
        }

        let mut builder = Builder::default();
        self.provide_links(&mut builder)?;

        Ok(builder.items)
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

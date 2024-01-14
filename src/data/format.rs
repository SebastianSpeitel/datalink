use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

use crate::data::{BoxedData, Data};
use crate::link_builder::{LinkBuilder, LinkBuilderError as LBE};
use crate::value::ValueBuiler;

#[derive(Default, Debug)]
pub struct FORMAT<const SERIAL: bool = false, const MAX_DEPTH: u16 = 6, const COMPACT: bool = false>;

pub type COMPACT<const MAX_DEPTH: u16 = 6> = FORMAT<true, MAX_DEPTH, true>;
pub type DEBUG = FORMAT<true, 6, false>;

pub trait DataFormatter {
    type State: Default + Copy;

    fn init_state() -> Self::State {
        Default::default()
    }

    #[inline]
    fn fmt(
        f: &mut fmt::Formatter<'_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        let mut d = Self::create_debug_struct(f, data, state);

        // Format values
        Self::fmt_values(&mut d, data, state);

        // Format links
        Self::fmt_links(&mut d, data, state);

        d.finish()
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_values(
        f: &mut fmt::DebugStruct<'_, '_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        data.provide_value(f);
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_links(
        f: &mut fmt::DebugStruct<'_, '_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        f.field("links", &format_args!("..."));
    }

    #[inline]
    fn fmt_link(
        f: &mut fmt::Formatter<'_>,
        key: Option<&(impl Data + ?Sized)>,
        target: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        if let Some(key) = key {
            Self::fmt(f, key, state)?;
            f.write_str(" -> ")?;
        } else {
            f.write_str("- ")?;
        }
        Self::fmt(f, target, state)?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[inline]
    fn create_debug_struct<'a, 'b>(
        f: &'a mut fmt::Formatter<'b>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::DebugStruct<'a, 'b> {
        #[cfg(feature = "unique")]
        if let Some(id) = data.get_id() {
            return f.debug_struct(&format!("Data[{id}]"));
        }

        f.debug_struct("Data")
    }
}

impl<const SERIAL: bool, const MAX_DEPTH: u16> DataFormatter for FORMAT<SERIAL, MAX_DEPTH, false> {
    type State = u16;

    #[inline]
    fn init_state() -> Self::State {
        MAX_DEPTH
    }

    #[inline]
    fn fmt_links(
        f: &mut fmt::DebugStruct<'_, '_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        if MAX_DEPTH == 0 || state == 0 {
            f.field("links", &format_args!("..."));
            return;
        }

        if SERIAL {
            let links = serial::SerialLinks::<Self, _>::new(data, state.saturating_sub(1));
            if !links.is_empty() {
                f.field("links", &links);
            }
            return;
        }

        let links = recursive::RecursiveLinks::<Self, _>::new(data, state.saturating_sub(1));
        f.field("links", &links);
    }
}

impl<const SERIAL: bool, const MAX_DEPTH: u16> DataFormatter for FORMAT<SERIAL, MAX_DEPTH, true> {
    type State = u16;

    #[inline]
    fn init_state() -> Self::State {
        MAX_DEPTH
    }

    #[inline]
    fn fmt(
        f: &mut fmt::Formatter<'_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        let values = crate::value::Value::from_data(data);
        let links = serial::SerialLinks::<Self, _>::new(data, state.saturating_sub(1));

        if links.is_empty() {
            match values.as_enum() {
                Some(None) => {
                    f.write_str("Data")?;
                    return Ok(());
                }
                Some(Some(v)) => {
                    f.write_fmt(format_args!("Data({v})"))?;
                    return Ok(());
                }
                None => {}
            }
        }

        let mut d = f.debug_struct("Data");

        // Format values
        Self::fmt_values(&mut d, &values, state);

        // Format links
        if links.is_empty() {
            // no links
        } else if MAX_DEPTH == 0 || state == 0 {
            d.field("links", &format_args!("..."));
        } else {
            d.field("links", &links);
        }

        d.finish()
    }

    #[inline]
    fn fmt_links(
        f: &mut fmt::DebugStruct<'_, '_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        if MAX_DEPTH == 0 || state == 0 {
            f.field("links", &format_args!("..."));
            return;
        }

        if SERIAL {
            let links = serial::SerialLinks::<Self, _>::new(data, state - 1);
            if !links.is_empty() {
                f.field("links", &links);
            }
            return;
        }

        let links = recursive::RecursiveLinks::<Self, _>::new(data, state - 1);
        f.field("links", &links);
    }
}

pub struct FormattableData<'d, F: DataFormatter, D: Data + ?Sized> {
    data: &'d D,
    phantom: PhantomData<F>,
}

impl<'d, F: DataFormatter, D: Data + ?Sized> From<&'d D> for FormattableData<'d, F, D> {
    #[inline]
    fn from(data: &'d D) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }
}

impl<F: DataFormatter, D: Data + ?Sized> Display for FormattableData<'_, F, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

impl<F: DataFormatter, D: Data + ?Sized> Debug for FormattableData<'_, F, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

struct FormattableLink<F: DataFormatter, K: Data, T: Data> {
    key: Option<K>,
    target: T,
    state: F::State,
    phantom: PhantomData<F>,
}

impl<F: DataFormatter, K: Data, T: Data> Debug for FormattableLink<F, K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt_link(f, self.key.as_ref(), &self.target, self.state)
    }
}

mod recursive {
    use super::*;

    pub(super) struct RecursiveLinks<'d, F: DataFormatter, D: Data + ?Sized> {
        data: &'d D,
        state: F::State,
        phantom: PhantomData<F>,
    }

    impl<'d, F: DataFormatter, D: Data + ?Sized> RecursiveLinks<'d, F, D> {
        #[inline]
        pub(super) fn new(data: &'d D, state: F::State) -> Self {
            Self {
                data,
                state,
                phantom: PhantomData,
            }
        }
    }

    impl<'d, F: DataFormatter, D: Data + ?Sized> Debug for RecursiveLinks<'d, F, D> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let fmt_set = f.debug_set();
            let mut builder = RecursiveLinkBuilder::<F> {
                fmt_set,
                state: self.state,
                next_key: None,
                next_target: None,
                phantom: PhantomData,
            };
            self.data
                .provide_links(&mut builder)
                .map_err(|_| std::fmt::Error)?;
            Ok(())
        }
    }

    struct RecursiveLinkBuilder<'a, 'b, F: DataFormatter> {
        fmt_set: fmt::DebugSet<'a, 'b>,
        state: F::State,
        next_key: Option<BoxedData>,
        next_target: Option<BoxedData>,
        phantom: PhantomData<F>,
    }

    impl<F: DataFormatter> LinkBuilder for RecursiveLinkBuilder<'_, '_, F> {
        fn set_key(&mut self, key: BoxedData) {
            self.next_key.replace(key);
        }
        fn set_target(&mut self, target: BoxedData) {
            self.next_target.replace(target);
        }
        fn build(&mut self) -> Result<(), LBE> {
            let key = self.next_key.take();
            let target = self.next_target.take().ok_or(LBE::MissingTarget)?;

            let link = FormattableLink::<F, _, _> {
                key,
                target,
                state: self.state,
                phantom: PhantomData,
            };
            self.fmt_set.entry(&link);

            Ok(())
        }

        fn end(&mut self) -> Result<(), LBE> {
            debug_assert!(self.next_key.is_none());
            debug_assert!(self.next_target.is_none());

            self.fmt_set.finish()?;
            Ok(())
        }
    }
}

mod serial {

    use super::*;

    pub(super) struct SerialLinks<'d, F: DataFormatter, D: Data + ?Sized> {
        state: F::State,
        links: Vec<(Option<BoxedData>, BoxedData)>,
        formatter: PhantomData<F>,
        data: PhantomData<&'d D>,
    }

    impl<'d, F: DataFormatter, D: Data + ?Sized> SerialLinks<'d, F, D> {
        #[inline]
        pub(super) fn new(data: &'d D, state: F::State) -> Self {
            let mut builder = SerialLinkBuilder::default();
            // TODO: do something about an error here
            let _ = data.provide_links(&mut builder);
            Self {
                state,
                links: builder.links,
                formatter: PhantomData,
                data: PhantomData,
            }
        }

        #[inline]
        pub(super) fn is_empty(&self) -> bool {
            self.links.is_empty()
        }
    }

    impl<'d, F: DataFormatter, D: Data + ?Sized> Debug for SerialLinks<'d, F, D> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut fmt_set = f.debug_set();
            for (k, t) in &self.links {
                let link = FormattableLink::<F, _, _> {
                    key: k.as_ref().map(|k| &**k),
                    target: t.as_ref(),
                    state: self.state,
                    phantom: PhantomData,
                };
                fmt_set.entry(&link);
            }
            fmt_set.finish()
        }
    }

    #[derive(Default)]
    struct SerialLinkBuilder {
        links: Vec<(Option<BoxedData>, BoxedData)>,
        next_key: Option<BoxedData>,
        next_target: Option<BoxedData>,
    }

    impl LinkBuilder for SerialLinkBuilder {
        fn set_key(&mut self, key: BoxedData) {
            self.next_key.replace(key);
        }
        fn set_target(&mut self, target: BoxedData) {
            self.next_target.replace(target);
        }
        fn build(&mut self) -> Result<(), LBE> {
            let key = self.next_key.take();
            let target = self.next_target.take().ok_or(LBE::MissingTarget)?;

            self.links.push((key, target));

            Ok(())
        }

        fn end(&mut self) -> Result<(), LBE> {
            debug_assert!(self.next_key.is_none());
            debug_assert!(self.next_target.is_none());
            Ok(())
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl ValueBuiler<'_> for fmt::DebugStruct<'_, '_> {
    fn bool(&mut self, value: bool) {
        self.field("bool", &value);
    }
    fn u8(&mut self, value: u8) {
        self.field("u8", &value);
    }
    fn i8(&mut self, value: i8) {
        self.field("i8", &value);
    }
    fn u16(&mut self, value: u16) {
        self.field("u16", &value);
    }
    fn i16(&mut self, value: i16) {
        self.field("i16", &value);
    }
    fn u32(&mut self, value: u32) {
        self.field("u32", &value);
    }
    fn i32(&mut self, value: i32) {
        self.field("i32", &value);
    }
    fn u64(&mut self, value: u64) {
        self.field("u64", &value);
    }
    fn i64(&mut self, value: i64) {
        self.field("i64", &value);
    }
    fn u128(&mut self, value: u128) {
        self.field("u128", &value);
    }
    fn i128(&mut self, value: i128) {
        self.field("i128", &value);
    }
    fn f32(&mut self, value: f32) {
        self.field("f32", &value);
    }
    fn f64(&mut self, value: f64) {
        self.field("f64", &value);
    }
    fn str(&mut self, value: Cow<'_, str>) {
        let value: &str = &value;
        self.field("str", &value);
    }
    fn bytes(&mut self, value: Cow<'_, [u8]>) {
        match String::from_utf8(value.to_vec()) {
            Ok(s) => self.field("bytes", &format_args!("b{s:?}")),
            Err(_) => self.field("bytes", &value.as_ref()),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataExt;

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_vec() {
        let v = vec![1, 2, 3];

        let data = &v as &dyn Data;
        // let list = DataExt::as_list(&data);

        dbg!(data);
        // dbg!(&list);

        assert!(false);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_map() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("key", "val");
        m.insert("key2", "val2");

        let data: &dyn Data = &m;
        // let items = DataExt::as_items(&data);

        dbg!(data);
        // dbg!(&items);

        assert!(false);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_deep_map() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("inner1", "val1");
        m.insert("inner2", "val2");

        let mut m2 = HashMap::new();
        m2.insert("key", m);

        let data: &dyn Data = &m2;

        dbg!(data);

        assert!(false);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_list() {
        let v = vec![1, 2, 3];

        let list = DataExt::as_list(&v);

        let out = format!("{:?}", list);

        assert_eq!(out, "[Data { i32: 1 }, Data { i32: 2 }, Data { i32: 3 }]");
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_items() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("key", "val");
        m.insert("key2", "val2");

        let items = DataExt::as_items(&m);

        let out = format!("{:?}", items);

        assert_eq!(
            out,
            "[(Data { str: \"key2\" }, Data { str: \"val2\" }), (Data { str: \"key\" }, Data { str: \"val\" })]"
        );
    }
}

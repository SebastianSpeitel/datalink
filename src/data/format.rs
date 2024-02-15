use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

use crate::links::{Links, MaybeKeyed, Result, CONTINUE};
use crate::value::ValueBuiler;
use crate::{
    data::{BoxedData, Data},
    links::Link,
};

#[derive(Default, Debug)]
pub struct FORMAT<const SERIAL: bool = false, const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = 0>;

pub type COMPACT<const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = -1> =
    FORMAT<true, MAX_DEPTH, VERBOSITY>;
pub type DEBUG = FORMAT<true, 6, 1>;

pub trait Format {
    type State: Default + Copy;

    #[inline]
    fn verbosity() -> i8 {
        0
    }

    #[inline]
    #[must_use]
    fn init_state() -> Self::State {
        Default::default()
    }

    #[inline]
    fn fmt(
        f: &mut fmt::Formatter<'_>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        // Format prefix
        Self::fmt_prefix(f, data)?;

        if Self::verbosity() <= -1 {
            let values = crate::value::Value::from_data(data);
            let no_links = std::cell::OnceCell::<bool>::new();

            let link_check = || {
                let mut link_check = None::<BoxedData>;
                // Ignore errors
                let _ = data.provide_links(&mut link_check);
                link_check.is_none()
            };

            if Self::verbosity() <= -2 {
                if let Some(num) = super::DataExt::as_number(&values) {
                    if *no_links.get_or_init(link_check) {
                        f.write_fmt(format_args!("{{{:?}}}", num))?;
                        return Ok(());
                    }
                }
            }

            if let Some(val) = values.as_enum() {
                if *no_links.get_or_init(link_check) {
                    if let Some(v) = val {
                        f.write_fmt(format_args!("{{{v}}}"))?;
                    } else {
                        f.write_str("")?;
                    }
                    return Ok(());
                }
            }
        }

        let mut set = f.debug_set();

        // Format values
        Self::fmt_values_into_set(&mut set, data, state);

        // Format links
        Self::fmt_links_into_set(&mut set, data, state);

        // Finish set
        set.finish()?;

        // Format suffix
        Self::fmt_suffix(f, data)?;
        Ok(())
    }

    #[inline]
    fn fmt_prefix(f: &mut fmt::Formatter<'_>, data: &(impl Data + ?Sized)) -> fmt::Result {
        if Self::verbosity() <= -2 {
            f.write_str("D")?;
        } else {
            f.write_str("Data")?;
        }

        #[cfg(feature = "unique")]
        if Self::verbosity() > 0 {
            if let Some(id) = data.get_id() {
                f.write_fmt(format_args!("[{id}]"))?;
            }
        }

        Ok(())
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_suffix(f: &mut fmt::Formatter<'_>, data: &(impl Data + ?Sized)) -> fmt::Result {
        Ok(())
    }

    #[inline]
    fn fmt_link<'a, 'b, L: Link>(
        f: &'a mut fmt::Formatter<'b>,
        link: &L,
        state: Self::State,
    ) -> fmt::Result {
        if let Some(key) = link.key() {
            Self::fmt(f, key, state)?;
            f.write_str(" -> ")?;
        } else {
            f.write_str("- ")?;
        }
        Self::fmt(f, link.target(), state)
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_values_into_set<'a, 'b>(
        set: &mut fmt::DebugSet<'a, 'b>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        data.provide_value(set);
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_links_into_set<'a, 'b>(
        set: &mut fmt::DebugSet<'a, 'b>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        set.entry(&format_args!("..."));
    }
}

impl<const SERIAL: bool, const MAX_DEPTH: u16, const VERBOSITY: i8> Format
    for FORMAT<SERIAL, MAX_DEPTH, VERBOSITY>
{
    type State = u16;

    #[inline]
    fn verbosity() -> i8 {
        VERBOSITY
    }

    #[inline]
    fn init_state() -> Self::State {
        MAX_DEPTH
    }

    #[inline]
    fn fmt_links_into_set<'a, 'b>(
        set: &mut fmt::DebugSet<'a, 'b>,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) {
        if MAX_DEPTH == 0 || state == 0 {
            set.entry(&format_args!("..."));
            return;
        }

        if SERIAL {
            let mut links = Vec::<MaybeKeyed<_, _>>::new();
            // Ignore errors
            let _ = data.provide_links(&mut links);
            let inner_state = state.saturating_sub(1);
            set.entries(links.into_iter().map(|link| {
                let link = LinkEntry::<_, Self> {
                    link,
                    state: inner_state,
                };
                link
            }));
        } else {
            let mut links = StreamingLinks::<'_, '_, '_, Self> {
                fmt_set: set,
                state: state.saturating_sub(1),
            };
            // Ignore errors
            let _ = data.provide_links(&mut links);
        }
    }
}

pub struct FormattableData<'d, F: Format, D: Data + ?Sized> {
    data: &'d D,
    phantom: PhantomData<F>,
}

impl<'d, F: Format, D: Data + ?Sized> From<&'d D> for FormattableData<'d, F, D> {
    #[inline]
    fn from(data: &'d D) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }
}

impl<F: Format, D: Data + ?Sized> Display for FormattableData<'_, F, D> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

impl<F: Format, D: Data + ?Sized> Debug for FormattableData<'_, F, D> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

struct LinkEntry<L, F: Format + ?Sized> {
    link: L,
    state: F::State,
}

impl<L: Link, F: Format + ?Sized> Debug for LinkEntry<L, F> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        F::fmt_link(f, &self.link, self.state)
    }
}

struct StreamingLinks<'a, 'b, 'c, F: Format> {
    fmt_set: &'a mut fmt::DebugSet<'b, 'c>,
    state: F::State,
}

impl<F: Format> Links for StreamingLinks<'_, '_, '_, F> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        let link = LinkEntry::<_, F> {
            link: MaybeKeyed::new(key, target),
            state: self.state,
        };
        self.fmt_set.entry(&link);

        CONTINUE
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        let link = LinkEntry::<_, F> {
            link: (key, target),
            state: self.state,
        };
        self.fmt_set.entry(&link);

        CONTINUE
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        let link = LinkEntry::<_, F> {
            link: target,
            state: self.state,
        };
        self.fmt_set.entry(&link);

        CONTINUE
    }
}

impl ValueBuiler<'_> for fmt::DebugSet<'_, '_> {
    #[inline]
    fn bool(&mut self, value: bool) {
        match value {
            true => self.entry(&format_args!("bool: true")),
            false => self.entry(&format_args!("bool: false")),
        };
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        self.entry(&format_args!("u8: {}", value));
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        self.entry(&format_args!("i8: {}", value));
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        self.entry(&format_args!("u16: {}", value));
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        self.entry(&format_args!("i16: {}", value));
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        self.entry(&format_args!("u32: {}", value));
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        self.entry(&format_args!("i32: {}", value));
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        self.entry(&format_args!("u64: {}", value));
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        self.entry(&format_args!("i64: {}", value));
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        self.entry(&format_args!("u128: {}", value));
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        self.entry(&format_args!("i128: {}", value));
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        self.entry(&format_args!("f32: {}", value));
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        self.entry(&format_args!("f64: {}", value));
    }
    #[inline]
    fn str(&mut self, value: Cow<'_, str>) {
        let value: &str = &value;
        self.entry(&format_args!("str: {:?}", value));
    }
    #[inline]
    fn bytes(&mut self, value: Cow<'_, [u8]>) {
        match String::from_utf8(value.to_vec()) {
            Ok(s) => self.entry(&format_args!("bytes: b{:?}", s)),
            Err(_) => self.entry(&format_args!("bytes: {:?}", value)),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataExt;

    #[test]
    fn keyed_and_unkeyed() {
        struct Unkeyed;
        struct Keyed;

        impl Data for Unkeyed {
            fn provide_links(&self, links: &mut dyn Links) -> Result<()> {
                links.push_unkeyed(Box::new("foo") as BoxedData)?;
                Ok(())
            }
        }

        impl Data for Keyed {
            fn provide_links(&self, links: &mut dyn Links) -> Result<()> {
                links.push_keyed(Box::new(()) as BoxedData, Box::new("foo") as BoxedData)?;
                Ok(())
            }
        }

        let unkeyed = Unkeyed;
        let keyed = Keyed;

        let debug_unkeyed = FormattableData::<DEBUG, _>::from(&unkeyed).to_string();
        let debug_keyed = FormattableData::<DEBUG, _>::from(&keyed).to_string();

        assert_ne!(debug_unkeyed, debug_keyed);
    }

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

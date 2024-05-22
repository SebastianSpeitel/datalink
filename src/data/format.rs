use std::{
    fmt::{self, Debug, Display, Write},
    marker::PhantomData,
};

use crate::rr::{Receiver, Request};
use crate::{
    data::{BoxedData, Data},
    links::Link,
};
use crate::{
    links::{Links, MaybeKeyed, Result, CONTINUE},
    rr::meta,
};

use super::DataExt;

#[derive(Default, Debug)]
pub struct FORMAT<const SERIAL: bool = false, const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = 0>;

pub type COMPACT<const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = -1> =
    FORMAT<true, MAX_DEPTH, VERBOSITY>;
pub type DEBUG = FORMAT<true, 6, 1>;

trait Verbosity {
    fn collapse_value(&self) -> bool;
    fn dedup_number_values(&self) -> bool;
    fn show_id(&self) -> bool;
}

impl Verbosity for i8 {
    #[inline]
    fn collapse_value(&self) -> bool {
        *self <= -1
    }
    #[inline]
    fn dedup_number_values(&self) -> bool {
        *self <= -2
    }
    #[inline]
    fn show_id(&self) -> bool {
        *self >= 1
    }
}

pub trait Format {
    type State: Default + Copy;
    const PREFIX: &'static str = "Data";
    const SUFFIX: &'static str = "";
    /// `0` means no threshold
    const ELLIPSIS_THRESHOLD: usize = 0;
    /// Hide meta values from output
    const HIDE_META: bool = false;
    /// Hide unknown values from output
    const HIDE_UNKNOWN: bool = false;

    #[inline]
    #[must_use]
    fn init_state() -> Self::State {
        Default::default()
    }

    #[inline]
    fn fmt(f: &mut fmt::Formatter, data: &(impl Data + ?Sized), state: Self::State) -> fmt::Result {
        // Format prefix
        Self::fmt_prefix(f, data)?;

        if data.has_links().unwrap_or(true) {
            Self::fmt_linked(f, data, state)?;
        } else {
            Self::fmt_unlinked(f, data, state)?;
        }

        // Format suffix
        Self::fmt_suffix(f, data)?;
        Ok(())
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_empty(f: &mut fmt::Formatter, state: Self::State) -> fmt::Result {
        f.write_str("{{}")
    }

    #[inline]
    fn fmt_unlinked(
        f: &mut fmt::Formatter,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        let mut set = f.debug_set();
        Self::fmt_value_entries(&mut set, data, state);
        set.finish()
    }

    #[inline]
    fn fmt_linked(
        f: &mut fmt::Formatter,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        let mut set = f.debug_set();

        // Format values
        Self::fmt_value_entries(&mut set, data, state);

        // Format links
        Self::fmt_link_entries(&mut set, data, state);

        // Finish set
        set.finish()
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_prefix(f: &mut fmt::Formatter, data: &(impl Data + ?Sized)) -> fmt::Result {
        if Self::PREFIX.is_empty() {
            return Ok(());
        }
        f.write_str(Self::PREFIX)
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_suffix(f: &mut fmt::Formatter, data: &(impl Data + ?Sized)) -> fmt::Result {
        if Self::SUFFIX.is_empty() {
            return Ok(());
        }
        f.write_str(Self::SUFFIX)
    }

    #[inline]
    fn fmt_str(f: &mut fmt::Formatter, str: &str) -> fmt::Result {
        if Self::ELLIPSIS_THRESHOLD == 0 {
            Debug::fmt(str, f)
        } else if str.len() >= Self::ELLIPSIS_THRESHOLD.div_ceil(8) {
            write!(f, "\"{}\"", str.escape_debug().ellipse::<Self>())
        } else {
            Debug::fmt(str, f)
        }
    }

    #[inline]
    fn fmt_bytes(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
        let escaped = bytes.escape_ascii();
        if Self::ELLIPSIS_THRESHOLD == 0 {
            write!(f, "b\"{escaped}\"")
        } else if bytes.len() >= Self::ELLIPSIS_THRESHOLD.div_ceil(8) {
            write!(f, "b\"{}\"", escaped.ellipse::<Self>())
        } else {
            write!(f, "b\"{escaped}\"")
        }
    }

    #[inline]
    fn fmt_value(f: &mut fmt::Formatter, value: &crate::value::Value) -> fmt::Result {
        use crate::value::Value;

        match value {
            Value::String(s) => Self::fmt_str(f, s),
            Value::Bytes(b) => Self::fmt_bytes(f, b),
            v => Display::fmt(v, f),
        }
    }

    #[inline]
    fn fmt_link<L: Link>(
        f: &'_ mut fmt::Formatter<'_>,
        link: &L,
        state: Self::State,
    ) -> fmt::Result {
        if let Some(key) = link.key() {
            Self::fmt(f, key, state)?;
            f.write_str(" -> ")?;
        } else if f.alternate() {
            f.write_str("- ")?;
        }
        Self::fmt(f, link.target(), state)
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_value_entries(set: &mut fmt::DebugSet, data: &(impl Data + ?Sized), state: Self::State) {
        let mut receiver = DebugReceiver::<Self> { set, state };

        let request = Request::new(&mut receiver as &mut dyn Receiver);
        data.provide_value(request);
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_link_entries(set: &mut fmt::DebugSet, data: &(impl Data + ?Sized), state: Self::State) {
        set.entry(&format_args!("..."));
    }
}

impl<const SERIAL: bool, const MAX_DEPTH: u16, const VERBOSITY: i8> Format
    for FORMAT<SERIAL, MAX_DEPTH, VERBOSITY>
{
    type State = u16;
    const ELLIPSIS_THRESHOLD: usize = {
        match VERBOSITY {
            1.. => 0,
            0 => 1024,
            ..=-1 => 25,
        }
    };
    const PREFIX: &'static str = {
        match VERBOSITY {
            ..=-1 => "D",
            _ => "Data",
        }
    };
    const HIDE_UNKNOWN: bool = VERBOSITY <= 0;
    const HIDE_META: bool = VERBOSITY <= -1;

    #[inline]
    fn init_state() -> Self::State {
        MAX_DEPTH
    }

    #[inline]
    fn fmt_prefix(f: &mut fmt::Formatter, data: &(impl Data + ?Sized)) -> fmt::Result {
        f.write_str(Self::PREFIX)?;

        #[cfg(feature = "unique")]
        if VERBOSITY.show_id() {
            if let Some(id) = data.get_id() {
                f.write_fmt(format_args!("[{id}]"))?;
            }
        }

        Ok(())
    }

    #[inline]
    fn fmt_unlinked(
        f: &mut fmt::Formatter,
        data: &(impl Data + ?Sized),
        state: Self::State,
    ) -> fmt::Result {
        use crate::value::{AllValues, Value};
        let mut values = AllValues::default();
        data.provide_value(Request::new(&mut values as &mut dyn Receiver));

        if values.is_empty() {
            return Self::fmt_empty(f, state);
        }

        if Self::HIDE_UNKNOWN || Self::HIDE_META {
            values.retain(|val| match val {
                Value::Other(val) => {
                    if Self::HIDE_UNKNOWN && Self::HIDE_META {
                        return false;
                    }

                    if meta::MetaInfo::about_val(val).is_some() {
                        return !Self::HIDE_META;
                    }

                    !Self::HIDE_UNKNOWN
                }
                _ => true,
            });
        }

        if values.is_empty() {
            return Self::fmt_empty(f, state);
        }

        if !VERBOSITY.collapse_value() {
            let mut set = f.debug_set();
            Self::fmt_value_entries(&mut set, &values, state);
            return set.finish();
        }

        if VERBOSITY.dedup_number_values() {
            let num = values
                .iter()
                .try_fold(None, |num, val| match (num, val.as_number()) {
                    // First number value
                    (None, Some(n)) => Some(Some(n)),
                    // Same number value
                    (Some(n), Some(m)) if n == m => Some(Some(n)),
                    // Different or not a number
                    _ => None,
                })
                .flatten();

            if let Some(num) = num {
                f.write_fmt(format_args!("{{{num}}}"))?;
                return Ok(());
            }
        }

        if let Some(val) = values.single() {
            f.write_char('{')?;
            Self::fmt_value(f, val)?;
            return f.write_char('}');
        }

        let mut set = f.debug_set();
        Self::fmt_value_entries(&mut set, &values, state);
        set.finish()
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_value_entries(set: &mut fmt::DebugSet, data: &(impl Data + ?Sized), state: Self::State) {
        let mut receiver = DebugReceiver::<Self> { set, state };

        let request = Request::new(&mut receiver as &mut dyn Receiver);
        data.provide_value(request);
    }

    #[inline]
    fn fmt_link_entries(set: &mut fmt::DebugSet, data: &(impl Data + ?Sized), state: Self::State) {
        if MAX_DEPTH == 0 || state == 0 {
            set.entry(&format_args!("..."));
            return;
        }

        if SERIAL {
            let mut links = Vec::<MaybeKeyed<_, _>>::new();
            // Ignore errors
            let _ = data.provide_links(&mut links);
            let inner_state = state.saturating_sub(1);
            set.entries(links.into_iter().map(|link| LinkEntry::<_, Self> {
                link,
                state: inner_state,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

impl<F: Format, D: Data + ?Sized> Debug for FormattableData<'_, F, D> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        F::fmt(f, self.data, F::init_state())
    }
}

struct LinkEntry<L, F: Format + ?Sized> {
    link: L,
    state: F::State,
}

impl<L: Link, F: Format + ?Sized> Debug for LinkEntry<L, F> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

struct DebugReceiver<'a, 'b, 'c, F: Format + ?Sized> {
    set: &'a mut fmt::DebugSet<'b, 'c>,
    state: F::State,
}

#[warn(clippy::missing_trait_methods)]
impl<F: Format + ?Sized> Receiver for DebugReceiver<'_, '_, '_, F> {
    #[inline]
    fn bool(&mut self, value: bool) {
        if value {
            self.set.entry(&format_args!("bool: true"));
        } else {
            self.set.entry(&format_args!("bool: false"));
        }
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        self.set.entry(&format_args!("u8: {value}"));
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        self.set.entry(&format_args!("i8: {value}"));
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        self.set.entry(&format_args!("u16: {value}"));
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        self.set.entry(&format_args!("i16: {value}"));
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        self.set.entry(&format_args!("u32: {value}"));
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        self.set.entry(&format_args!("i32: {value}"));
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        self.set.entry(&format_args!("u64: {value}"));
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        self.set.entry(&format_args!("i64: {value}"));
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        self.set.entry(&format_args!("u128: {value}"));
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        self.set.entry(&format_args!("i128: {value}"));
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        self.set.entry(&format_args!("f32: {value}"));
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        self.set.entry(&format_args!("f64: {value}"));
    }

    #[inline]
    fn char(&mut self, value: char) {
        self.set.entry(&format_args!("char: {value:?}"));
    }

    #[inline]
    fn str(&mut self, value: &str) {
        struct StrEntry<'a, F: ?Sized>(&'a str, PhantomData<F>);
        impl<F: Format + ?Sized> Debug for StrEntry<'_, F> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("str: ")?;
                F::fmt_str(f, self.0)
            }
        }
        self.set.entry(&StrEntry::<F>(value, PhantomData));
    }

    #[inline]
    fn str_owned(&mut self, value: String) {
        self.str(&value);
    }

    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        struct BytesEntry<'a, F: ?Sized>(&'a [u8], PhantomData<F>);
        impl<F: Format + ?Sized> Debug for BytesEntry<'_, F> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("bytes: ")?;
                F::fmt_bytes(f, self.0)
            }
        }
        self.set.entry(&BytesEntry::<F>(value, PhantomData));
    }

    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
        self.bytes(&value);
    }

    #[inline]
    fn other_ref(&mut self, value: &dyn std::any::Any) {
        if F::HIDE_UNKNOWN && F::HIDE_META {
            return;
        }

        if let Some(info) = meta::MetaInfo::about_val(value) {
            if !F::HIDE_META {
                self.set.entry(&format_args!("{info}"));
            }
            return;
        }

        self.set.entry(&format_args!("{{unknown}}"));
    }

    #[inline]
    fn other_boxed(&mut self, value: Box<dyn std::any::Any>) {
        self.other_ref(&*value);
    }

    #[inline]
    fn accepts<T: 'static + ?Sized>() -> bool {
        true
    }
}
pub trait Ellipsable {
    fn ellipse<F: Format + ?Sized>(&self) -> impl Display;
}

#[derive(Debug)]
struct Ellipsed<F, I>
where
    F: Format + ?Sized,
    I: IntoIterator,
    I::Item: Into<char>,
{
    format: PhantomData<F>,
    iter: I,
}

impl<F, I> Display for Ellipsed<F, I>
where
    F: Format + ?Sized,
    I: IntoIterator + Clone,
    I::Item: Into<char>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut chars = self.iter.clone().into_iter();

        if F::ELLIPSIS_THRESHOLD == 0 {
            return chars.try_for_each(|c| f.write_char(c.into()));
        }

        chars
            .by_ref()
            .take(F::ELLIPSIS_THRESHOLD - 1)
            .try_for_each(|c| f.write_char(c.into()))?;

        match chars.next() {
            // No more chars
            None => Ok(()),
            // At least two more chars
            Some(_) if chars.next().is_some() => f.write_char('…'),
            // Exactly one more char
            Some(c) => f.write_char(c.into()),
        }
    }
}

impl<T: IntoIterator + Clone> Ellipsable for T
where
    T::Item: Into<char>,
{
    #[inline]
    fn ellipse<F: Format + ?Sized>(&self) -> impl Display {
        Ellipsed {
            format: PhantomData::<F>,
            iter: self.to_owned(),
        }
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

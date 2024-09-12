use core::{
    fmt::{self, Debug, Display, Write},
    marker::PhantomData,
};

use crate::{meta, value::Value, Data, Query, Receiver, TypeFilter};

#[derive(Default, Debug)]
pub struct FORMAT<const SERIAL: bool = false, const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = 0>;

pub type COMPACT<const MAX_DEPTH: u16 = 6, const VERBOSITY: i8 = -1> =
    FORMAT<true, MAX_DEPTH, VERBOSITY>;
pub type DEBUG = FORMAT<true, 6, 1>;

pub trait Format {
    type State: Default + Copy + Debug;

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
    fn enter_key(state: Self::State) -> Self::State {
        state
    }

    #[inline]
    fn enter_target(state: Self::State) -> Self::State {
        state
    }

    #[inline]
    fn fmt_value(f: &mut DataFormatter<impl fmt::Write, Self>, value: Value) -> fmt::Result {
        match value {
            Value::False => f.field(|mut f| write!(f, "bool: false")),
            Value::True => f.field(|mut f| write!(f, "bool: true")),
            Value::Bool(v) => f.field(|mut f| write!(f, "bool: {v}")),
            Value::Char(v) => f.field(|mut f| write!(f, "char: {v}")),
            Value::U8(v) => f.field(|mut f| write!(f, "u8: {v}")),
            Value::I8(v) => f.field(|mut f| write!(f, "i8: {v}")),
            Value::U16(v) => f.field(|mut f| write!(f, "u16: {v}")),
            Value::I16(v) => f.field(|mut f| write!(f, "i16: {v}")),
            Value::U32(v) => f.field(|mut f| write!(f, "u32: {v}")),
            Value::I32(v) => f.field(|mut f| write!(f, "i32: {v}")),
            Value::U64(v) => f.field(|mut f| write!(f, "u64: {v}")),
            Value::I64(v) => f.field(|mut f| write!(f, "i64: {v}")),
            Value::U128(v) => f.field(|mut f| write!(f, "u128: {v}")),
            Value::I128(v) => f.field(|mut f| write!(f, "i128: {v}")),
            Value::F32(v) => f.field(|mut f| write!(f, "f32: {v}")),
            Value::F64(v) => f.field(|mut f| write!(f, "f64: {v}")),
            Value::String(v) => Self::fmt_str(f, &v),
            Value::Bytes(v) => Self::fmt_bytes(f, &v),
            Value::Other(v) => Self::fmt_other(f, v.as_ref()),
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_str(f: &mut DataFormatter<impl fmt::Write, Self>, str: &str) -> fmt::Result {
        let escaped = str.escape_debug();
        if Self::ELLIPSIS_THRESHOLD == 0 {
            f.field(|mut f| write!(f, "str: \"{escaped}\""))
        } else if str.len() >= Self::ELLIPSIS_THRESHOLD.div_ceil(8) {
            f.field(|mut f| write!(f, "str: \"{}\"", escaped.ellipse::<Self>()))
        } else {
            f.field(|mut f| write!(f, "str: \"{escaped}\""))
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn fmt_bytes(f: &mut DataFormatter<impl fmt::Write, Self>, bytes: &[u8]) -> fmt::Result {
        let escaped = bytes.escape_ascii();
        if Self::ELLIPSIS_THRESHOLD == 0 {
            f.field(|mut f| write!(f, "bytes: b\"{escaped}\""))
        } else if bytes.len() >= Self::ELLIPSIS_THRESHOLD.div_ceil(8) {
            f.field(|mut f| write!(f, "bytes: b\"{}\"", escaped.ellipse::<Self>()))
        } else {
            f.field(|mut f| write!(f, "bytes: b\"{escaped}\""))
        }
    }

    #[inline]
    fn fmt_other(
        f: &mut DataFormatter<impl fmt::Write, Self>,
        value: &dyn std::any::Any,
    ) -> fmt::Result {
        if Self::HIDE_UNKNOWN && Self::HIDE_META {
            return Ok(());
        }

        if !Self::HIDE_META && meta::META_TYPES.accepts(value) {
            let info = meta::MetaInfo::about_val(value);
            f.field(|mut f| write!(f, "{info}"))
        } else {
            f.field(|mut f| write!(f, "{{unknown}}"))
        }
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
    const HIDE_UNKNOWN: bool = VERBOSITY <= 0;
    const HIDE_META: bool = VERBOSITY <= -1;

    #[inline]
    fn init_state() -> Self::State {
        MAX_DEPTH
    }

    #[inline]
    fn enter_key(state: Self::State) -> Self::State {
        state.saturating_sub(1)
    }

    #[inline]
    fn enter_target(state: Self::State) -> Self::State {
        state.saturating_sub(1)
    }
}

pub struct FormattableData<'d, F: Format, D: Data + ?Sized> {
    data: &'d D,
    format: PhantomData<F>,
}

impl<'d, F: Format, D: Data + ?Sized> From<&'d D> for FormattableData<'d, F, D> {
    #[inline]
    fn from(data: &'d D) -> Self {
        Self {
            data,
            format: PhantomData,
        }
    }
}

impl<F: Format, D: Data + ?Sized> Display for FormattableData<'_, F, D> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = DataFormatter::<_, F>::new(f);
        self.data.query(&mut f);
        f.finish()
    }
}

impl<F: Format, D: Data + ?Sized> Debug for FormattableData<'_, F, D> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = DataFormatter::<_, F>::new(f);
        self.data.query(&mut f);
        f.finish()
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

struct Indenter<W> {
    buf: W,
    pending: Option<()>,
}

impl<W> From<W> for Indenter<W> {
    #[inline]
    fn from(buf: W) -> Self {
        Self {
            buf,
            pending: Some(()),
        }
    }
}

impl<W: fmt::Write> Indenter<W> {
    #[inline]
    fn write_indented(&mut self, s: &str, indent: &str) -> fmt::Result {
        for line in s.split_inclusive('\n') {
            if self.pending.is_some() {
                self.buf.write_str(indent)?;
            }
            self.pending = line.ends_with('\n').then(|| ());
            self.buf.write_str(line)?;
        }

        Ok(())
    }
}

impl<W: fmt::Write> fmt::Write for Indenter<W> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_indented(s, "  ")
    }
}

#[must_use]
pub struct DataFormatter<W: Write, F: Format + ?Sized = DEBUG> {
    buf: W,
    state: F::State,
    link: Option<(String, String)>,
    has_fields: bool,
    finished: bool,
}

impl<W, F> fmt::Debug for DataFormatter<W, F>
where
    W: Write,
    F: Format + ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataFormatter")
            .field("buf", &format_args!("{}", core::any::type_name::<W>()))
            .field("state", &self.state)
            .field("link", &self.link)
            .finish()
    }
}

impl<W: Write, F: Format + ?Sized> DataFormatter<W, F> {
    #[inline]
    #[must_use]
    pub fn new(buf: W) -> Self {
        Self {
            buf,
            state: F::init_state(),
            link: None,
            has_fields: false,
            finished: false,
        }
    }

    fn new_with_state(buf: W, state: F::State) -> Self {
        Self {
            buf,
            state,
            link: None,
            has_fields: false,
            finished: false,
        }
    }

    #[inline]
    pub fn field(&mut self, f: impl FnOnce(Indenter<&mut W>) -> fmt::Result) -> fmt::Result {
        if self.has_fields {
            self.buf.write_str(",\n")?;
        } else {
            self.buf.write_str("{\n")?;
            self.has_fields = true;
        }
        f(Indenter::from(&mut self.buf))?;
        Ok(())
    }

    fn finalize_link(&mut self) -> fmt::Result {
        if let Some((key, target)) = self.link.take() {
            self.field(|mut f| {
                if !key.is_empty() {
                    f.write_str(&key)?;
                    f.write_str(" => ")?;
                }
                f.write_str(&target)
            })?;
        }
        Ok(())
    }

    #[inline]
    pub fn finish(&mut self) -> fmt::Result {
        if self.finished {
            return Ok(());
        }
        self.finished = true;
        self.finalize_link()?;
        if self.has_fields {
            self.buf.write_str("\n}")?;
        }
        Ok(())
    }
}

#[warn(clippy::missing_trait_methods)]
#[allow(unused_must_use)]
impl<W: Write, F: Format + ?Sized> Receiver for DataFormatter<W, F> {
    fn bool(&mut self, value: bool) {
        F::fmt_value(self, crate::value::Value::Bool(value));
    }
    fn bytes(&mut self, value: &[u8]) {
        F::fmt_value(self, crate::value::Value::Bytes(value.into()));
    }
    fn bytes_owned(&mut self, value: Box<[u8]>) {
        F::fmt_value(self, crate::value::Value::Bytes(value.into()));
    }
    fn char(&mut self, value: char) {
        F::fmt_value(self, crate::value::Value::Char(value));
    }
    fn str(&mut self, value: &str) {
        F::fmt_value(self, crate::value::Value::String(value.into()));
    }
    fn str_owned(&mut self, value: Box<str>) {
        F::fmt_value(self, crate::value::Value::String(value.into()));
    }
    fn u8(&mut self, value: u8) {
        F::fmt_value(self, crate::value::Value::U8(value));
    }
    fn i8(&mut self, value: i8) {
        F::fmt_value(self, crate::value::Value::I8(value));
    }
    fn u16(&mut self, value: u16) {
        F::fmt_value(self, crate::value::Value::U16(value));
    }
    fn i16(&mut self, value: i16) {
        F::fmt_value(self, crate::value::Value::I16(value));
    }
    fn u32(&mut self, value: u32) {
        F::fmt_value(self, crate::value::Value::U32(value));
    }
    fn i32(&mut self, value: i32) {
        F::fmt_value(self, crate::value::Value::I32(value));
    }
    fn u64(&mut self, value: u64) {
        F::fmt_value(self, crate::value::Value::U64(value));
    }
    fn i64(&mut self, value: i64) {
        F::fmt_value(self, crate::value::Value::I64(value));
    }
    fn u128(&mut self, value: u128) {
        F::fmt_value(self, crate::value::Value::U128(value));
    }
    fn i128(&mut self, value: i128) {
        F::fmt_value(self, crate::value::Value::I128(value));
    }
    fn f32(&mut self, value: f32) {
        F::fmt_value(self, crate::value::Value::F32(value));
    }
    fn f64(&mut self, value: f64) {
        F::fmt_value(self, crate::value::Value::F64(value));
    }
    fn other_ref(&mut self, value: &dyn std::any::Any) {
        F::fmt_other(self, value);
    }
    fn other_boxed(&mut self, value: Box<dyn std::any::Any>) {
        F::fmt_value(self, crate::value::Value::Other(value));
    }
}

impl<W: Write, F: Format + ?Sized> Query for DataFormatter<W, F> {
    type Receiver<'q> = &'q mut Self where Self:'q;
    type Filter<'q> = crate::filter::Any where Self:'q;
    type TargetQuery<'q> = DataFormatter<&'q mut String,F> where Self:'q;
    type KeyQuery<'q> = DataFormatter<&'q mut String,F> where Self:'q;

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        self
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        crate::filter::Any
    }

    #[inline]
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
        let _ = self.finalize_link();
        let (key, target) = self
            .link
            .get_or_insert_with(|| (String::new(), String::new()));

        let key_state = F::enter_key(self.state);
        let target_state = F::enter_target(self.state);

        let target = DataFormatter::new_with_state(target, target_state);
        let key = DataFormatter::new_with_state(key, key_state);

        (target, key)
    }
}

impl<W: Write, F: Format + ?Sized> Drop for DataFormatter<W, F> {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataExt;
    use crate::ErasedData;

    #[test]
    fn keyed_and_unkeyed() {
        struct Unkeyed;
        struct Keyed;

        impl Data for Unkeyed {
            fn query(&self, request: &mut impl crate::Request) {
                request.push_link(crate::link::Unkeyed("foo"))
            }
        }

        impl Data for Keyed {
            fn query(&self, request: &mut impl crate::Request) {
                request.push_link(((), "bar"));
            }
        }

        let unkeyed = Unkeyed;
        let keyed = Keyed;

        let mut debug_unkeyed = String::new();
        let mut debug_keyed = String::new();

        unkeyed.query(&mut DataFormatter::<_>::new(&mut debug_unkeyed));
        keyed.query(&mut DataFormatter::<_>::new(&mut debug_keyed));

        dbg!(&debug_unkeyed);
        dbg!(&debug_keyed);

        // assert!(false);

        assert_ne!(debug_unkeyed, debug_keyed);
    }

    #[test]
    #[cfg(feature = "std")]
    fn debug_vec() {
        let v = vec![1, 2, 3];

        let data = &v as &ErasedData;
        // let list = DataExt::as_list(&data);

        dbg!(data);
        // dbg!(&list);

        // assert!(false);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn debug_map() {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert("key", "val");
        m.insert("key2", "val2");

        let data: &ErasedData = &m;
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

        let data: &ErasedData = &m2;

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

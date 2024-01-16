use crate::{data::Data, id::ID, link_builder::Link};
use std::ops::{BitAnd, BitOr, Not};

pub trait Selector<On: ?Sized> {
    fn selects(&self, obj: &On) -> bool;

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        None
    }

    #[inline]
    fn optimize(&mut self) {}
}

#[derive(Debug)]
pub struct TextSelector {
    pub search: Box<str>,
}

#[derive(Default, Debug)]
#[non_exhaustive]
pub enum DataSelector {
    #[default]
    Any,
    Or(Vec<DataSelector>),
    And(Vec<DataSelector>),
    Not(Box<DataSelector>),
    Text(TextSelector),
    Unique,
    Id(ID),
    NotId(ID),
    Linked(Box<LinkSelector>),
    None,
}

#[derive(Default, Debug)]
#[non_exhaustive]
pub enum LinkSelector {
    #[default]
    Any,
    Key(DataSelector),
    Target(DataSelector),
    Or(Vec<LinkSelector>),
    And(Vec<LinkSelector>),
    Not(Box<LinkSelector>),
    None,
}

impl From<String> for TextSelector {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            search: value.into_boxed_str(),
        }
    }
}

impl From<&str> for TextSelector {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl<S: AsRef<str>> Selector<S> for TextSelector {
    #[inline]
    fn selects(&self, obj: &S) -> bool {
        self.search.as_ref() == obj.as_ref()
    }
}

impl DataSelector {
    #[inline]
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }
    #[inline]
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
    #[inline]
    #[must_use]
    pub fn text(s: impl Into<TextSelector>) -> Self {
        Self::Text(s.into())
    }
    #[inline]
    #[must_use]
    pub const fn unique() -> Self {
        Self::Unique
    }
    #[inline]
    #[must_use]
    pub fn id(id: impl Into<ID>) -> Self {
        Self::Id(id.into())
    }
    #[inline]
    #[must_use]
    pub fn not_id(id: impl Into<ID>) -> Self {
        Self::NotId(id.into())
    }
    #[inline]
    #[must_use]
    pub fn linked(selector: impl Into<LinkSelector>) -> Self {
        Self::Linked(Box::new(selector.into()))
    }
    #[cfg(feature = "unique")]
    #[inline]
    #[must_use]
    pub fn eq(data: &impl crate::data::unique::Unique) -> Self {
        Self::Id(data.id())
    }
    #[cfg(feature = "unique")]
    #[inline]
    #[must_use]
    pub fn ne(data: &impl crate::data::unique::Unique) -> Self {
        Self::NotId(data.id())
    }
    #[inline]
    #[must_use]
    pub fn and(mut self, s: impl Into<Self>) -> Self {
        match &mut self {
            Self::And(and) => {
                and.push(s.into());
                self
            }
            _ => Self::And(vec![self, s.into()]),
        }
    }
    #[inline]
    #[must_use]
    pub fn or(mut self, s: impl Into<Self>) -> Self {
        match &mut self {
            Self::Or(or) => {
                or.push(s.into());
                self
            }
            _ => Self::Or(vec![self, s.into()]),
        }
    }
}
impl<D: Data + ?Sized> Selector<D> for DataSelector {
    #[inline]
    fn selects(&self, d: &D) -> bool {
        use DataSelector as E;
        match self {
            E::Any => true,
            E::None => false,
            E::And(and) => and.iter().all(|s| s.selects(d)),
            E::Or(or) => or.iter().any(|s| s.selects(d)),
            E::Id(id) => d.get_id().is_some_and(|ref i| i == id),
            E::NotId(id) => !d.get_id().is_some_and(|ref i| i == id),
            E::Not(s) => !s.selects(d),
            E::Unique => d.get_id().is_some(),
            E::Linked(_s) => {
                unimplemented!();
            }
            E::Text(s) => {
                enum Matcher<'a> {
                    Found,
                    Selecting(&'a TextSelector),
                }
                impl crate::value::ValueBuiler<'_> for Matcher<'_> {
                    fn str(&mut self, value: std::borrow::Cow<'_, str>) {
                        match self {
                            Matcher::Selecting(s) if s.selects(&value) => *self = Matcher::Found,
                            _ => {}
                        }
                    }
                }
                let mut m = Matcher::Selecting(s);
                d.provide_value(&mut m);
                matches!(m, Matcher::Found)
            }
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        use DataSelector as E;
        match self {
            E::Any => Some(true),
            E::None => Some(false),
            _ => None,
        }
    }

    #[inline]
    fn optimize(&mut self) {
        use DataSelector as E;
        match self {
            E::And(and) => match optimize_and::<Self, D>(and) {
                Some(true) => *self = E::Any,
                Some(false) => *self = E::None,
                None => {}
            },
            E::Or(or) => match optimize_or::<Self, D>(or) {
                Some(true) => *self = E::Any,
                Some(false) => *self = E::None,
                None => {}
            },
            _ => {}
        }
    }
}
impl<S: Into<Self>> BitAnd<S> for DataSelector {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: S) -> Self {
        self.and(rhs)
    }
}
impl<S: Into<Self>> BitOr<S> for DataSelector {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: S) -> Self {
        self.or(rhs)
    }
}
impl Not for DataSelector {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl LinkSelector {
    #[inline]
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }
    #[inline]
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
    #[inline]
    #[must_use]
    pub fn key(s: impl Into<DataSelector>) -> Self {
        Self::Key(s.into())
    }
    #[inline]
    #[must_use]
    pub fn target(s: impl Into<DataSelector>) -> Self {
        Self::Target(s.into())
    }
    #[inline]
    #[must_use]
    pub fn and(mut self, s: impl Into<Self>) -> Self {
        match &mut self {
            Self::And(and) => {
                and.push(s.into());
                self
            }
            _ => Self::And(vec![self, s.into()]),
        }
    }
    #[inline]
    #[must_use]
    pub fn or(mut self, s: impl Into<Self>) -> Self {
        match &mut self {
            Self::Or(or) => {
                or.push(s.into());
                self
            }
            _ => Self::Or(vec![self, s.into()]),
        }
    }
}
impl<L: Link + ?Sized> Selector<L> for LinkSelector {
    #[inline]
    fn selects(&self, l: &L) -> bool {
        use LinkSelector as E;
        match self {
            E::Any => true,
            E::None => false,
            E::Not(s) => !s.selects(l),
            E::And(and) => and.iter().all(|s| s.selects(l)),
            E::Or(or) => or.iter().any(|s| s.selects(l)),
            E::Key(s) => l.key().is_some_and(|k| s.selects(k)),
            E::Target(s) => s.selects(l.target()),
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        use LinkSelector as E;
        match self {
            E::Any => Some(true),
            E::None => Some(false),
            _ => None,
        }
    }

    #[inline]
    fn optimize(&mut self) {
        use LinkSelector as E;
        match self {
            E::And(and) => match optimize_and::<Self, L>(and) {
                Some(true) => *self = E::Any,
                Some(false) => *self = E::None,
                None => {}
            },
            E::Or(or) => match optimize_or::<Self, L>(or) {
                Some(true) => *self = E::Any,
                Some(false) => *self = E::None,
                None => {}
            },
            _ => {}
        }
    }
}
impl<S: Into<Self>> BitAnd<S> for LinkSelector {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: S) -> Self {
        self.and(rhs)
    }
}
impl<S: Into<Self>> BitOr<S> for LinkSelector {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: S) -> Self {
        self.or(rhs)
    }
}
impl Not for LinkSelector {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

#[inline]
fn optimize_and<S: Selector<On>, On: ?Sized>(and: &mut Vec<S>) -> Option<bool> {
    let mut anys = Vec::new();
    for (i, s) in and.iter_mut().enumerate() {
        s.optimize();
        match s.as_bool() {
            Some(false) => return Some(false),
            Some(true) => anys.push(i),
            None => {}
        }
    }
    if and.is_empty() || (and.len() == anys.len()) {
        return Some(true);
    }

    for i in anys.drain(..) {
        and.swap_remove(i);
    }
    None
}

#[inline]
fn optimize_or<S: Selector<On>, On: ?Sized>(or: &mut Vec<S>) -> Option<bool> {
    let mut nones = Vec::new();
    for (i, s) in or.iter_mut().enumerate() {
        s.optimize();
        match s.as_bool() {
            Some(true) => return Some(true),
            Some(false) => nones.push(i),
            None => {}
        }
    }
    if or.is_empty() || (or.len() == nones.len()) {
        return Some(true);
    }

    for i in nones.drain(..) {
        or.swap_remove(i);
    }
    None
}

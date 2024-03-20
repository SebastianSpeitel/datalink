use crate::{data::Data, id::ID, links::Link};
use std::{
    borrow::Borrow,
    ops::{BitAnd, BitOr, Not},
};

pub trait Filter<On: ?Sized> {
    fn matches<T: Borrow<On>>(&self, obj: T) -> bool;

    #[inline]
    fn optimize(&mut self) {}

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        None
    }
}

#[derive(Debug)]
pub struct TextFilter {
    pub search: Box<str>,
}

#[derive(Default, Debug)]
#[non_exhaustive]
pub enum DataFilter {
    #[default]
    Any,
    Or(Vec<DataFilter>),
    And(Vec<DataFilter>),
    Not(Box<DataFilter>),
    Text(TextFilter),
    Unique,
    Id(ID),
    NotId(ID),
    Linked(Box<LinkFilter>),
    None,
}

#[derive(Default, Debug)]
#[non_exhaustive]
pub enum LinkFilter {
    #[default]
    Any,
    Key(DataFilter),
    Target(DataFilter),
    Or(Vec<LinkFilter>),
    And(Vec<LinkFilter>),
    Not(Box<LinkFilter>),
    None,
}

impl From<String> for TextFilter {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            search: value.into_boxed_str(),
        }
    }
}

impl From<&str> for TextFilter {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl Filter<str> for TextFilter {
    #[inline]
    fn matches<T: Borrow<str>>(&self, obj: T) -> bool {
        self.search.as_ref() == obj.borrow()
    }
}

impl DataFilter {
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
    pub fn text(f: impl Into<TextFilter>) -> Self {
        Self::Text(f.into())
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
    pub fn linked(filter: impl Into<LinkFilter>) -> Self {
        Self::Linked(Box::new(filter.into()))
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
    pub fn and(mut self, f: impl Into<Self>) -> Self {
        match &mut self {
            Self::And(and) => {
                and.push(f.into());
                self
            }
            _ => Self::And(vec![self, f.into()]),
        }
    }
    #[inline]
    #[must_use]
    pub fn or(mut self, f: impl Into<Self>) -> Self {
        match &mut self {
            Self::Or(or) => {
                or.push(f.into());
                self
            }
            _ => Self::Or(vec![self, f.into()]),
        }
    }
}
impl<D: Data + ?Sized> Filter<D> for DataFilter {
    #[inline]
    fn matches<T: Borrow<D>>(&self, d: T) -> bool {
        use DataFilter as E;
        match self {
            E::Any => true,
            E::None => false,
            E::And(and) => and.iter().all(|f| Filter::<D>::matches(f, d.borrow())),
            E::Or(or) => or.iter().any(|f| Filter::<D>::matches(f, d.borrow())),
            E::Id(id) => d.borrow().get_id().is_some_and(|ref i| i == id),
            E::NotId(id) => !d.borrow().get_id().is_some_and(|ref i| i == id),
            E::Not(f) => !f.matches(d),
            E::Unique => d.borrow().get_id().is_some(),
            E::Linked(f) => {
                struct Searcher<'a>(bool, &'a LinkFilter);
                impl crate::links::Links for Searcher<'_> {
                    #[inline]
                    fn push(
                        &mut self,
                        target: crate::BoxedData,
                        key: Option<crate::BoxedData>,
                    ) -> crate::links::Result {
                        if let Some(key) = key {
                            self.push_keyed(target, key)
                        } else {
                            self.push_unkeyed(target)
                        }
                    }
                    #[inline]
                    fn push_keyed(
                        &mut self,
                        target: crate::BoxedData,
                        key: crate::BoxedData,
                    ) -> crate::links::Result {
                        if self.1.matches((key, target)) {
                            self.0 = true;
                            crate::links::BREAK
                        } else {
                            crate::links::CONTINUE
                        }
                    }
                    #[inline]
                    fn push_unkeyed(&mut self, target: crate::BoxedData) -> crate::links::Result {
                        if Filter::<crate::BoxedData>::matches(self.1, target) {
                            self.0 = true;
                            crate::links::BREAK
                        } else {
                            crate::links::CONTINUE
                        }
                    }
                }
                let mut searcher = Searcher(false, f);
                let _ = d.borrow().provide_links(&mut searcher);
                searcher.0
            }
            E::Text(f) => {
                enum Matcher<'a> {
                    Found,
                    Selecting(&'a TextFilter),
                }
                impl crate::value::ValueBuiler<'_> for Matcher<'_> {
                    fn str(&mut self, value: std::borrow::Cow<'_, str>) {
                        match self {
                            Matcher::Selecting(f) if f.matches(value) => *self = Matcher::Found,
                            _ => {}
                        }
                    }
                }
                let mut m = Matcher::Selecting(f);
                d.borrow().provide_value(&mut m);
                matches!(m, Matcher::Found)
            }
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        use DataFilter as E;
        match self {
            E::Any => Some(true),
            E::None => Some(false),
            _ => None,
        }
    }

    #[inline]
    fn optimize(&mut self) {
        use DataFilter as E;
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
impl<F: Into<Self>> BitAnd<F> for DataFilter {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: F) -> Self {
        self.and(rhs)
    }
}
impl<F: Into<Self>> BitOr<F> for DataFilter {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: F) -> Self {
        self.or(rhs)
    }
}
impl Not for DataFilter {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl LinkFilter {
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
    pub fn key(f: impl Into<DataFilter>) -> Self {
        Self::Key(f.into())
    }
    #[inline]
    #[must_use]
    pub fn target(f: impl Into<DataFilter>) -> Self {
        Self::Target(f.into())
    }
    #[inline]
    #[must_use]
    pub fn and(mut self, f: impl Into<Self>) -> Self {
        match &mut self {
            Self::And(and) => {
                and.push(f.into());
                self
            }
            _ => Self::And(vec![self, f.into()]),
        }
    }
    #[inline]
    #[must_use]
    pub fn or(mut self, f: impl Into<Self>) -> Self {
        match &mut self {
            Self::Or(or) => {
                or.push(f.into());
                self
            }
            _ => Self::Or(vec![self, f.into()]),
        }
    }
}
impl<L: Link + ?Sized> Filter<L> for LinkFilter {
    #[inline]
    fn matches<T: Borrow<L>>(&self, l: T) -> bool {
        use LinkFilter as E;
        match self {
            E::Any => true,
            E::None => false,
            E::Not(f) => !f.matches(l),
            E::And(and) => return and.iter().all(|f| Filter::<L>::matches(f, l.borrow())),
            E::Or(or) => or.iter().any(|f| Filter::<L>::matches(f, l.borrow())),
            E::Key(f) => l
                .borrow()
                .key()
                .is_some_and(|k| Filter::<L::Key>::matches(f, k)),
            E::Target(f) => Filter::<L::Target>::matches(f, l.borrow().target()),
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        use LinkFilter as E;
        match self {
            E::Any => Some(true),
            E::None => Some(false),
            _ => None,
        }
    }

    #[inline]
    fn optimize(&mut self) {
        use LinkFilter as E;
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
impl<F: Into<Self>> BitAnd<F> for LinkFilter {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: F) -> Self {
        self.and(rhs)
    }
}
impl<F: Into<Self>> BitOr<F> for LinkFilter {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: F) -> Self {
        self.or(rhs)
    }
}
impl Not for LinkFilter {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

#[inline]
fn optimize_and<F: Filter<On>, On: ?Sized>(and: &mut Vec<F>) -> Option<bool> {
    let mut anys = Vec::new();
    for (i, f) in and.iter_mut().enumerate() {
        f.optimize();
        match f.as_bool() {
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
fn optimize_or<F: Filter<On>, On: ?Sized>(or: &mut Vec<F>) -> Option<bool> {
    let mut nones = Vec::new();
    for (i, f) in or.iter_mut().enumerate() {
        f.optimize();
        match f.as_bool() {
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

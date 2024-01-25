use super::{Links, Result, CONTINUE};
use crate::data::BoxedData;
use crate::query::{LinkSelector, Selector};

#[derive(Debug)]
pub struct Filtered<'s, 'l, L: Links + ?Sized> {
    pub(super) selector: &'s LinkSelector,
    pub(super) inner: &'l mut L,
}

impl<'s, 'l, L: Links + ?Sized> Links for Filtered<'s, 'l, L> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        let selects = if let Some(key) = &key {
            self.selector.selects(&(key.as_ref(), target.as_ref()))
        } else {
            self.selector.selects(target.as_ref())
        };
        if selects {
            self.inner.push(target, key)
        } else {
            CONTINUE
        }
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        if self.selector.selects(&(key.as_ref(), target.as_ref())) {
            self.inner.push_keyed(target, key)
        } else {
            CONTINUE
        }
    }
    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        if self.selector.selects(target.as_ref()) {
            self.inner.push_unkeyed(target)
        } else {
            CONTINUE
        }
    }
}

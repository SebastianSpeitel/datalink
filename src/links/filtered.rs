use super::{Links, Result, CONTINUE};
use crate::data::BoxedData;
use crate::query::{Filter, LinkFilter};

#[derive(Debug)]
pub struct Filtered<'f, 'l, L: Links + ?Sized> {
    pub(super) filter: &'f LinkFilter,
    pub(super) inner: &'l mut L,
}

impl<'f, 'l, L: Links + ?Sized> Links for Filtered<'f, 'l, L> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        let selects = if let Some(key) = &key {
            self.filter.matches((key.as_ref(), target.as_ref()))
        } else {
            Filter::<BoxedData>::matches(self.filter, &target)
        };
        if selects {
            self.inner.push(target, key)
        } else {
            CONTINUE
        }
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        if self.filter.matches((key.as_ref(), target.as_ref())) {
            self.inner.push_keyed(target, key)
        } else {
            CONTINUE
        }
    }
    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        if Filter::<BoxedData>::matches(self.filter, &target) {
            self.inner.push_unkeyed(target)
        } else {
            CONTINUE
        }
    }
}

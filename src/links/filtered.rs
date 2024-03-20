use super::{Links, Result, CONTINUE};
use crate::data::{BoxedData, Data};
use crate::query::Filter;

#[derive(Debug)]
pub struct Filtered<'f, 'l, L, F>
where
    L: Links + ?Sized,
    F: ?Sized,
{
    pub(super) filter: &'f F,
    pub(super) inner: &'l mut L,
}

impl<'f, 'l, L, F> Links for Filtered<'f, 'l, L, F>
where
    L: Links + ?Sized,
    F: ?Sized,
    // Filters target only
    F: Filter<dyn Data>,
    // Filters key *and* target
    F: for<'d1, 'd2> Filter<(&'d1 dyn Data, &'d2 dyn Data)>,
{
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        let matched = if let Some(key) = &key {
            self.filter.matches((key.as_ref(), target.as_ref()))
        } else {
            <F as Filter<dyn Data>>::matches(self.filter, target.as_ref())
        };
        if matched {
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
        if <F as Filter<dyn Data>>::matches(self.filter, target.as_ref()) {
            self.inner.push_unkeyed(target)
        } else {
            CONTINUE
        }
    }
}

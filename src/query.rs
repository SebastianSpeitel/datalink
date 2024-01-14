mod selectors;
pub use selectors::*;

use crate::data::BoxedData;
use crate::link_builder::Link;

#[derive(Default, Debug)]
pub struct Query {
    selector: LinkSelector,
}

impl Query {
    #[inline]
    #[must_use]
    pub fn new(selector: LinkSelector) -> Self {
        Query { selector }
    }

    #[inline]
    #[must_use]
    pub fn build(mut self) -> Self {
        Selector::<BoxedData>::optimize(&mut self.selector);
        self
    }

    #[inline]
    #[must_use]
    pub fn build_unoptimized(self) -> Self {
        self
    }

    #[inline]
    #[must_use]
    pub fn selector(&self) -> &LinkSelector {
        &self.selector
    }
}

impl<L: Link + ?Sized> Selector<L> for Query {
    #[inline]
    fn selects(&self, obj: &L) -> bool {
        self.selector.selects(obj)
    }
    #[inline]
    fn optimize(&mut self) {
        Selector::<L>::optimize(&mut self.selector);
    }
}

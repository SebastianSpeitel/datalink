mod selectors;
use std::borrow::Borrow;
use std::num::NonZeroUsize;

pub use selectors::*;

use crate::data::BoxedData;
use crate::links::Link;

pub mod prelude {
    pub use super::DataSelector as Data;
    pub use super::LinkSelector as Link;
    pub use super::Query;
    pub use super::TextSelector as Text;
}

#[derive(Default, Debug)]
pub struct Query {
    /// The selector to use to select links.
    selector: LinkSelector,
    /// The maximum number of results to return.
    /// `None` means no limit.
    limit: Option<NonZeroUsize>,
}

impl Query {
    #[inline]
    #[must_use]
    pub const fn new(selector: LinkSelector) -> Self {
        Query {
            selector,
            limit: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn with_limit(mut self, limit: usize) -> Self {
        self.limit = NonZeroUsize::new(limit);
        self
    }

    #[inline]
    #[must_use]
    pub fn build(mut self) -> Self {
        Selector::<BoxedData>::optimize(&mut self.selector);
        self
    }

    #[inline]
    #[must_use]
    pub const fn build_unoptimized(self) -> Self {
        self
    }

    #[inline]
    #[must_use]
    pub const fn selector(&self) -> &LinkSelector {
        &self.selector
    }

    #[inline]
    #[must_use]
    pub const fn limit(&self) -> usize {
        match self.limit {
            Some(limit) => limit.get(),
            None => usize::MAX,
        }
    }
}

impl<L: Link + ?Sized> Selector<L> for Query {
    #[inline]
    fn selects<O: Borrow<L>>(&self, obj: O) -> bool {
        self.selector.selects(obj)
    }
    #[inline]
    fn optimize(&mut self) {
        Selector::<L>::optimize(&mut self.selector);
    }
}

impl From<LinkSelector> for Query {
    #[inline]
    fn from(value: LinkSelector) -> Self {
        Self::new(value)
    }
}

impl From<DataSelector> for Query {
    #[inline]
    fn from(value: DataSelector) -> Self {
        Self::new(LinkSelector::Target(value))
    }
}

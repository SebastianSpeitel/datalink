mod filter;
use std::borrow::Borrow;
use std::num::NonZeroUsize;

pub use filter::*;

use crate::data::BoxedData;
use crate::links::Link;

pub mod prelude {
    pub use super::DataFilter as Data;
    pub use super::LinkFilter as Link;
    pub use super::Query;
    pub use super::TextFilter as Text;
}

#[derive(Default, Debug)]
pub struct Query {
    /// The filter to apply to the links.
    filter: LinkFilter,
    /// The maximum number of results to return.
    /// `None` means no limit.
    limit: Option<NonZeroUsize>,
}

impl Query {
    #[inline]
    #[must_use]
    pub const fn new(filter: LinkFilter) -> Self {
        Query {
            filter,
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
        Filter::<BoxedData>::optimize(&mut self.filter);
        self
    }

    #[inline]
    #[must_use]
    pub const fn build_unoptimized(self) -> Self {
        self
    }

    #[inline]
    #[must_use]
    pub const fn filter(&self) -> &LinkFilter {
        &self.filter
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

impl<L: Link + ?Sized> Filter<L> for Query {
    #[inline]
    fn matches<O: Borrow<L>>(&self, obj: O) -> bool {
        self.filter.matches(obj)
    }
    #[inline]
    fn optimize(&mut self) {
        Filter::<L>::optimize(&mut self.filter);
    }
}

impl From<LinkFilter> for Query {
    #[inline]
    fn from(value: LinkFilter) -> Self {
        Self::new(value)
    }
}

impl From<DataFilter> for Query {
    #[inline]
    fn from(value: DataFilter) -> Self {
        Self::new(LinkFilter::Target(value))
    }
}

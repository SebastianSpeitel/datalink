use std::num::NonZeroUsize;

pub use filters::{Filter, Optimizable};

mod datafilter;
mod linkfilter;
pub use datafilter::DataFilter;
pub use linkfilter::LinkFilter;

use crate::links::Link;

pub mod prelude {
    pub use super::DataFilter as Data;
    pub use super::LinkFilter as Link;
    pub use super::Query;
    pub use filters::prelude::*;
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
        self.filter.optimize();
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
    fn matches(&self, link: &L) -> bool {
        self.filter.matches(link)
    }
}

impl Optimizable for Query {
    #[inline]
    fn optimize(&mut self) {
        self.filter.optimize();
    }
    #[inline]
    fn as_bool(&self) -> Option<bool> {
        self.filter.as_bool()
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

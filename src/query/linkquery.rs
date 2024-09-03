use super::{DataQuery, ErasedDataQuery};

pub trait LinkQuery {
    type KeyQuery<'q>: DataQuery
    where
        Self: 'q;
    type TargetQuery<'q>: DataQuery
    where
        Self: 'q;

    fn key_query(&mut self) -> Self::KeyQuery<'_>;
    fn target_query(&mut self) -> Self::TargetQuery<'_>;

    #[inline]
    fn into_erased<'q>(self) -> ErasedLinkQuery<'q>
    where
        Self: Sized + 'q,
    {
        ErasedLinkQuery::new(self)
    }
}

impl<Q> LinkQuery for &mut Q
where
    Q: LinkQuery + ?Sized,
{
    type KeyQuery<'q> = Q::KeyQuery<'q> where Self:'q;
    type TargetQuery<'q> = Q::TargetQuery<'q> where Self:'q;

    #[inline]
    fn key_query(&mut self) -> Self::KeyQuery<'_> {
        (**self).key_query()
    }

    #[inline]
    fn target_query(&mut self) -> Self::TargetQuery<'_> {
        (**self).target_query()
    }
}

trait ErasableLinkQuery {
    fn erased_key_query(&mut self) -> ErasedDataQuery;
    fn erased_target_query(&mut self) -> ErasedDataQuery;
}

impl<Q> ErasableLinkQuery for Q
where
    Q: LinkQuery + ?Sized,
{
    #[inline]
    fn erased_key_query(&mut self) -> ErasedDataQuery {
        DataQuery::into_erased(self.key_query())
    }
    #[inline]
    fn erased_target_query(&mut self) -> ErasedDataQuery {
        DataQuery::into_erased(self.target_query())
    }
}

impl core::fmt::Debug for dyn ErasableLinkQuery + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErasableLinkQuery").finish_non_exhaustive()
    }
}

impl LinkQuery for ErasedLinkQuery<'_> {
    type KeyQuery<'q> = ErasedDataQuery<'q> where Self:'q;
    type TargetQuery<'q> = ErasedDataQuery<'q> where Self:'q;

    #[inline]
    fn key_query(&mut self) -> Self::KeyQuery<'_> {
        match self {
            Self { query: Some(q) } => q.erased_key_query(),
            _ => ErasedDataQuery::noop(),
        }
    }

    #[inline]
    fn target_query(&mut self) -> Self::TargetQuery<'_> {
        match self {
            Self { query: Some(q) } => q.erased_target_query(),
            _ => ErasedDataQuery::noop(),
        }
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedLinkQuery<'q>
    where
        Self: 'q,
    {
        self
    }
}

#[derive(Debug)]
pub struct ErasedLinkQuery<'q> {
    query: Option<Box<dyn ErasableLinkQuery + 'q>>,
}

impl Default for ErasedLinkQuery<'_> {
    #[inline]
    fn default() -> Self {
        Self::noop()
    }
}

impl<'q> ErasedLinkQuery<'q> {
    #[inline]
    #[must_use]
    pub const fn noop() -> Self {
        Self { query: None }
    }

    #[inline]
    #[must_use]
    pub fn new<Q>(query: Q) -> Self
    where
        Q: LinkQuery + 'q,
    {
        Self {
            query: Some(Box::new(query)),
        }
    }
}

impl LinkQuery for () {
    type KeyQuery<'q> = ();
    type TargetQuery<'q> = ();

    #[inline]
    fn key_query(&mut self) -> Self::KeyQuery<'_> {}

    #[inline]
    fn target_query(&mut self) -> Self::TargetQuery<'_> {}
}

use super::{
    filter::ValidErasedFilter, ErasedLinkQuery, ErasedReceiver, LinkQuery, Receiver, TypeFilter,
};

pub trait DataQuery {
    type Receiver<'q>: Receiver
    where
        Self: 'q;
    type LinkQuery<'q>: LinkQuery
    where
        Self: 'q;
    type Filter<'q>: TypeFilter
    where
        Self: 'q;

    fn receiver(&mut self) -> Self::Receiver<'_>;
    fn link_query(&mut self) -> Self::LinkQuery<'_>;
    fn filter(&self) -> Self::Filter<'_>;

    #[inline]
    fn is_erasing(&self) -> bool {
        false
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedDataQuery<'q>
    where
        Self: Sized + 'q,
    {
        ErasedDataQuery::new(self)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<Q> DataQuery for &mut Q
where
    Q: DataQuery + ?Sized,
{
    type Receiver<'q> = Q::Receiver<'q> where Self:'q;
    type LinkQuery<'q> = Q::LinkQuery<'q> where Self:'q;
    type Filter<'q> = Q::Filter<'q> where Self:'q;

    #[inline]
    fn link_query(&mut self) -> Self::LinkQuery<'_> {
        (**self).link_query()
    }

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        (**self).receiver()
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        (**self).filter()
    }

    #[inline]
    fn is_erasing(&self) -> bool {
        (**self).is_erasing()
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedDataQuery<'q>
    where
        Self: 'q,
    {
        ErasedDataQuery::new(self)
    }
}

#[derive(Debug)]
pub struct ErasedDataQuery<'q> {
    erase_data: bool,
    query: Option<Box<dyn ErasableDataQuery + 'q>>,
}

impl Default for ErasedDataQuery<'_> {
    #[inline]
    fn default() -> Self {
        Self::noop()
    }
}

impl<'q> ErasedDataQuery<'q> {
    #[inline]
    #[must_use]
    pub const fn noop() -> Self {
        Self {
            erase_data: false,
            query: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn new<Q>(query: Q) -> Self
    where
        Q: DataQuery + 'q,
    {
        Self {
            erase_data: query.is_erasing(),
            query: Some(Box::new(query)),
        }
    }
}

trait ErasableDataQuery {
    fn erased_receiver(&mut self) -> ErasedReceiver;
    fn erased_link_query(&mut self) -> ErasedLinkQuery;
    fn erased_filter(&self) -> ValidErasedFilter;
}

impl<Q> ErasableDataQuery for Q
where
    Q: DataQuery + ?Sized,
{
    #[inline]
    fn erased_receiver(&mut self) -> ErasedReceiver {
        if self.filter().is_empty() {
            return ErasedReceiver::default();
        }
        ErasedReceiver::new(self.receiver())
    }

    #[inline]
    fn erased_link_query(&mut self) -> ErasedLinkQuery {
        LinkQuery::into_erased(self.link_query())
    }

    #[inline]
    fn erased_filter(&self) -> ValidErasedFilter {
        TypeFilter::into_erased(self.filter())
    }
}

impl core::fmt::Debug for dyn ErasableDataQuery + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErasableDataQuery").finish_non_exhaustive()
    }
}

impl DataQuery for ErasedDataQuery<'_> {
    type Receiver<'q> = ErasedReceiver<'q> where Self:'q;
    type LinkQuery<'q> = ErasedLinkQuery<'q> where Self:'q;
    type Filter<'q> = ValidErasedFilter<'q> where Self:'q;

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        self.query
            .as_mut()
            .map(|q| (**q).erased_receiver())
            .unwrap_or_default()
    }

    #[inline]
    fn link_query(&mut self) -> Self::LinkQuery<'_> {
        self.query
            .as_mut()
            .map(|q| (**q).erased_link_query())
            .unwrap_or_default()
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        self.query
            .as_ref()
            .map(|q| q.erased_filter())
            .unwrap_or_default()
    }

    #[inline]
    fn is_erasing(&self) -> bool {
        self.erase_data
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedDataQuery<'q>
    where
        Self: 'q,
    {
        self
    }
}

impl DataQuery for () {
    type Receiver<'q> = ();
    type LinkQuery<'q> = ();
    type Filter<'q> = super::filter::Only<()>;

    #[inline]
    fn link_query(&mut self) -> Self::LinkQuery<'_> {}

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {}

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        Default::default()
    }
}

impl<T> DataQuery for Option<T>
where
    T: 'static,
    Option<T>: Receiver,
{
    type LinkQuery<'q> = () where Self:'q;
    type Receiver<'q> = &'q mut Self where Self:'q;
    type Filter<'q> = super::filter::AcceptedBy<Option<T>>;

    #[inline]
    fn link_query(&mut self) -> Self::LinkQuery<'_> {}

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        self
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        Default::default()
    }

    #[inline]
    fn is_erasing(&self) -> bool {
        core::any::TypeId::of::<T>() == core::any::TypeId::of::<Box<crate::ErasedData>>()
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedDataQuery<'q>
    where
        Self: 'q,
    {
        if self.is_some() {
            ErasedDataQuery::noop()
        } else {
            ErasedDataQuery::new(self)
        }
    }
}

use super::{filter::ValidErasedFilter, ErasedReceiver, Receiver, TypeFilter};

pub trait Query {
    type Receiver<'q>: Receiver
    where
        Self: 'q;
    type Filter<'q>: TypeFilter
    where
        Self: 'q;
    type KeyQuery<'q>: Query
    where
        Self: 'q;
    type TargetQuery<'q>: Query
    where
        Self: 'q;

    fn receiver(&mut self) -> Self::Receiver<'_>;
    fn filter(&self) -> Self::Filter<'_>;
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>);

    #[inline]
    fn key_query(&mut self) -> Self::KeyQuery<'_> {
        self.link_query().1
    }

    #[inline]
    fn target_query(&mut self) -> Self::TargetQuery<'_> {
        self.link_query().0
    }

    #[inline]
    fn is_erasing(&self) -> bool {
        false
    }

    #[inline]
    fn into_erased<'q>(self) -> ErasedQuery<'q>
    where
        Self: Sized + 'q,
    {
        ErasedQuery::new(self)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<Q> Query for &mut Q
where
    Q: Query + ?Sized,
{
    type Receiver<'q> = Q::Receiver<'q> where Self:'q;
    type Filter<'q> = Q::Filter<'q> where Self:'q;
    type KeyQuery<'q> = Q::KeyQuery<'q> where Self:'q;
    type TargetQuery<'q> = Q::TargetQuery<'q> where Self:'q;

    #[inline]
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
        (**self).link_query()
    }

    #[inline]
    fn key_query(&mut self) -> Self::KeyQuery<'_> {
        (**self).key_query()
    }

    #[inline]
    fn target_query(&mut self) -> Self::TargetQuery<'_> {
        (**self).target_query()
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
    fn into_erased<'q>(self) -> ErasedQuery<'q>
    where
        Self: 'q,
    {
        ErasedQuery::new(self)
    }
}

#[derive(Debug)]
pub struct ErasedQuery<'q> {
    erase_data: bool,
    query: Option<Box<dyn ErasableQuery + 'q>>,
}

impl Default for ErasedQuery<'_> {
    #[inline]
    fn default() -> Self {
        Self::noop()
    }
}

impl<'q> ErasedQuery<'q> {
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
        Q: Query + 'q,
    {
        Self {
            erase_data: query.is_erasing(),
            query: Some(Box::new(query)),
        }
    }
}

trait ErasableQuery {
    fn erased_receiver(&mut self) -> ErasedReceiver;
    fn erased_filter(&self) -> ValidErasedFilter;
    fn erased_link_query(&mut self) -> (ErasedQuery, ErasedQuery);
}

impl<Q> ErasableQuery for Q
where
    Q: Query + ?Sized,
{
    #[inline]
    fn erased_receiver(&mut self) -> ErasedReceiver {
        if self.filter().is_empty() {
            return ErasedReceiver::default();
        }
        ErasedReceiver::new(self.receiver())
    }

    #[inline]
    fn erased_link_query(&mut self) -> (ErasedQuery, ErasedQuery) {
        let (target, key) = self.link_query();
        let target = Query::into_erased(target);
        let key = Query::into_erased(key);
        (target, key)
    }

    #[inline]
    fn erased_filter(&self) -> ValidErasedFilter {
        TypeFilter::into_erased(self.filter())
    }
}

impl core::fmt::Debug for dyn ErasableQuery + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErasedQuery").finish_non_exhaustive()
    }
}

impl Query for ErasedQuery<'_> {
    type Receiver<'q> = ErasedReceiver<'q> where Self:'q;
    type Filter<'q> = ValidErasedFilter<'q> where Self:'q;
    type KeyQuery<'q> = ErasedQuery<'q> where Self:'q;
    type TargetQuery<'q> = ErasedQuery<'q> where Self:'q;

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {
        self.query
            .as_mut()
            .map(|q| (**q).erased_receiver())
            .unwrap_or_default()
    }

    #[inline]
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
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
    fn into_erased<'q>(self) -> ErasedQuery<'q>
    where
        Self: 'q,
    {
        self
    }
}

impl Query for () {
    type Receiver<'q> = ();
    type Filter<'q> = super::filter::Only<()>;
    type KeyQuery<'q> = ();
    type TargetQuery<'q> = ();

    #[inline]
    fn receiver(&mut self) -> Self::Receiver<'_> {}

    #[inline]
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
        ((), ())
    }

    #[inline]
    fn filter(&self) -> Self::Filter<'_> {
        Default::default()
    }
}

impl<T> Query for Option<T>
where
    T: 'static,
    Option<T>: Receiver,
{
    type Receiver<'q> = &'q mut Self where Self:'q;
    type Filter<'q> = super::filter::AcceptedBy<Option<T>>;
    type KeyQuery<'q> = ();
    type TargetQuery<'q> = ();

    #[inline]
    fn link_query(&mut self) -> (Self::TargetQuery<'_>, Self::KeyQuery<'_>) {
        ((), ())
    }

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
    fn into_erased<'q>(self) -> ErasedQuery<'q>
    where
        Self: 'q,
    {
        if self.is_some() {
            ErasedQuery::noop()
        } else {
            ErasedQuery::new(self)
        }
    }
}

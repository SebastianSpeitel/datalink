use crate::{query::ErasedQuery, Request};

use super::Data;

pub trait ErasableData {
    fn erased_query(&self, request: ErasedQuery);
}

impl<D: Data + ?Sized> ErasableData for D {
    #[inline]
    fn erased_query(&self, mut request: ErasedQuery) {
        Data::query(self, &mut request);
    }
}

pub type ErasedData = dyn ErasableData;

impl Data for dyn ErasableData + '_ {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        (*self).erased_query(request.as_erased());
    }
}

impl Data for Box<dyn ErasableData + '_> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        (**self).erased_query(request.as_erased());
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        (*self).erased_query(request.as_erased());
    }
}

impl core::fmt::Debug for dyn ErasableData + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use super::format::DataFormatter;
        let mut formatter: DataFormatter<_> = DataFormatter::new(f);
        self.query(&mut formatter);

        formatter.finish()
    }
}

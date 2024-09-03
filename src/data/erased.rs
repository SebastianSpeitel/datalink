use crate::{query::ErasedDataQuery, Request};

use super::Data;

pub trait ErasableData {
    fn erased_query(&self, request: ErasedDataQuery);
}

impl<D: Data + ?Sized> ErasableData for D {
    #[inline]
    fn erased_query(&self, mut request: ErasedDataQuery) {
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

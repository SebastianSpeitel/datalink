use crate::{Request, request::ErasableRequest};

type ErasedRequest<'r> = &'r mut (dyn ErasableRequest + 'r);

use super::Data;

pub trait ErasableData {
    fn erased_query(&self, request: ErasedRequest);
}

impl<D: Data + ?Sized> ErasableData for D {
    #[inline]
    fn erased_query(&self, request: ErasedRequest) {
        self.query(request);
    }
}

pub type ErasedData = dyn ErasableData;

macro_rules! impl_unsized_data {
    ($($ty:ty $(,)?)*) => {
        $(
            impl Data for $ty {
                #[inline]
                fn query(&self, mut request: impl Request) {
                    (*self).erased_query(&mut request.as_erased());
                }
            }

            impl Data for &($ty) {
                #[inline]
                fn query(&self, mut request: impl Request) {
                    (*self).erased_query(&mut request.as_erased());
                }
            }

            impl Data for Box<$ty> {
                #[inline]
                fn query(&self, mut request: impl Request) {
                    (**self).erased_query(&mut request.as_erased());
                }
            }

            #[cfg(feature = "std")]
            impl Data for std::sync::Arc<$ty> {
                #[inline]
                fn query(&self, mut request: impl Request) {
                    (**self).erased_query(&mut request.as_erased());
                }
            }

            #[cfg(feature = "std")]
            impl Data for std::rc::Rc<$ty> {
                #[inline]
                fn query(&self, mut request: impl Request) {
                    (**self).erased_query(&mut request.as_erased());
                }
            }


            impl core::fmt::Debug for $ty {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    use super::format::DataFormatter;
                    let mut formatter: DataFormatter<_> = DataFormatter::new(f);
                    self.query(formatter.by_ref());

                    formatter.finish()
                }
            }
        )*
    };
}

impl_unsized_data!(
    dyn ErasableData + '_,
    dyn ErasableData + '_ + Send,
    dyn ErasableData + '_ + Sync,
    dyn ErasableData + '_ + Send + Sync
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataExt;

    #[test]
    fn dyn_data() {
        let i = 100;
        let b = true;

        let int_data = &i as &ErasedData;
        let bool_data: &ErasedData = &b;

        assert_eq!(DataExt::as_i32(int_data), Some(100));
        assert_eq!(DataExt::as_i32(bool_data), None);

        assert_eq!(DataExt::as_bool(int_data), None);
        assert_eq!(DataExt::as_bool(bool_data), Some(true));
    }
}

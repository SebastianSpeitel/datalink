use crate::Request;

#[cfg(feature = "unique")]
pub mod constant;
pub mod erased;
mod ext;
pub mod format;
mod impls;
#[cfg(feature = "unique")]
pub mod unique;

pub use ext::DataExt;

/// The core trait of this crate.
pub trait Data {
    fn query(&self, request: &mut impl Request);

    #[inline]
    fn query_owned(self, request: &mut impl Request)
    where
        Self: Sized,
    {
        self.query(request);
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        let mut q = None;
        self.query(&mut q);
        q
    }
}

// #[cfg(feature = "unique")]
// impl<D: Data + ?Sized> PartialEq<D> for dyn Data {
//     #[inline]
//     fn eq(&self, other: &D) -> bool {
//         match (self.get_id(), other.get_id()) {
//             (Some(self_id), Some(other_id)) => self_id == other_id,
//             _ => false,
//         }
//     }
// }

// impl Debug for dyn Data {
//     #[inline]
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.format::<format::DEBUG>().fmt(f)
//     }
// }
// impl Debug for dyn Data + Sync {
//     #[inline]
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.format::<format::DEBUG>().fmt(f)
//     }
// }
// impl Debug for dyn Data + Send {
//     #[inline]
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.format::<format::DEBUG>().fmt(f)
//     }
// }
// impl Debug for dyn Data + Sync + Send {
//     #[inline]
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.format::<format::DEBUG>().fmt(f)
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct EnsuredErasable<D>(pub D);

#[warn(clippy::missing_trait_methods)]
impl<D: Data + 'static> Data for EnsuredErasable<D> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        self.0.query(request);
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        if request.is_erasing() {
            request.provide_erased_data(Box::new(self.0));
            return;
        }
        self.0.query_owned(request);
    }
    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        self.0.get_id()
    }
}

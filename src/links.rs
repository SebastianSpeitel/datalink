use std::ops::ControlFlow;

use crate::{
    data::{BoxedData, Data},
    query::LinkSelector,
};

pub mod filtered;
pub mod impls;

use filtered::Filtered;

pub mod prelude {
    pub use super::Link;
    pub use super::LinkError;
    pub use super::Links;
    pub use super::LinksExt;
    pub use super::MaybeKeyed;
    pub use super::Result;
    pub use super::BREAK;
    pub use super::CONTINUE;
}

pub type Result<T = ControlFlow<()>, E = LinkError> = core::result::Result<T, E>;
pub const CONTINUE: Result = Ok(ControlFlow::Continue(()));
pub const BREAK: Result = Ok(ControlFlow::Break(()));

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("Unsupported query")]
    UnsupportedQuery,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<std::fmt::Error> for LinkError {
    #[inline]
    fn from(err: std::fmt::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

impl From<std::io::Error> for LinkError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        LinkError::Other(Box::new(err))
    }
}

impl From<std::str::Utf8Error> for LinkError {
    #[inline]
    fn from(err: std::str::Utf8Error) -> Self {
        LinkError::Other(Box::new(err))
    }
}

impl From<std::string::FromUtf8Error> for LinkError {
    #[inline]
    fn from(err: std::string::FromUtf8Error) -> Self {
        LinkError::Other(Box::new(err))
    }
}

impl From<std::num::ParseIntError> for LinkError {
    #[inline]
    fn from(err: std::num::ParseIntError) -> Self {
        LinkError::Other(Box::new(err))
    }
}

impl From<std::num::ParseFloatError> for LinkError {
    #[inline]
    fn from(err: std::num::ParseFloatError) -> Self {
        LinkError::Other(Box::new(err))
    }
}

impl LinkError {
    #[inline]
    pub fn other<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(Box::new(err))
    }
}

pub trait Links {
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result;

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        self.push(target, Some(key))
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        self.push(target, None)
    }
}

pub trait LinksExt: Links {
    #[inline]
    fn push_link<L: Link>(&mut self, link: L) -> Result
    where
        L::Target: 'static + Sized,
        L::Key: 'static + Sized,
    {
        link.build_into(self)
    }

    #[inline]
    fn extend<L: Link>(&mut self, links: impl IntoIterator<Item = L>) -> Result
    where
        L::Target: 'static + Sized,
        L::Key: 'static + Sized,
    {
        for link in links {
            if link.build_into(self)?.is_break() {
                return BREAK;
            }
        }
        CONTINUE
    }

    #[inline]
    fn filter<'s>(&mut self, selector: &'s LinkSelector) -> Filtered<'s, '_, Self> {
        Filtered {
            selector,
            inner: self,
        }
    }
}

impl<T: Links + ?Sized> LinksExt for T {}

pub trait Link {
    type Target: Data + ?Sized;
    type Key: Data + ?Sized;

    fn target(&self) -> &Self::Target;
    fn key(&self) -> Option<&Self::Key>;

    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result
    where
        Self: Sized,
        Self::Key: 'static + Sized,
        Self::Target: 'static + Sized;
}

impl<T> Link for T
where
    T: Data + ?Sized,
{
    type Key = ();
    type Target = T;

    #[inline]
    fn key(&self) -> Option<&Self::Key> {
        None
    }

    #[inline]
    fn target(&self) -> &Self::Target {
        self
    }

    #[inline]
    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result
    where
        Self::Target: 'static + Sized,
    {
        links.push_unkeyed(Box::new(self))
    }
}

impl<K, T> Link for (K, T)
where
    K: Data,
    T: Data + ?Sized,
{
    type Key = K;
    type Target = T;

    #[inline]
    fn key(&self) -> Option<&Self::Key> {
        Some(&self.0)
    }

    #[inline]
    fn target(&self) -> &Self::Target {
        &self.1
    }

    #[inline]
    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result
    where
        Self: Sized,
        Self::Key: 'static,
        Self::Target: 'static + Sized,
    {
        links.push_keyed(Box::new(self.1), Box::new(self.0))
    }
}

#[derive(Debug)]
pub enum MaybeKeyed<K, T> {
    Keyed(K, T),
    Unkeyed(T),
}

impl<K, T> MaybeKeyed<K, T> {
    #[inline]
    pub fn new(key: Option<K>, target: T) -> Self {
        match key {
            Some(key) => Self::Keyed(key, target),
            None => Self::Unkeyed(target),
        }
    }
}

impl<K, T> Link for MaybeKeyed<K, T>
where
    K: Data,
    T: Data,
{
    type Key = K;
    type Target = T;

    #[inline]
    fn key(&self) -> Option<&Self::Key> {
        match self {
            Self::Keyed(key, _) => Some(key),
            Self::Unkeyed(_) => None,
        }
    }

    #[inline]
    fn target(&self) -> &Self::Target {
        match self {
            Self::Keyed(_, t) | Self::Unkeyed(t) => t,
        }
    }

    #[inline]
    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result
    where
        Self::Key: 'static,
        Self::Target: 'static,
    {
        match self {
            Self::Keyed(key, target) => links.push_keyed(Box::new(target), Box::new(key)),
            Self::Unkeyed(target) => links.push_unkeyed(Box::new(target)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_l: &dyn Links) {}
    }

    #[test]
    fn target_only() {
        let mut links: Vec<BoxedData> = Vec::new();

        let target = 42u32;
        links.push_link(target).unwrap();
    }

    #[test]
    fn target_and_key() {
        let mut links: Vec<(BoxedData, BoxedData)> = Vec::new();

        let target = 42u32;
        let key = "foo";
        links.push_link((key, target)).unwrap();
    }

    #[test]
    fn ref_target() {
        let mut links: Vec<BoxedData> = Vec::new();

        let target = &42u32;
        links.push_link(target).unwrap();
    }

    #[test]
    fn option_target() {
        let mut links: Vec<BoxedData> = Vec::new();

        let target = Some(42u32);
        links.push_link(target).unwrap();
    }
}

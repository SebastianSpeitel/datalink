use std::ops::ControlFlow;

use crate::data::{BoxedData, Data};

pub mod impls;

pub mod prelude {
    pub use super::Link;
    pub use super::LinkError;
    pub use super::Links;
    pub use super::LinksExt;
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
    fn push_link(&mut self, link: impl Link) -> Result {
        link.build_into(self)
    }

    #[inline]
    fn extend(&mut self, links: impl IntoIterator<Item = impl Link>) -> Result {
        for link in links {
            if link.build_into(self)?.is_break() {
                return BREAK;
            }
        }
        CONTINUE
    }
}

impl<T: Links + ?Sized> LinksExt for T {}

pub trait Link {
    type Target: Data + 'static;
    type Key: Data + 'static;

    fn target(&self) -> &Self::Target;
    fn key(&self) -> Option<&Self::Key>;

    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result;
}

impl<T> Link for T
where
    T: Data + 'static,
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
    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result {
        links.push_unkeyed(Box::new(self))
    }
}

impl<K, T> Link for (K, T)
where
    K: Data + 'static,
    T: Data + 'static,
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
    fn build_into(self, links: &mut (impl Links + ?Sized)) -> Result {
        links.push_keyed(Box::new(self.1), Box::new(self.0))
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

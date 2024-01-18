use crate::data::{BoxedData, Data};

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
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result<(), LinkError>;

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result<(), LinkError> {
        self.push(target, Some(key))
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result<(), LinkError> {
        self.push(target, None)
    }
}

pub trait LinksExt: Links {
    #[inline]
    fn push(&mut self, link: impl Link) -> Result<(), LinkError> {
        link.build_into(self)
    }

    #[inline]
    fn extend(&mut self, links: impl IntoIterator<Item = impl Link>) -> Result<(), LinkError> {
        for link in links {
            link.build_into(self)?;
        }
        Ok(())
    }
}

impl<T: Links + ?Sized> LinksExt for T {}

impl Links for Vec<(Option<BoxedData>, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result<(), LinkError> {
        self.push((key, target));
        Ok(())
    }
}

pub trait Link {
    fn key(&self) -> Option<&dyn Data>;
    fn target(&self) -> &dyn Data;

    fn build_into(self, builder: &mut (impl Links + ?Sized)) -> Result<(), LinkError>;
}

impl<T> Link for T
where
    T: Data + 'static,
{
    #[inline]
    fn key(&self) -> Option<&dyn Data> {
        None
    }

    #[inline]
    fn target(&self) -> &dyn Data {
        self
    }

    #[inline]
    fn build_into(self, builder: &mut (impl Links + ?Sized)) -> Result<(), LinkError> {
        builder.push_unkeyed(Box::new(self))
    }
}

impl<K, T> Link for (K, T)
where
    K: Data + 'static,
    T: Data + 'static,
{
    #[inline]
    fn key(&self) -> Option<&dyn Data> {
        Some(&self.0)
    }

    #[inline]
    fn target(&self) -> &dyn Data {
        &self.1
    }

    #[inline]
    fn build_into(self, builder: &mut (impl Links + ?Sized)) -> Result<(), LinkError> {
        builder.push_keyed(Box::new(self.1), Box::new(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_l: &dyn Links) {}
    }
}

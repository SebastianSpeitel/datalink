use crate::data::{BoxedData, Data};

#[derive(Debug, thiserror::Error)]
pub enum LinkBuilderError {
    #[error("Missing target")]
    MissingTarget,
    #[error("Already ended")]
    AlreadyEnded,
    #[error("Unsupported query")]
    UnsupportedQuery,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<std::fmt::Error> for LinkBuilderError {
    #[inline]
    fn from(err: std::fmt::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

impl LinkBuilderError {
    #[inline]
    pub fn other<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(Box::new(err))
    }
}

pub trait LinkBuilder {
    fn set_key(&mut self, key: BoxedData);

    fn set_target(&mut self, target: BoxedData);

    fn build(&mut self) -> Result<(), LinkBuilderError>;

    fn end(&mut self) -> Result<(), LinkBuilderError>;
}

pub trait LinkBuilderExt: LinkBuilder {
    #[inline]
    fn push(&mut self, link: impl Link) -> Result<(), LinkBuilderError> {
        link.build_into(self)
    }

    #[inline]
    fn extend(
        &mut self,
        links: impl IntoIterator<Item = impl Link>,
    ) -> Result<(), LinkBuilderError> {
        for link in links {
            link.build_into(self)?;
        }
        Ok(())
    }
}

impl<T: LinkBuilder + ?Sized> LinkBuilderExt for T {}

pub trait Link {
    fn key(&self) -> Option<&dyn Data>;
    fn target(&self) -> &dyn Data;

    fn build_into(self, builder: &mut (impl LinkBuilder + ?Sized)) -> Result<(), LinkBuilderError>;
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
    fn build_into(self, builder: &mut (impl LinkBuilder + ?Sized)) -> Result<(), LinkBuilderError> {
        builder.set_target(Box::new(self));
        builder.build()
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
    fn build_into(self, builder: &mut (impl LinkBuilder + ?Sized)) -> Result<(), LinkBuilderError> {
        builder.set_key(Box::new(self.0));
        builder.set_target(Box::new(self.1));
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        fn _f(_l: &dyn LinkBuilder) {}
    }
}

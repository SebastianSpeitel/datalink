use core::convert::Infallible;

use crate::{
    Data, Request,
    r#type::{Type, TypeOf},
};

mod builder;

pub use builder::LinkBuilder;

#[derive(Debug, Default, Clone, Copy)]
pub struct NoKey;

impl Data for NoKey {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_str("NoKey");
    }
}

pub trait Link: Sized {
    type Key: Type + Default;
    type Target: Type + Default;

    fn query(self, key_request: impl Request, target_request: impl Request);

    fn into_tuple(self) -> (impl Data + 'static, impl Data + 'static);

    #[inline]
    fn into_key(self) -> impl Data + 'static {
        self.into_tuple().0
    }

    #[inline]
    fn into_target(self) -> impl Data + 'static {
        self.into_tuple().1
    }

    #[inline]
    fn query_key(self, key_request: impl Request) {
        self.query(key_request, ());
    }

    #[inline]
    fn query_target(self, target_request: impl Request) {
        self.query((), target_request);
    }
}

impl<K: Data + 'static, T: Data + 'static> Link for (K, T) {
    type Key = TypeOf<K>;
    type Target = TypeOf<T>;

    #[inline]
    fn query(self, key_request: impl Request, target_request: impl Request) {
        self.0.query_owned(key_request);
        self.1.query_owned(target_request);
    }

    #[inline]
    fn query_key(self, key_request: impl Request) {
        self.0.query_owned(key_request);
    }

    #[inline]
    fn query_target(self, target_request: impl Request) {
        self.1.query_owned(target_request);
    }

    #[inline]
    fn into_tuple(self) -> (impl Data + 'static, impl Data + 'static) {
        (self.0, self.1)
    }
}

impl<T: Data + 'static> Link for (T,) {
    type Key = TypeOf<Infallible>;
    type Target = TypeOf<T>;

    #[inline]
    fn query(self, _: impl Request, target_request: impl Request) {
        self.0.query_owned(target_request);
    }

    #[inline]
    fn into_tuple(self) -> (impl Data + 'static, impl Data + 'static) {
        (NoKey, self.0)
    }
}

impl Link for Infallible {
    type Key = TypeOf<Infallible>;
    type Target = TypeOf<Infallible>;

    #[inline]
    fn query(self, _: impl Request, _: impl Request) {
        match self {}
    }

    #[inline]
    fn into_tuple(self) -> (impl Data + 'static, impl Data + 'static) {
        #[allow(unreachable_code)]
        (unreachable!() as Infallible, unreachable!() as Infallible)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErasedData;

    struct Foo;

    impl Data for Foo {
        fn query(&self, mut request: impl Request) {
            request.provide_str("Foo");

            request.provide_link(("key", "value"));
        }
    }

    struct Bar(String);

    impl Data for Bar {
        fn query(&self, mut request: impl Request) {
            self.0.as_str().query_owned(request.by_ref());

            request.provide_link_with(|| ("foo", self.0.clone()));
        }

        fn query_owned(self, mut request: impl Request) {
            self.0.as_str().query_owned(request.by_ref());

            request.provide_link(("foo", self.0));
        }
    }

    #[test]
    fn foo() {
        let foo = Foo;

        dbg!(&foo as &ErasedData);
    }

    #[test]
    fn bar() {
        let bar = Bar("bar".to_string());

        dbg!(&bar as &ErasedData);
    }
}

use core::{marker::PhantomData, ops::Deref};

use crate::{
    Data, Request,
    r#type::{Type, TypeOf},
    util::{EraseToOwned, Eraser},
};

use super::{Link, NoKey};

#[derive(Debug, Clone, Copy)]
pub struct LinkBuilder<K, T, KT = TypeOf<K>, TT = TypeOf<T>, KE = (), TE = ()> {
    key: K,
    target: T,
    _pd: PhantomData<(KT, TT, KE, TE)>,
}

impl<T> LinkBuilder<NoKey, T> {
    #[inline]
    pub const fn new(target: T) -> Self
    where
        T: 'static,
    {
        Self {
            key: NoKey,
            target,
            _pd: PhantomData,
        }
    }
}

impl LinkBuilder<NoKey, NoKey> {
    #[allow(clippy::type_complexity)]
    #[inline]
    pub const fn new_ownable<T>(
        target: T,
    ) -> LinkBuilder<
        NoKey,
        T,
        TypeOf<NoKey>,
        TypeOf<<T::Target as ToOwned>::Owned>,
        (),
        EraseToOwned<T>,
    >
    where
        T: Deref<Target: ToOwned<Owned: 'static>>,
    {
        LinkBuilder {
            key: NoKey,
            target,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub const fn new_with<T, TE: Eraser<T>>(
        target: T,
    ) -> LinkBuilder<NoKey, T, TypeOf<NoKey>, TypeOf<TE::Erased>, (), TE> {
        LinkBuilder {
            key: NoKey,
            target,
            _pd: PhantomData,
        }
    }
}

impl<K, T, KT, TT, KE, TE> LinkBuilder<K, T, KT, TT, KE, TE> {
    #[inline]
    pub fn target<T2>(self, target: T2) -> LinkBuilder<K, T2, KT, TypeOf<T2>, KE, TE>
    where
        T2: 'static,
    {
        LinkBuilder {
            key: self.key,
            target,
            _pd: PhantomData,
        }
    }

    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn target_ownable<T2>(
        self,
        target: T2,
    ) -> LinkBuilder<K, T2, KT, TypeOf<<T2::Target as ToOwned>::Owned>, KE, EraseToOwned<T2>>
    where
        T2: Deref<Target: ToOwned<Owned: 'static>>,
    {
        LinkBuilder {
            key: self.key,
            target,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn target_with<T2, T2E: Eraser<T2>>(
        self,
        target: T2,
    ) -> LinkBuilder<K, T2, KT, TypeOf<T2E::Erased>, KE, T2E> {
        LinkBuilder {
            key: self.key,
            target,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn key<K2>(self, key: K2) -> LinkBuilder<K2, T, TypeOf<K2>, TT, (), TE>
    where
        K2: 'static,
    {
        LinkBuilder {
            key,
            target: self.target,
            _pd: PhantomData,
        }
    }

    #[allow(clippy::type_complexity)]
    #[inline]
    pub fn key_ownable<K2>(
        self,
        key: K2,
    ) -> LinkBuilder<K2, T, TypeOf<<K2::Target as ToOwned>::Owned>, TT, EraseToOwned<K2>, TE>
    where
        K2: Deref<Target: ToOwned<Owned: 'static>>,
    {
        LinkBuilder {
            key,
            target: self.target,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn key_with<K2, K2E: Eraser<K2>>(
        self,
        key: K2,
    ) -> LinkBuilder<K2, T, TypeOf<K2E::Erased>, TT, K2E, TE> {
        LinkBuilder {
            key,
            target: self.target,
            _pd: PhantomData,
        }
    }
}

impl<K, T, KT, TT, KE, TE> Link for LinkBuilder<K, T, KT, TT, KE, TE>
where
    K: Data,
    T: Data,
    KT: Type + Default,
    TT: Type + Default,
    KE: Eraser<K, Erased: Data>,
    TE: Eraser<T, Erased: Data>,
{
    type Key = KT;
    type Target = TT;

    #[inline]
    fn query(self, key_request: impl Request, target_request: impl Request) {
        self.key.query_owned(key_request);
        self.target.query_owned(target_request);
    }

    #[inline]
    fn into_tuple(self) -> (impl Data + 'static, impl Data + 'static) {
        (KE::erase(self.key), TE::erase(self.target))
    }

    #[inline]
    fn into_key(self) -> impl Data + 'static {
        KE::erase(self.key)
    }

    #[inline]
    fn into_target(self) -> impl Data + 'static {
        TE::erase(self.target)
    }
}

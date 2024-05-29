use super::{
    typeset::{self, TypeSet},
    Receiver,
};

pub trait Query {
    type Request;
    type Receiver<'r>: Receiver;
    type Requesting<'r>: TypeSet;

    fn get_receiver<'r>(request: &'r mut Self::Request) -> Self::Receiver<'r>;

    fn get_requesting<'r>(request: &'r Self::Request) -> Self::Requesting<'r>;
}

#[derive(Debug)]
pub struct IgnoreMeta<Q: Query>(core::marker::PhantomData<Q>);

impl<Q: Query> Query for IgnoreMeta<Q> {
    type Request = Q::Request;
    type Receiver<'r> = Q::Receiver<'r>;
    type Requesting<'r> = typeset::And<Q::Requesting<'r>, typeset::Not<super::meta::MetaTypes>>;

    #[inline]
    fn get_receiver(request: &mut Self::Request) -> Self::Receiver<'_> {
        Q::get_receiver(request)
    }

    #[inline]
    fn get_requesting<'r>(request: &'r Self::Request) -> Self::Requesting<'r> {
        typeset::And(
            Q::get_requesting(request),
            typeset::Not(super::meta::MetaTypes::default()),
        )
    }
}

impl<R: Receiver + 'static> Query for R {
    type Request = R;
    type Receiver<'r> = &'r mut R;
    type Requesting<'r> = typeset::AcceptedBy<R>;

    #[inline]
    fn get_receiver(request: &mut Self::Request) -> Self::Receiver<'_> {
        request
    }

    #[inline]
    fn get_requesting<'r>(_request: &'r Self::Request) -> Self::Requesting<'r> {
        typeset::AcceptedBy::new()
    }
}

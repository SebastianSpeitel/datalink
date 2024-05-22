pub mod meta;
pub mod provided;
pub mod receiver;
pub mod request;

pub use receiver::Receiver;
pub use request::Request;

pub mod prelude {
    pub use super::provided::Provided;
    pub use super::receiver::Receiver;
    pub use super::request::Request;
    pub use super::Req;
}

pub trait Req: 'static {
    type Receiver<'d>: Receiver;

    #[inline]
    #[must_use]
    fn requests<T: 'static + ?Sized>() -> bool {
        Self::Receiver::accepts::<T>()
    }
}

#[derive(Debug)]
pub struct Unknown;
impl Req for Unknown {
    type Receiver<'d> = &'d mut dyn Receiver;
}

#[derive(Debug)]
pub struct RefOption<T>(core::marker::PhantomData<T>);
impl<T> Req for RefOption<T>
where
    T: 'static,
    for<'a> &'a mut Option<T>: Receiver,
{
    type Receiver<'d> = &'d mut Option<T>;
}

#[derive(Debug)]
pub struct IgnoreMeta<R: Req>(core::marker::PhantomData<R>);
impl<R: Req> Req for IgnoreMeta<R> {
    type Receiver<'d> = R::Receiver<'d>;

    #[inline]
    fn requests<T: 'static + ?Sized>() -> bool {
        if meta::MetaInfo::about::<T>().name().is_some() {
            return false;
        }
        R::requests::<T>()
    }
}

#[macro_export]
macro_rules! type_eq {
    ($ty1:ty, $ty2:ty) => {
        core::any::TypeId::of::<$ty1>() == core::any::TypeId::of::<$ty2>()
    };
}

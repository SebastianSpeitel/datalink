pub mod meta;
pub mod receiver;
mod request;

pub use receiver::Receiver;
pub use request::Request;

pub trait Req: 'static {
    type Receiver<'d>: Receiver;

    #[inline]
    #[must_use]
    fn requests<T: core::any::Any + ?Sized>() -> bool {
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

use core::{any::Any, marker::PhantomData};

use super::receiver::{Receiver, ReceiverExt};

pub struct ErasedReceiver<'r>(&'r mut dyn ReceiverExt);

impl core::fmt::Debug for ErasedReceiver<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErasedReceiver").finish()
    }
}

#[warn(clippy::missing_trait_methods)]
impl Receiver for ErasedReceiver<'_> {
    #[inline]
    fn bool(&mut self, value: bool) {
        self.0.bool(value);
    }
    #[inline]
    fn i8(&mut self, value: i8) {
        self.0.i8(value);
    }
    #[inline]
    fn u8(&mut self, value: u8) {
        self.0.u8(value);
    }
    #[inline]
    fn i16(&mut self, value: i16) {
        self.0.i16(value);
    }
    #[inline]
    fn u16(&mut self, value: u16) {
        self.0.u16(value);
    }
    #[inline]
    fn i32(&mut self, value: i32) {
        self.0.i32(value);
    }
    #[inline]
    fn u32(&mut self, value: u32) {
        self.0.u32(value);
    }
    #[inline]
    fn i64(&mut self, value: i64) {
        self.0.i64(value);
    }
    #[inline]
    fn u64(&mut self, value: u64) {
        self.0.u64(value);
    }
    #[inline]
    fn i128(&mut self, value: i128) {
        self.0.i128(value);
    }
    #[inline]
    fn u128(&mut self, value: u128) {
        self.0.u128(value);
    }
    #[inline]
    fn f32(&mut self, value: f32) {
        self.0.f32(value);
    }
    #[inline]
    fn f64(&mut self, value: f64) {
        self.0.f64(value);
    }
    #[inline]
    fn char(&mut self, value: char) {
        self.0.char(value);
    }
    #[inline]
    fn str(&mut self, value: &str) {
        self.0.str(value);
    }
    #[inline]
    fn str_owned(&mut self, value: String) {
        self.0.str_owned(value);
    }
    #[inline]
    fn bytes(&mut self, value: &[u8]) {
        self.0.bytes(value);
    }
    #[inline]
    fn bytes_owned(&mut self, value: Vec<u8>) {
        self.0.bytes_owned(value);
    }
    #[inline]
    fn other_ref(&mut self, value: &dyn Any) {
        self.0.other_ref(value);
    }
    #[inline]
    fn other_boxed(&mut self, value: Box<dyn Any>) {
        self.0.other_boxed(value);
    }
    #[inline]
    fn accepting() -> impl super::TypeSet + 'static {
        debug_assert!(false, "ErasedReceiver::accepting() should not be called");
        super::typeset::All
    }
}

pub struct ErasedAccepting<'r>(&'r dyn super::receiver::ReceiverExt);

impl core::fmt::Debug for ErasedAccepting<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErasedAccepting").finish()
    }
}

impl super::TypeSet for ErasedAccepting<'_> {
    #[inline]
    fn contains_id(&self, type_id: core::any::TypeId) -> bool {
        self.0.accepts_id(type_id)
    }
}

#[derive(Debug)]
pub struct Erased<'q>(PhantomData<&'q ()>);

impl<'q> super::query::Query for Erased<'q> {
    type Request = &'q mut dyn ReceiverExt;
    type Receiver<'r> = ErasedReceiver<'r>;
    type Requesting<'r> = ErasedAccepting<'r>;

    #[inline]
    fn get_receiver(request: &mut Self::Request) -> Self::Receiver<'_> {
        ErasedReceiver(*request)
    }

    #[inline]
    fn get_requesting(request: &Self::Request) -> Self::Requesting<'_> {
        ErasedAccepting(*request)
    }
}

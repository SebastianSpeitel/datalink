pub(crate) struct DefaultImpl;
impl Provided for DefaultImpl {
    #[inline]
    fn was_provided(&self) -> bool {
        false
    }
}

pub trait Provided {
    #[inline]
    fn was_provided(&self) -> bool {
        true
    }

    #[inline]
    #[track_caller]
    fn assert_provided(&self) {
        assert!(self.was_provided());
    }

    #[inline]
    #[track_caller]
    fn debug_assert_provided(&self) {
        debug_assert!(self.was_provided());
    }
}

impl Provided for () {}

impl Provided for bool {
    #[inline]
    fn was_provided(&self) -> bool {
        *self
    }
}

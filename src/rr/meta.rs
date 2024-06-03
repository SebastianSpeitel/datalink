use core::any::TypeId;

#[derive(Debug)]
pub struct IsNone;
#[derive(Debug)]
pub struct IsSome;
#[derive(Debug)]
pub struct IsBorrowed;
#[derive(Debug)]
pub struct IsOwned;
#[derive(Debug)]
pub struct IsNull;
#[derive(Debug)]
pub struct IsUnit;

#[derive(Debug)]
#[repr(transparent)]
pub struct MetaInfo(TypeId);

impl MetaInfo {
    #[inline]
    #[must_use]
    pub const fn new(type_id: TypeId) -> Self {
        Self(type_id)
    }

    #[inline]
    #[must_use]
    pub fn about<T: 'static + ?Sized>() -> Self {
        let id = TypeId::of::<T>();
        Self::new(id)
    }

    #[inline]
    #[must_use]
    pub fn about_val(value: &dyn core::any::Any) -> Self {
        let id = value.type_id();
        Self::new(id)
    }

    #[inline]
    pub fn name(&self) -> Option<&'static str> {
        if self.0 == TypeId::of::<IsNone>() {
            Some("IsNone")
        } else if self.0 == TypeId::of::<IsSome>() {
            Some("IsSome")
        } else if self.0 == TypeId::of::<IsBorrowed>() {
            Some("IsBorrowed")
        } else if self.0 == TypeId::of::<IsOwned>() {
            Some("IsOwned")
        } else if self.0 == TypeId::of::<IsNull>() {
            Some("IsNull")
        } else if self.0 == TypeId::of::<IsUnit>() {
            Some("IsUnit")
        } else {
            None
        }
    }
}

impl core::fmt::Display for MetaInfo {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "#{}", self.name().unwrap_or("Unknown"))
    }
}

impl From<TypeId> for MetaInfo {
    #[inline]
    fn from(type_id: TypeId) -> Self {
        Self::new(type_id)
    }
}

pub type MetaTypes = super::typeset::AnyOf<(IsNone, IsSome, IsBorrowed, IsNull, IsOwned, IsUnit)>;

pub const META_TYPES: MetaTypes = super::typeset::AnyOf::new();

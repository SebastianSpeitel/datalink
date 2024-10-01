use core::any::TypeId;

#[derive(Debug, Default)]
pub struct IsNone;
#[derive(Debug, Default)]
pub struct IsSome;
#[derive(Debug, Default)]
pub struct IsBorrowed;
#[derive(Debug, Default)]
pub struct IsOwned;
#[derive(Debug, Default)]
pub struct IsNull;
#[derive(Debug, Default)]
pub struct IsUnit;
#[derive(Debug, Default)]
pub struct IsInfallible;
#[derive(Debug, Default)]
pub struct IsEmptyCell;

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
    #[must_use]
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
        } else if self.0 == TypeId::of::<IsInfallible>() {
            Some("IsInfallible")
        } else if self.0 == TypeId::of::<IsEmptyCell>() {
            Some("IsEmptyCell")
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

pub type MetaTypes = crate::filter::AnyOf<(
    IsNone,
    IsSome,
    IsBorrowed,
    IsNull,
    IsOwned,
    IsUnit,
    IsInfallible,
    IsEmptyCell,
)>;

pub const META_TYPES: MetaTypes = crate::filter::AnyOf::new();

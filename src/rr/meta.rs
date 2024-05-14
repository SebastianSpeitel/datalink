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
pub struct MetaInfo {
    pub name: &'static str,
}

impl MetaInfo {
    #[inline]
    pub fn from_type_id(type_id: TypeId) -> Option<Self> {
        let info = if type_id == TypeId::of::<IsNone>() {
            Self { name: "IsNone" }
        } else if type_id == TypeId::of::<IsSome>() {
            Self { name: "IsSome" }
        } else if type_id == TypeId::of::<IsBorrowed>() {
            Self { name: "IsBorrowed" }
        } else if type_id == TypeId::of::<IsOwned>() {
            Self { name: "IsOwned" }
        } else if type_id == TypeId::of::<IsNull>() {
            Self { name: "IsNull" }
        } else if type_id == TypeId::of::<IsUnit>() {
            Self { name: "IsUnit" }
        } else {
            return None;
        };
        Some(info)
    }

    #[inline]
    pub fn about<T: 'static + ?Sized>() -> Option<Self> {
        let id = TypeId::of::<T>();
        Self::from_type_id(id)
    }

    #[inline]
    pub fn about_val(value: &dyn core::any::Any) -> Option<Self> {
        let id = value.type_id();
        Self::from_type_id(id)
    }
}

impl TryFrom<TypeId> for MetaInfo {
    type Error = ();

    #[inline]
    fn try_from(type_id: TypeId) -> Result<Self, Self::Error> {
        Self::from_type_id(type_id).ok_or(())
    }
}

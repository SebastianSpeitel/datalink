use crate::type_eq;

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

#[inline]
pub fn is_meta<T: 'static + ?Sized>() -> bool {
    type_eq!(T, IsNone)
        || type_eq!(T, IsSome)
        || type_eq!(T, IsBorrowed)
        || type_eq!(T, IsOwned)
        || type_eq!(T, IsNull)
        || type_eq!(T, IsUnit)
}

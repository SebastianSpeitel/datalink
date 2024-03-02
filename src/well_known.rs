macro_rules! impl_well_known {
    ($val:ident, $type:ident $($args:tt)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type;
        $crate::impl_data!($type $($args)*);
        pub const $val: $type = $type;
    };
}

impl_well_known!(NONE, NoneType, id = 0x734BFA09_662B_477C_8B61_7E85B6C47645);
impl_well_known!(
    TAG,
    TagType,
    id = 0x734BFA09_662B_477C_8B61_7E85B6C47646,
    value = "tag"
);
impl_well_known!(
    TYPE,
    TypeType,
    id = 0x734BFA09_662B_477C_8B61_7E85B6C47647,
    value = "type"
);
impl_well_known!(
    KEY,
    KeyType,
    id = 0x734BFA09_662B_477C_8B61_7E85B6C47648,
    value = "key"
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of_val;

    #[test]
    fn none_is_zst() {
        assert_eq!(size_of_val(&NONE), 0);
    }
}

use crate::data::constant::Const;

const NONE_ID: u128 = 0x734BFA09_662B_477C_8B61_7E85B6C47645;
pub const NONE: Const<NONE_ID> = Const::empty();

const TAG_ID: u128 = 0x734BFA09_662B_477C_8B61_7E85B6C47646;
pub const TAG: Const<TAG_ID, &str> = Const::new("tag");

const TYPE_ID: u128 = 0x734BFA09_662B_477C_8B61_7E85B6C47647;
pub const TYPE: Const<TYPE_ID, &str> = Const::new("type");

const KEY_ID: u128 = 0x734BFA09_662B_477C_8B61_7E85B6C47648;
pub const KEY: Const<KEY_ID, &str> = Const::new("key");

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of_val;

    #[test]
    fn none_is_zst() {
        assert_eq!(size_of_val(&NONE), 0);
    }
}

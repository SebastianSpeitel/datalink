use crate::{id::ID, Data, Request};

pub trait WellKnown {
    const ID: ID;

    #[inline]
    fn query(req: &mut impl Request) {
        let _ = req;
    }
}

macro_rules! impl_well_known {
    ($val:ident : $type:ident $(= $data:expr)? ; $id:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type;
        impl WellKnown for $type {
            const ID: $crate::id::ID = unsafe { $crate::id::ID::new_unchecked($id) };

            #[allow(unused_imports, unused_variables)]
            #[inline]
            fn query(request: &mut impl $crate::Request) {
                use $crate::{Data};
                $($data.query(request);)?
            }
        }
        impl Data for $type {
            #[inline]
            fn query(&self, request: &mut impl Request) {
                request.provide_id(Self::ID);
                <Self as WellKnown>::query(request);
            }
        }
        pub const $val: $type = $type;
    };
}

impl_well_known!(NONE:NoneType; 0x734BFA09_662B_477C_8B61_7E85B6C47645);
impl_well_known!(TAG:TagType = "tag"; 0x734BFA09_662B_477C_8B61_7E85B6C47646);
impl_well_known!(TYPE:TypeType = "type"; 0x734BFA09_662B_477C_8B61_7E85B6C47647);
impl_well_known!(KEY:KeyType = "key"; 0x734BFA09_662B_477C_8B61_7E85B6C47648);

pub mod net {
    use super::*;
    impl_well_known!(IP:IPType = "ip"; 0x734BFA09_662B_477C_8B61_7E85B6C47649);
    impl_well_known!(PORT:PortType = "port"; 0x734BFA09_662B_477C_8B61_7E85B6C4764A);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of_val;

    #[test]
    fn none_is_zst() {
        assert_eq!(size_of_val(&NONE), 0);
    }
}

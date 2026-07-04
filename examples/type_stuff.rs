#![allow(unused)]
use core::any::Any;
use core::any::TypeId;
use core::mem::{size_of_val, transmute};
use std::convert::Infallible;
use std::ptr::NonNull;

use datalink::r#type::SimpleType;

enum Type {
    Cpu,
    Gpu { eight: u16, sixteen: u8 },
}

fn main() {
    dbg!(size_of::<SimpleType>());

    let d = true;
    let o = &d as &dyn Any;
    let b = Box::new(d) as Box<dyn Any>;

    dbg!(TypeId::of::<bool>());
    dbg!(TypeId::of::<dyn Any>());
    dbg!(TypeId::of::<Box<dyn Any>>());
    dbg!(d.type_id());
    dbg!(o.type_id());
    dbg!(b.type_id());
    dbg!((*b).type_id());

    dbg!(size_of_val(&None::<&Infallible>));

    enum Foo {
        AB(Infallible, Infallible),
        BA(Infallible, usize),
    }

    dbg!(size_of::<Foo>());

    dbg!(size_of::<Option<(Infallible, Infallible)>>());

    dbg!(size_of::<Type>());

    let mut zero_str = Some("");
    // print bytes of pointer of zero_str
    unsafe {
        let mut raw = transmute::<_, &mut u128>(&mut zero_str);
        dbg!(raw.to_ne_bytes());
        *raw = 0;
        *raw += ((u64::MAX as u128) << 64);
        dbg!(raw.to_ne_bytes());
    }

    if let Some(s) = zero_str {
        dbg!(s.as_bytes().as_ptr());
        dbg!(s.as_ptr().addr());
        dbg!(s.len());
    }
    dbg!(zero_str);

    dbg!(size_of::<NonNull<u8>>());
    dbg!(size_of::<NonNull<[u8]>>());
}

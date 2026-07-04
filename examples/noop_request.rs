use datalink::{DataExt, prelude::*};
// use serde_json::Value;

// use assert_no_alloc::*;
// #[global_allocator]
// static A: AllocDisabler = AllocDisabler;

fn list() {
    let list = vec!["a", "b", "c"];

    let dyn_list = &list as &ErasedData;

    for _ in 0..1_000_000_000usize {
        let as_vec = dyn_list.as_list();

        assert!(as_vec.len() == 3);
        assert!(as_vec[0].as_string().as_deref() == Some("a"));
    }
}

fn strings() {
    let hello = "Hello, world!";
    // let hello_num = 42u32;
    // let hello = 35u32;
    // let hello = Value::String("Hello, world!".to_string());

    // let as_str = DataExt::as_string(&hello);

    // let dyn_hello = &hello as &ErasedData;
    // let dyn_hello_num = &hello_num as &ErasedData;
    // dbg!(dyn_hello);

    for _ in 0..1_000_000_000usize {
        let as_str = hello.as_string();
        // let as_num = assert_no_alloc(|| dyn_hello_num.as_u32());

        // assert!(as_num.is_some());
        // assert_eq!(as_num, Some(42));

        assert!(as_str.is_some());
        assert_eq!(as_str.as_deref(), Some("Hello, world!"));
    }
}

fn ints() {
    let num = 42u32;

    let dyn_num = &num as &ErasedData;

    for _ in 0..20_000_000_000usize {
        let as_num = dyn_num.as_u32();

        assert!(as_num.is_some());
        assert_eq!(as_num, Some(42));
    }
}

fn main() {
    // list();
    // strings();
    ints();
}

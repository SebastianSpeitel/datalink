#![cfg(feature = "derive")]

use datalink::prelude::*;
use datalink::DataExt;

// 1. Unit Struct
#[derive(Data, Debug, Default)]
struct UnitStruct;

// 2. Newtype Struct (should delegate directly to the inner type)
#[derive(Data, Debug, Default)]
struct NewtypeStruct(u32);

// 3. Tuple Struct (should provide elements as indexed or unkeyed links)
#[derive(Data, Debug)]
struct TupleStruct(#[data(copy)] u32, #[data(provide, clone)] String);

// 4. Named Struct with various attributes
#[derive(Data, Debug)]
struct NamedStruct {
    // a is not provided directly, but is a link (default)
    a: u32,

    // b is skipped entirely
    #[data(skip)]
    b: String,

    // c is provided directly (cloned)
    #[data(provide, clone)]
    c: String,

    // d is provided directly (copied)
    #[data(provide, copy)]
    d: bool,

    // e has link skipped, but is provided (ref)
    #[data(provide, link = skip)]
    e: u32,
}

// 5. Enum with all variant types
#[derive(Data, Debug)]
enum TestEnum {
    Unit,
    Tuple(#[data(copy)] u32, #[data(provide, copy)] bool),
    Struct {
        #[data(provide, clone)]
        name: String,
        #[data(skip)]
        secret: u32,
    },
}

#[derive(Data, Debug)]
enum EnumWithDiscriminant {
    A = 10,
    B = 20,
}

#[test]
fn test_unit_struct() {
    let data = UnitStruct;
    // Unit struct has no values or links
    assert!(data.all_values().is_empty());
    assert!(data.as_items().is_empty());
}

#[test]
fn test_newtype_struct() {
    let data = NewtypeStruct(100);
    // Newtype delegates directly, so it behaves exactly like its inner u32
    assert_eq!(data.as_u32(), Some(100));
}

#[test]
fn test_tuple_struct() {
    let data = TupleStruct(42, "hello".to_string());
    
    // First field is copy link, second is provide/clone
    let values = data.all_values();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].as_string(), Some("hello".to_string()));

    let links = data.as_items();
    // Tuple fields are provided as unkeyed links (key is NoKey)
    assert_eq!(links.len(), 2);
    // First link target is 42
    assert_eq!(links[0].1.as_u32(), Some(42));
    // Second link target is "hello"
    assert_eq!(links[1].1.as_string(), Some("hello".to_string()));
}

#[test]
fn test_named_struct_attributes() {
    let data = NamedStruct {
        a: 10,
        b: "skip me".to_string(),
        c: "provide me".to_string(),
        d: true,
        e: 200,
    };

    // Values provided directly:
    // c (String: provide, clone), d (bool: provide, copy), e (u32: provide, link=skip)
    let values = data.all_values();
    assert_eq!(values.len(), 3);
    assert!(values.iter().any(|v| v.as_string() == Some("provide me".to_string())));
    assert!(values.iter().any(|v| v.as_bool() == Some(true)));
    assert!(values.iter().any(|v| v.as_u32() == Some(200)));

    // Links provided:
    // a (default link: key "a"), c (clone link: key "c"), d (copy link: key "d")
    // b is skipped, e is link=skip
    let links = data.as_items();
    assert_eq!(links.len(), 3);

    let keys: Vec<String> = links.iter().map(|l| l.0.as_string().unwrap()).collect();
    assert!(keys.contains(&"a".to_string()));
    assert!(keys.contains(&"c".to_string()));
    assert!(keys.contains(&"d".to_string()));
    assert!(!keys.contains(&"b".to_string()));
    assert!(!keys.contains(&"e".to_string()));
}

#[test]
fn test_enum_unit_variant() {
    let data = TestEnum::Unit;
    assert!(data.all_values().is_empty());
    assert!(data.as_items().is_empty());
}

#[test]
fn test_enum_tuple_variant() {
    let data = TestEnum::Tuple(500, false);
    
    // Second field is provide, copy
    let values = data.all_values();
    assert_eq!(values.len(), 1);
    assert_eq!(values.as_bool(), Some(false));

    // Both fields are links (one copy, one provide+copy)
    let links = data.as_items();
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].1.as_u32(), Some(500));
    assert_eq!(links[1].1.as_bool(), Some(false));
}

#[test]
fn test_enum_struct_variant() {
    let data = TestEnum::Struct {
        name: "Alice".to_string(),
        secret: 999,
    };

    // name is provide, clone. secret is skipped.
    let values = data.all_values();
    assert_eq!(values.len(), 1);
    assert_eq!(values.as_string(), Some("Alice".to_string()));

    // name defaults to link with key "name"
    let links = data.as_items();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].0.as_string(), Some("name".to_string()));
    assert_eq!(links[0].1.as_string(), Some("Alice".to_string()));
}

#[test]
fn test_enum_discriminant() {
    let data = EnumWithDiscriminant::B;

    let links = data.as_items();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].0.as_string(), Some("discriminant".to_string()));
    // Discriminant value should be 20
    assert_eq!(links[0].1.as_i32(), Some(20));
}

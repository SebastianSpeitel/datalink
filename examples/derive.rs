use datalink::prelude::*;

#[derive(Data, Debug, Default)]
struct Foo {
    #[data(provide, clone)]
    key: String,
    #[data(copy)]
    optional: Option<bool>,
}

#[derive(Data, Debug, Default)]
struct Bar(#[data(provide, copy)] u8, #[data(copy)] u8);

#[derive(Data, Debug, Default)]
struct Baz(u8);

#[derive(Data, Debug)]
enum MyEnum {
    Unit,
    Tuple(#[data(copy)] u32, #[data(provide, copy)] bool),
    Struct {
        #[data(provide, clone)]
        name: String,
    },
}

fn main() {
    let foo = &Foo::default() as &ErasedData;
    let bar = &Bar::default() as &ErasedData;
    let baz = &Baz::default() as &ErasedData;
    let my_enum = &MyEnum::Tuple(123, true) as &ErasedData;

    dbg!(foo);
    dbg!(bar);
    dbg!(baz);
    dbg!(my_enum);
}

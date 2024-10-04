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

fn main() {
    let foo = &Foo::default() as &ErasedData;
    let bar = &Bar::default() as &ErasedData;
    let baz = &Baz::default() as &ErasedData;

    dbg!(foo);
    dbg!(bar);
    dbg!(baz);
}

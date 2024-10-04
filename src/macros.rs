#[cfg(feature = "unique")]
#[macro_export]
macro_rules! impl_unique {
    ($name:ident, $id:expr) => {
        impl $crate::data::unique::Unique for $name {
            #[inline]
            fn id(&self) -> $crate::id::ID {
                $crate::id::ID::try_new($id).unwrap()
            }
        }
    };
}

#[cfg(not(feature = "unique"))]
#[macro_export]
macro_rules! impl_unique {
    ($name:ident, $id:expr) => {};
}

/// Implement the `Data` trait for a type.
///
/// # Examples
///
/// ```rust
/// use datalink::impl_data;
///
/// struct Foo;
/// impl_data!(Foo);
///
/// // Provide a single value
/// struct Bar;
/// impl_data!(Bar, value=42u32);
///
/// // Provide multiple values
/// struct Baz;
/// impl_data!(Baz, values=[42u32, 42u64]);
///
/// // Provide links
/// struct Qux;
/// impl_data!(Qux, links=[("key", "val")]);
///
/// // Providing an id automatically implements `Unique`
/// struct Quux;
/// impl_data!(Quux, id=0x734BFA09_662B_477C_8B61_7E85B6C47645);
///
/// struct Corge;
/// impl_data!(Corge, id=0x734BFA09_662B_477C_8B61_7E85B6C47645, values=[42u32, 42u64], links=[("key", "val")]);
/// ```
#[macro_export]
macro_rules! impl_data {
    ($name:ident $(,id=$id:expr)? $(,values=[$($val:expr),+])? $(,links=[$($link:expr),+])?) => {
        impl $crate::data::Data for $name {
            $(
                #[inline]
                fn provide_value(&self, request: &mut $crate::value::ValueRequest) {
                    $(
                        request.provide_value($val);
                    )+
                }

                #[inline]
                fn provide_requested<Q: $crate::rr::Query>(
                    &self,
                    request: &mut $crate::value::ValueRequest<Q>,
                ) -> impl $crate::value::Provided {
                    $(
                        request.provide_value($val);
                    )+
                }
            )?

            $(
                #[inline]
                fn provide_links(&self, links: &mut dyn $crate::links::Links) -> $crate::links::Result<(), $crate::links::LinkError> {
                    use $crate::links::LinksExt;
                    $(links.push_link($link)?;)+
                    Ok(())
                }
            )?

            $(
                #[inline]
                fn get_id(&self) -> Option<$crate::id::ID> {
                    $crate::id::ID::try_new($id).ok()
                }
            )?
        }
        $(
            $crate::impl_unique!($name, $id);
        )?
    };
    ($name:ident $(,id=$id:expr)? $(,value=$val:expr)* $(,links=[$links:tt])?) => {
        $crate::impl_data!($name $(,id=$id)?, values=[$($val),*] $(,links=[$links])?);
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Data, DataExt},
        BoxedData,
    };

    #[test]
    fn impl_data() {
        struct Foo;
        impl_data!(Foo, id = 0x734BFA09_662B_477C_8B61_7E85B6C47645);

        assert!(Foo.get_id().is_some());
    }

    #[test]
    fn impl_data_value() {
        struct Foo;
        impl_data!(Foo, value = 42u32);

        assert_eq!(Foo.as_u32(), Some(42u32));
    }

    #[test]
    fn impl_data_values() {
        struct Foo;
        impl_data!(Foo, values = [42u32, "foo"]);

        assert_eq!(Foo.as_u32(), Some(42u32));
        assert_eq!(Foo.as_str(), Some("foo".into()));
    }

    #[test]
    fn impl_data_links() {
        struct Foo;
        impl_data!(Foo, links = [(42u32, "foo")]);

        let links: Vec<(BoxedData, BoxedData)> = Foo.collect_links().unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0.as_u32(), Some(42u32));
        assert_eq!(links[0].1.as_str(), Some("foo".into()));
    }

    #[test]
    fn impl_data_values_links() {
        struct Foo;
        impl_data!(Foo, values = [42u32, "foo"], links = [(42u32, "foo")]);

        assert_eq!(Foo.as_u32(), Some(42u32));
        assert_eq!(Foo.as_str(), Some("foo".into()));

        let links: Vec<(BoxedData, BoxedData)> = Foo.collect_links().unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0.as_u32(), Some(42u32));
        assert_eq!(links[0].1.as_str(), Some("foo".into()));
    }

    #[test]
    fn impl_data_id_values_links() {
        struct Foo;
        impl_data!(
            Foo,
            id = 0x734BFA09_662B_477C_8B61_7E85B6C47645,
            values = [42u32, "foo"],
            links = [(42u32, "foo")]
        );

        assert!(Foo.get_id().is_some());
        assert_eq!(Foo.as_u32(), Some(42u32));
        assert_eq!(Foo.as_str(), Some("foo".into()));

        let links: Vec<(BoxedData, BoxedData)> = Foo.collect_links().unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0.as_u32(), Some(42u32));
        assert_eq!(links[0].1.as_str(), Some("foo".into()));
    }
}

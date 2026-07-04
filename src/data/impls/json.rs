use serde_json::{Map, Number, Value};

use crate::{Data, LinkBuilder, Request};

impl Data for Value {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_discriminant(self);
        match *self {
            Self::Null => {
                #[cfg(feature = "well_known")]
                crate::well_known::NONE.query_owned(request);
            }
            Self::Bool(b) => b.query_owned(request),
            Self::String(ref s) => s.query_owned(request),
            Self::Number(ref n) => n.query_owned(request),
            Self::Array(ref v) => v.query_owned(request),
            Self::Object(ref o) => o.query_owned(request),
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_discriminant(&self);
        match self {
            Self::Null => {
                #[cfg(feature = "well_known")]
                crate::well_known::NONE.query_owned(request);
            }
            Self::Bool(b) => b.query_owned(request),
            Self::String(s) => s.query_owned(request),
            Self::Number(n) => n.query_owned(request),
            Self::Array(v) => v.query_owned(request),
            Self::Object(o) => o.query_owned(request),
        }
    }
}

impl Data for Map<String, Value> {
    #[inline]
    fn query(&self, mut request: impl Request) {
        for (k, v) in self {
            request.provide_link_with(|| LinkBuilder::new_ownable(v).key_ownable(k));
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        for l in self {
            request.provide_link(l);
        }
    }
}

impl Data for Number {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.try_provide_value_with(|| self.as_f64());
        request.try_provide_value_with(|| self.as_i128());
        request.try_provide_value_with(|| self.as_i64());
        request.try_provide_value_with(|| self.as_u128());
        request.try_provide_value_with(|| self.as_u64());
    }
    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.try_provide_value_with(|| self.as_f64());
        request.try_provide_value_with(|| self.as_i128());
        request.try_provide_value_with(|| self.as_i64());
        request.try_provide_value_with(|| self.as_u128());
        request.try_provide_value_with(|| self.as_u64());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataExt, ErasedData};

    #[test]
    fn number() {
        let n = Number::from(42);
        let mut r = None::<u64>;
        (&n).query(r.by_ref());
        assert_eq!(r.unwrap(), 42u64);

        let mut r = None::<u64>;
        n.query(r.by_ref());
        assert_eq!(r.unwrap(), 42u64);

        let dyn_num = &n as &ErasedData;
        let deb_num = n.format::<crate::data::format::DEBUG>();

        assert_ne!(deb_num.to_string().trim(), "");

        dbg!(&n);
        dbg!(&dyn_num);
        dbg!(&deb_num);

        assert_eq!(format!("{dyn_num:?}"), format!("{deb_num:?}"));
    }

    #[test]
    fn map() {
        let mut map = Map::new();
        map.insert("a".to_string(), Value::from(42));
        map.insert("b".to_string(), Value::from("hello"));

        let items = map.as_items();

        assert_eq!(items.len(), 2);

        assert_eq!(items[0].0.as_string().unwrap(), "a");
        assert_eq!(items[0].1.as_u64().unwrap(), 42);

        assert_eq!(items[1].0.as_string().unwrap(), "b");
        assert_eq!(items[1].1.as_string().unwrap(), "hello");

        let dyn_map = &map as &ErasedData;
        let deb_map = map.format::<crate::data::format::DEBUG>();

        dbg!(&map);
        dbg!(&dyn_map);
        dbg!(&deb_map);

        assert_eq!(format!("{dyn_map:?}"), format!("{deb_map:?}"));
    }
}

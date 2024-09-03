use serde_json::{Map, Number, Value as Val};

use crate::meta;
use crate::{Data, Request};

#[cfg(feature = "well_known")]
use crate::well_known::WellKnown;

impl Data for Val {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_discriminant(self);
        match *self {
            Self::Null => {
                request.provide_default_of::<meta::IsNull>();
                #[cfg(feature = "well_known")]
                request.provide_id(crate::well_known::NoneType::ID);
            }
            Self::Bool(b) => b.query(request),
            Self::String(ref s) => s.query(request),
            Self::Number(ref n) => n.query(request),
            Self::Array(ref v) => v.query(request),
            Self::Object(ref o) => o.query(request),
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        request.provide_discriminant(&self);
        match self {
            Self::Null => {
                request.provide_default_of::<meta::IsNull>();
                #[cfg(feature = "well_known")]
                request.provide_id(crate::well_known::NoneType::ID);
            }
            Self::Bool(b) => b.query(request),
            Self::String(s) => s.query(request),
            Self::Number(n) => n.query(request),
            Self::Array(v) => v.query(request),
            Self::Object(o) => o.query(request),
        }
    }
}

impl Data for Map<String, Val> {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        for (k, v) in self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())) {
            request.push_link((k, v));
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        for l in self {
            request.push_link(l);
        }
    }
}

impl Data for Number {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if true {
            // check if query requests u64
            if let Some(n) = self.as_u64() {
                request.provide_u64(n);
            }
        }

        if true {
            // check if query requests i64
            if let Some(n) = self.as_i64() {
                request.provide_i64(n);
            }
        }

        if true {
            // check if query requests f64
            if let Some(n) = self.as_f64() {
                request.provide_f64(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let n = Number::from(42);
        let mut r = None;
        (&n).query(&mut r);
        assert_eq!(r, Some(42u64));

        let mut r = None::<u64>;
        n.query(&mut r);
        assert_eq!(r, Some(42));
    }
}

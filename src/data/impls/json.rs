use serde_json::{Map, Number, Value as Val};

use crate::data::{Data, Provided};
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{meta, Query, Request};

impl Data for Val {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }

    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        match self {
            Val::Null => request.provide_owned(meta::IsNull),
            Val::Bool(b) => request.provide_ref(b),
            Val::Number(n) => n.provide_requested(request).debug_assert_provided(),
            Val::String(s) => request.provide_str(s),
            Val::Array(..) | Val::Object(..) => {
                // Array and object have no value
            }
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Val::Array(v) => v.provide_links(links),
            Val::Object(m) => m.provide_links(links),
            _ => Ok(()),
        }
    }

    #[inline]
    fn get_id(&self) -> Option<crate::id::ID> {
        match self {
            #[cfg(feature = "well_known")]
            Val::Null => crate::well_known::NONE.get_id(),
            _ => None,
        }
    }
}

impl Data for Map<String, Val> {
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.extend(self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())))?;
        Ok(())
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        use crate::query::Filter;
        links.extend(self.iter().filter_map(|(k, v)| {
            if query.matches_owned((k, v)) {
                Some((k.to_owned(), v.to_owned()))
            } else {
                None
            }
        }))?;
        Ok(())
    }
}

impl Data for Number {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        if request.requests::<u64>() {
            if let Some(n) = self.as_u64() {
                request.provide_u64(n);
            }
        }
        if request.requests::<i64>() {
            if let Some(n) = self.as_i64() {
                request.provide_i64(n);
            }
        }
        if request.requests::<f64>() {
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
        let mut req = Request::new_erased(&mut r);
        n.provide_value(&mut req);
        assert_eq!(r, Some(42u64));

        let mut r = Request::<Option<u64>>::default();
        n.provide_requested(&mut r).assert_provided();
        assert_eq!(r.take(), Some(42));
    }
}

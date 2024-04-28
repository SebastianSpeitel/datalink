use serde_json::{Map, Number, Value as Val};

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{meta, Req, Request};

impl<R: Req> Data<R> for Val {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        match self {
            Val::Null => request.provide_owned(meta::IsNull),
            Val::Bool(b) => request.provide_ref(b),
            Val::Number(n) => n.provide_value(request),
            Val::String(s) => request.provide_ref(s),
            Val::Array(v) => v.provide_value(request),
            Val::Object(m) => m.provide_value(request),
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Val::Array(v) => Data::<R>::provide_links(v, links),
            Val::Object(m) => Data::<R>::provide_links(m, links),
            _ => Ok(()),
        }
    }
}

impl<R: Req> Data<R> for Map<String, Val> {
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

impl<R: Req> Data<R> for Number {
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        if R::requests::<u64>() {
            if let Some(n) = self.as_u64() {
                request.provide_u64(n);
            }
        }
        if R::requests::<i64>() {
            if let Some(n) = self.as_i64() {
                request.provide_i64(n);
            }
        }
        if R::requests::<f64>() {
            if let Some(n) = self.as_f64() {
                request.provide_f64(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr::{Receiver, RefOption};

    #[test]
    fn number() {
        let n = Number::from(42);
        let mut r = None;
        let req: Request = Request::new(&mut r as &mut dyn Receiver);
        n.provide_value(req);
        assert_eq!(r, Some(42u64));

        let mut r = None;
        n.provide_value(Request::<RefOption<u64>>::new(&mut r));
        assert_eq!(r, Some(42));
    }
}

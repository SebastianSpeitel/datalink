use ::toml::{Table, Value as Val};

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{Req, Request};

impl<R: Req> Data<R> for Val {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        match self {
            Val::String(s) => request.provide_ref(s),
            Val::Integer(i) => request.provide_ref(i),
            Val::Float(f) => request.provide_ref(f),
            Val::Boolean(b) => request.provide_ref(b),
            Val::Datetime(dt) => {
                request.provide_ref(dt);
                if R::requests::<String>() {
                    request.provide_str_owned(dt.to_string());
                }
            }
            Val::Array(..) | Val::Table(..) => {}
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Val::Table(table) => Data::<R>::provide_links(table, links),
            Val::Array(array) => Data::<R>::provide_links(array, links),
            _ => Ok(()),
        }
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        match self {
            Val::Table(table) => Data::<R>::query_links(table, links, query),
            Val::Array(array) => Data::<R>::query_links(array, links, query),
            _ => Ok(()),
        }
    }
}

impl<R: Req> Data<R> for Table {
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

use ::toml::{Table, Value as Val};

use crate::data::{Data, Provided};
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{Req, Request};

impl Data for Val {
    #[inline]
    fn provide_value(&self, mut request: Request) {
        self.provide_requested(&mut request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<'d, R: Req>(&self, request: &mut Request<'d, R>) -> impl Provided {
        match self {
            Val::String(s) => request.provide_str(s),
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
            Val::Table(table) => table.provide_links(links),
            Val::Array(array) => array.provide_links(links),
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
            Val::Table(table) => table.query_links(links, query),
            Val::Array(array) => array.query_links(links, query),
            _ => Ok(()),
        }
    }
}

impl Data for Table {
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

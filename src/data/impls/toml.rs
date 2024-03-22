use ::toml::{Table, Value as Val};

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::value::ValueBuiler;

impl Data for Val {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        match self {
            Val::String(s) => value.str(s.into()),
            Val::Integer(i) => value.i64(*i),
            Val::Float(f) => value.f64(*f),
            Val::Boolean(b) => value.bool(*b),
            Val::Datetime(dt) => value.str(dt.to_string().into()),
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

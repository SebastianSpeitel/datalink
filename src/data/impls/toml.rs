use ::toml::value::{Date, Datetime, Offset, Table, Time, Value};

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{provided::Provided, Query, Request};

impl Data for Value {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        use Value as V;
        match *self {
            V::String(ref s) => request.provide_str(s),
            V::Integer(i) => request.provide_i64(i),
            V::Float(f) => request.provide_f64(f),
            V::Boolean(b) => request.provide_bool(b),
            V::Datetime(dt) => dt.provide_requested(request).debug_assert_provided(),
            V::Array(..) | V::Table(..) => {}
        }
    }

    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        use Value as V;
        match self {
            V::Table(table) => table.provide_links(links),
            V::Array(array) => array.provide_links(links),
            V::Datetime(dt) => dt.provide_links(links),
            _ => Ok(()),
        }
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        use Value as V;
        match self {
            V::Table(table) => table.query_links(links, query),
            V::Array(array) => array.query_links(links, query),
            V::Datetime(dt) => dt.query_links(links, query),
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

impl Data for Datetime {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        if request.requests::<String>() {
            request.provide_str_owned(self.to_string());
        }
        request.provide_ref(self);
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.push_link(("date", self.date))?;
        links.push_link(("time", self.time))?;
        links.push_link(("offset", self.offset))?;
        Ok(())
    }
}

impl Data for Date {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        if request.requests::<String>() {
            request.provide_str_owned(self.to_string());
        }
        request.provide_ref(self);
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.push_link(("year", self.year))?;
        links.push_link(("month", self.month))?;
        links.push_link(("day", self.day))?;
        Ok(())
    }
}

impl Data for Time {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        if request.requests::<String>() {
            request.provide_str_owned(self.to_string());
        }
        request.provide_ref(self);
    }
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.push_link(("hour", self.hour))?;
        links.push_link(("minute", self.minute))?;
        links.push_link(("second", self.second))?;
        links.push_link(("nanosecond", self.nanosecond))?;
        Ok(())
    }
}

impl Data for Offset {
    #[inline]
    fn provide_value(&self, request: &mut Request) {
        self.provide_requested(request).debug_assert_provided();
    }
    #[inline]
    fn provide_requested<Q: Query>(&self, request: &mut Request<Q>) -> impl Provided {
        if request.requests::<String>() {
            request.provide_str_owned(self.to_string());
        }
        match *self {
            Self::Z => request.provide_i16(0),
            Self::Custom { minutes } => request.provide_i16(minutes),
        }
        request.provide_ref(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn datetime() {
        let table = Table::from_str("value = 1911-01-01T10:11:12-00:36\n").unwrap();
        dbg!(&table as &dyn Data);

        let entries = table.as_items().unwrap();

        assert_eq!(entries.len(), 1);

        let (key, value) = &entries[0];

        assert_eq!(key.as_str().unwrap(), "value");
        assert_eq!(
            value.as_str().unwrap().to_string(),
            "1911-01-01T10:11:12-00:36"
        );

        let items = value.as_items().unwrap();

        assert_eq!(items.len(), 3);
    }
}

use toml::value::{Date, Datetime, Offset, Table, Time, Value};

use crate::{Data, LinkBuilder, Request};

impl Data for Value {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_discriminant(self);
        match *self {
            Self::String(ref s) => s.query_owned(request),
            Self::Integer(i) => i.query_owned(request),
            Self::Float(f) => f.query_owned(request),
            Self::Boolean(b) => b.query_owned(request),
            Self::Datetime(ref dt) => dt.query_owned(request),
            Self::Array(ref a) => a.query_owned(request),
            Self::Table(ref t) => t.query_owned(request),
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_discriminant(&self);
        match self {
            Self::String(s) => s.query_owned(request),
            Self::Integer(i) => i.query_owned(request),
            Self::Float(f) => f.query_owned(request),
            Self::Boolean(b) => b.query_owned(request),
            Self::Datetime(dt) => dt.query_owned(request),
            Self::Array(a) => a.query_owned(request),
            Self::Table(t) => t.query_owned(request),
        }
    }
}

impl Data for Table {
    #[inline]
    fn query(&self, mut request: impl Request) {
        for (k, v) in self {
            request.provide_link_with(|| LinkBuilder::new_ownable(v).key_ownable(k));
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        for e in self {
            request.provide_link(e);
        }
    }
}

impl Data for Datetime {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_value_with(|| self.to_string());
        request.provide_link(("date", self.date));
        request.provide_link(("time", self.time));
        request.provide_link(("offset", self.offset));
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_value_with(|| self.to_string());
        request.provide_link(("date", self.date));
        request.provide_link(("time", self.time));
        request.provide_link(("offset", self.offset));
    }
}

impl Data for Date {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_value_with(|| self.to_string());
        request.provide_link(("year", self.year));
        request.provide_link(("month", self.month));
        request.provide_link(("day", self.day));
    }
}

impl Data for Time {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_value_with(|| self.to_string());
        request.provide_link(("hour", self.hour));
        request.provide_link(("minute", self.minute));
        request.provide_link(("second", self.second));
        request.provide_link(("nanosecond", self.nanosecond));
    }
}

impl Data for Offset {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_value_with(|| self.to_string());
        match *self {
            Self::Z => {
                request.provide_value(0i16);
                request.provide_str("Z");
            }
            Self::Custom { minutes } => request.provide_value(minutes),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DataExt, ErasedData};
    use std::str::FromStr;

    use super::*;

    #[test]
    fn datetime() {
        let table = Table::from_str("value = 1911-01-01T10:11:12-00:36\n").unwrap();

        // dbg!(&table);
        // dbg!(&table as &dyn Data);

        let entries = table.as_items();

        // dbg!(&entries);

        assert_eq!(entries.len(), 1);

        let (key, value) = &entries[0];

        // dbg!(core::any::type_name_of_val(key));
        // dbg!(core::any::type_name_of_val(value));

        // dbg!(5);

        assert_eq!(key.as_string().unwrap(), "value");
        assert_eq!(value.as_string().unwrap(), "1911-01-01T10:11:12-00:36");

        dbg!(&value);

        let items = value.as_items();

        dbg!(&items);

        assert_eq!(items.len(), 3);
    }

    #[test]
    fn cargo_toml() {
        let cargo_toml = String::from_utf8(include_bytes!("../../../Cargo.toml").to_vec()).unwrap();

        let table = Table::from_str(&cargo_toml).unwrap();

        dbg!(&table);
        dbg!(&table as &ErasedData);

        dbg!(table.format::<crate::data::format::DEBUG>());

        // panic!()
    }
}

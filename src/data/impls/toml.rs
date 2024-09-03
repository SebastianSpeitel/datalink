use ::toml::value::{Date, Datetime, Offset, Table, Time, Value};

use crate::{Data, Request};

impl Data for Value {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_discriminant(self);
        match *self {
            Value::String(ref s) => s.query(request),
            Value::Integer(i) => i.query(request),
            Value::Float(f) => f.query(request),
            Value::Boolean(b) => b.query(request),
            Value::Datetime(ref dt) => dt.query(request),
            Value::Array(ref a) => a.query(request),
            Value::Table(ref t) => t.query(request),
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        request.provide_discriminant(&self);
        match self {
            Value::String(s) => s.query(request),
            Value::Integer(i) => i.query(request),
            Value::Float(f) => f.query(request),
            Value::Boolean(b) => b.query(request),
            Value::Datetime(dt) => dt.query(request),
            Value::Array(a) => a.query(request),
            Value::Table(t) => t.query(request),
        }
    }
}

impl Data for Table {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        for l in self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())) {
            request.push_link(l);
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        for l in self {
            request.push_link(l);
        }
    }
}

impl Data for Datetime {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        if true {
            // check if query requests String
            request.provide_string(self.to_string());
        }
        request.push_link(("date", self.date));
        request.push_link(("time", self.time));
        request.push_link(("offset", self.offset));
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        if true {
            // check if query requests String
            request.provide_string(self.to_string());
        }

        request.push_link(("date", self.date));
        request.push_link(("time", self.time));
        request.push_link(("offset", self.offset));
    }
}

impl Data for Date {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_ref_unchecked(self);

        if true {
            // check if query requests String
            request.provide_string(self.to_string());
        }

        request.push_link(("year", self.year));
        request.push_link(("month", self.month));
        request.push_link(("day", self.day));
    }
}

impl Data for Time {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_ref_unchecked(self);

        if true {
            // check if query requests String
            request.provide_string(self.to_string());
        }

        request.push_link(("hour", self.hour));
        request.push_link(("minute", self.minute));
        request.push_link(("second", self.second));
        request.push_link(("nanosecond", self.nanosecond));
    }
}

impl Data for Offset {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_ref_unchecked(self);

        if true {
            // check if query requests String
            request.provide_string(self.to_string());
        }

        match *self {
            Self::Z => 0i16.query(request),
            Self::Custom { minutes } => minutes.query(request),
        }
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
        dbg!(&table);
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

        let items = value.as_items();

        // dbg!(&items);

        assert_eq!(items.len(), 3);
    }
}

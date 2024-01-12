use ::toml::{Table, Value as Val};

use crate::data::Data;
use crate::link_builder::{LinkBuilder, LinkBuilderExt};
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
    fn provide_links(&self, builder: &mut dyn LinkBuilder) {
        match self {
            Val::Table(table) => table.provide_links(builder),
            Val::Array(array) => array.provide_links(builder),
            _ => {
                builder.end().unwrap();
            }
        }
    }
}

impl Data for Table {
    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) {
        builder
            .extend(self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())))
            .unwrap();
        builder.end().unwrap();
    }
}

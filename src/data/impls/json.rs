use serde_json::{Map, Number, Value as Val};

use crate::data::Data;
use crate::id::ID;
use crate::link_builder::{LinkBuilder, LinkBuilderExt};
use crate::value::ValueBuiler;

impl Data for Val {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        match self {
            Val::Null => {}
            Val::Bool(b) => {
                builder.bool(*b);
            }
            Val::Number(n) => n.provide_value(builder),
            Val::String(s) => {
                builder.str(s.into());
            }
            Val::Array(_) => {}
            Val::Object(_) => {}
        }
    }

    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) {
        match self {
            Val::Array(v) => v.provide_links(builder),
            Val::Object(m) => m.provide_links(builder),
            _ => {
                builder.end().unwrap();
            }
        }
    }

    #[inline]
    fn get_id(&self) -> Option<ID> {
        match self {
            Val::Null => crate::well_known::NONE.get_id(),
            _ => None,
        }
    }
}

impl Data for Map<String, Val> {
    #[inline]
    fn provide_links(&self, builder: &mut dyn LinkBuilder) {
        builder
            .extend(self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())))
            .unwrap()
    }
}

impl Data for Number {
    #[inline]
    fn provide_value<'d>(&'d self, builder: &mut dyn ValueBuiler<'d>) {
        if let Some(n) = self.as_u64() {
            builder.u64(n);
        }
        if let Some(n) = self.as_i64() {
            builder.i64(n);
        }
        if let Some(n) = self.as_f64() {
            builder.f64(n);
        }
    }
}

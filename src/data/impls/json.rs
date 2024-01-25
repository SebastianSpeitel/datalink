use serde_json::{Map, Number, Value as Val};

use crate::data::Data;
use crate::id::ID;
use crate::links::{LinkError, Links, LinksExt};
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
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        match self {
            Val::Array(v) => v.provide_links(links),
            Val::Object(m) => m.provide_links(links),
            _ => Ok(()),
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
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.extend(self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())))?;
        Ok(())
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

use ::smol_str::SmolStr;

use crate::{Data, Request};

impl Data for SmolStr {
    #[inline]
    fn query(&self, request: impl Request) {
        self.as_str().query_owned(request);
    }

    #[inline]
    fn query_owned(self, request: impl Request) {
        self.as_str().query_owned(request);
    }
}

use crate::schema::Schema;
use crate::{Data, ErasableData, Link};

use super::visitor::Visitor;
use super::Request;

pub type SelfReceiver<'a> = Box<dyn FnMut(Box<dyn ErasableData>) + 'a>;
// pub type ErasedSchema = crate::schema::DataSchemaImpl<'static>;
pub type ErasedVisitor<'a> = Option<&'a mut dyn Visitor>;

#[derive(Debug)]
pub enum ErasedRequest<'r> {
    Borrowed(&'r mut (dyn ErasableRequest + 'r)),
    Owned(Box<dyn ErasableRequest + 'r>),
}

impl<'a> AsRef<dyn ErasableRequest + 'a> for ErasedRequest<'a> {
    #[inline]
    fn as_ref(&self) -> &(dyn ErasableRequest + 'a) {
        match self {
            Self::Borrowed(req) => *req,
            Self::Owned(req) => req.as_ref(),
        }
    }
}

impl<'a> AsMut<dyn ErasableRequest + 'a> for ErasedRequest<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut (dyn ErasableRequest + 'a) {
        match self {
            Self::Borrowed(req) => *req,
            Self::Owned(req) => req.as_mut(),
        }
    }
}

#[warn(clippy::missing_trait_methods)]
impl ErasableRequest for ErasedRequest<'_> {
    #[inline]
    fn erased_query(&self) -> &(dyn crate::ErasableData + '_) {
        self.as_ref().erased_query()
    }
    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        self.as_ref().erased_schema()
    }
    #[inline]
    fn erased_visitor(&mut self) -> ErasedVisitor {
        self.as_mut().erased_visitor()
    }
    #[inline]
    fn erased_visitor_for(&mut self, type_id: core::any::TypeId) -> ErasedVisitor {
        self.as_mut().erased_visitor_for(type_id)
    }
    #[inline]
    fn erased_self_receiver(&mut self) -> Option<SelfReceiver> {
        self.as_mut().erased_self_receiver()
    }
    #[inline]
    fn erased_link_request(&mut self) -> (Option<ErasedRequest>, Option<ErasedRequest>) {
        self.as_mut().erased_link_request()
    }
    #[inline]
    fn provide_erased_link(&mut self, key: &dyn ErasableData, target: &dyn ErasableData) {
        self.as_mut().provide_erased_link(key, target)
    }
}

pub trait ErasableRequest {
    #[inline]
    fn erased_query(&self) -> &(dyn crate::ErasableData + '_) {
        &() as _
    }

    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        crate::schema::SimpleSchema::ANY
    }
    fn erased_visitor(&mut self) -> ErasedVisitor;
    #[inline]
    fn erased_visitor_for(&mut self, type_id: core::any::TypeId) -> ErasedVisitor {
        let _ = type_id;
        self.erased_visitor()
    }
    #[inline]
    fn erased_self_receiver(&mut self) -> Option<SelfReceiver> {
        None
    }
    #[inline]
    fn erased_link_request(&mut self) -> (Option<ErasedRequest>, Option<ErasedRequest>) {
        (None, None)
    }

    #[inline]
    fn provide_erased_link(&mut self, _key: &dyn ErasableData, _target: &dyn ErasableData) {}
}

#[deny(clippy::missing_trait_methods)]
impl<R: ErasableRequest + ?Sized> ErasableRequest for &mut R {
    #[inline]
    fn erased_query(&self) -> &(dyn crate::ErasableData + '_) {
        (**self).erased_query()
    }

    #[inline]
    fn erased_schema(&self) -> crate::schema::SimpleSchema<'_> {
        (**self).erased_schema()
    }
    #[inline]
    fn erased_visitor(&mut self) -> ErasedVisitor {
        (**self).erased_visitor()
    }
    #[inline]
    fn erased_visitor_for(&mut self, type_id: core::any::TypeId) -> ErasedVisitor {
        (**self).erased_visitor_for(type_id)
    }
    #[inline]
    fn erased_link_request(&mut self) -> (Option<ErasedRequest>, Option<ErasedRequest>) {
        (**self).erased_link_request()
    }
    #[inline]
    fn erased_self_receiver(&mut self) -> Option<SelfReceiver> {
        (**self).erased_self_receiver()
    }
    #[inline]
    fn provide_erased_link(&mut self, key: &dyn ErasableData, target: &dyn ErasableData) {
        (**self).provide_erased_link(key, target)
    }
}

impl Request for dyn ErasableRequest + '_ {
    #[inline]
    fn query(&self) -> impl crate::Data {
        self.erased_query()
    }

    #[inline]
    fn schema(&self) -> impl Schema {
        self.erased_schema()
    }

    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        self.erased_visitor()
    }
    #[inline]
    fn visitor_for<T: 'static>(&mut self) -> impl Visitor {
        if !self.schema().accepts_value_of::<T>() {
            return None;
        }
        Some(self.erased_visitor_for(core::any::TypeId::of::<T>()))

        // use core::any::TypeId;
        // if self
        //     .validator()
        //     .validate_value_type(crate::r#type::TypeOf::<T>::new())
        // {
        //     Some(self.erased_visitor_for(TypeId::of::<T>()))
        // } else {
        //     None
        // }
    }
    #[allow(clippy::similar_names)]
    #[inline]
    fn provide_link_unchecked<L: Link>(&mut self, link: L) {
        


        let (mut req_k, mut req_t) = self.erased_link_request();

        let mut rec_k = req_k
            .as_mut()
            .and_then(ErasableRequest::erased_self_receiver);
        let mut rec_t = req_t
            .as_mut()
            .and_then(ErasableRequest::erased_self_receiver);

        if rec_k.is_none() && rec_t.is_none() {
            drop(rec_k);
            drop(rec_t);
            match (req_k, req_t) {
                (Some(r_k), Some(r_t)) => link.query(r_k, r_t),
                (Some(r_k), None) => link.query_key(r_k),
                (None, Some(r_t)) => link.query_target(r_t),
                (None, None) => {}
            }
            return;
        }

        match (&mut rec_k, &mut rec_t) {
            (Some(rec_k), Some(rec_t)) => {
                let (key, target) = link.into_tuple();
                rec_k(Box::new(key));
                rec_t(Box::new(target));
            }
            (Some(rec_k), None) => {
                drop(rec_t);
                if let Some(req_t) = req_t {
                    let (k, t) = link.into_tuple();
                    k.query_owned(req_t);
                    rec_k(Box::new(t));
                } else {
                    // If no request for target, just provide the key
                    rec_k(Box::new(link.into_key()));
                }
            }
            (None, Some(target_rec)) => {
                drop(rec_k);
                if let Some(req_k) = req_k {
                    let (k, t) = link.into_tuple();
                    k.query_owned(req_k);
                    target_rec(Box::new(t));
                } else {
                    // If no request for key, just provide the target
                    target_rec(Box::new(link.into_target()));
                }
            }
            (None, None) => {
                unreachable!("already handled");
            }
        }
    }
    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {
        self as &mut dyn ErasableRequest
    }
}

impl core::fmt::Debug for dyn ErasableRequest + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErasedRequest")
            .field("schema", &self.erased_schema())
            .field("query", &self.erased_query())
            .finish_non_exhaustive()
    }
}

impl Request for ErasedRequest<'_> {
    #[inline]
    fn query(&self) -> impl Data {
        self.as_ref().erased_query()
    }
    #[inline]
    fn schema(&self) -> impl Schema {
        self.as_ref().erased_schema()
    }
    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        self.as_mut().erased_visitor()
    }
    #[inline]
    fn visitor_for<T: 'static>(&mut self) -> impl Visitor {
        self.as_mut().visitor_for::<T>()
    }
    #[inline]
    fn provide_link_unchecked<L: Link>(&mut self, link: L) {
        self.as_mut().provide_link_unchecked(link);
    }
    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {
        self.as_mut()
    }
    #[inline]
    fn by_ref(&mut self) -> impl Request {
        self.as_mut()
    }
}

use crate::services::base::service::Service;

#[derive(Clone, Debug)]
pub(crate) struct Params {}

pub(crate) struct Response {

}

pub trait CreateDiscussionService: Service<Params, Response> + Sized {}

use crate::entities::user::User;
use crate::services::base::service::Service;

#[derive(Clone, Debug)]
pub(crate) struct Params {
    pub user: User,
    pub content: &'static str
}


pub(crate) struct Response {

}

pub trait CreateDiscussionService: Service<Params, Response> + Sized {}

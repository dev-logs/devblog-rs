use serde_derive::{Deserialize, Serialize};
use crate::entities::user::User;
use crate::services::base::service::{Service, VoidResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub blog_title: String,
    pub user: Option<User>
}

pub trait MarkReadService: Service<Params, VoidResponse> {}

use serde_derive::{Deserialize, Serialize};
use crate::entities::discussion::Discussion;
use crate::services::base::service::Service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub blog_title: Option<String>
}

pub trait GetDiscussionsService: Service<Params, Vec<Discussion>> {}

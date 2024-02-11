use serde_derive::{Deserialize, Serialize};
use crate::entities::discussion::Discussion;
use crate::services::base::service::Service;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub display_name: String,
    pub content: String,
    pub blog_title: String
}

pub trait CreateDiscussionService: Service<Params, Discussion> + Sized {}

use serde_derive::{Deserialize, Serialize};
use crate::entities::discussion::Discussion;
use crate::services::base::service::Service;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub email: String,
    pub content: String
}

pub trait CreateDiscussionService: Service<Params, Discussion> + Sized {}

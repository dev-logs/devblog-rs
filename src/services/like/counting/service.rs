use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::Service;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CountBlogLikeParams {
    pub title: String
}

pub trait CountBlogLikeService : Service<CountBlogLikeParams, u32> {}

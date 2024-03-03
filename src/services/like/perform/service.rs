use std::fmt::Debug;
use serde_derive::{Deserialize, Serialize};
use crate::entities::blog::Blog;
use crate::entities::like::Like;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;
use crate::services::base::service::Service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikeBlogParam {
    pub blog_title: String,
    pub display_name: Option<String>,
    pub count: u32
}

pub trait LikeBlogService: Service<LikeBlogParam, Relation<Like, User, Blog>> {}

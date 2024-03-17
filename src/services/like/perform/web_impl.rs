use surrealdb_id::relation::{LinkRelation};
use crate::api::web_controllers::like::like_blog::like_blog;
use crate::entities::blog::Blog;
use crate::entities::like::Like;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::perform::service::{LikeBlogParam, LikeBlogService};

pub struct LikeBlogServiceWebImpl {}

impl Service<LikeBlogParam, LinkRelation<User, Like, Blog>> for LikeBlogServiceWebImpl {
    async fn execute(self, params: LikeBlogParam) -> Resolve<LinkRelation<User, Like, Blog>> {
        Ok(like_blog(params).await?)
    }
}

impl LikeBlogService for LikeBlogServiceWebImpl {}
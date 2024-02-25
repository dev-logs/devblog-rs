use crate::api::web_controllers::like::like_blog::like_blog;
use crate::entities::blog::Blog;
use crate::entities::like::Like;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::service::{LikeBlogParam, LikeBlogService};

pub struct LikeBlogServiceWebImpl {}

impl Service<LikeBlogParam, Relation<Like, User, Blog>> for LikeBlogServiceWebImpl {
    async fn execute(self, params: LikeBlogParam) -> Resolve<Relation<Like, User, Blog>> {
        Ok(like_blog(params).await?)
    }
}

impl LikeBlogService for LikeBlogServiceWebImpl {}
use crate::api::web_controllers::like::counting::count_blog_like;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::counting::service::{CountBlogLikeParams, CountBlogLikeService};

pub struct CountBlogLikeWebImpl {

}

impl Service<CountBlogLikeParams, u32> for CountBlogLikeWebImpl {
    async fn execute(self, params: CountBlogLikeParams) -> Resolve<u32> {
        Ok(count_blog_like(params).await?)
    }
}

impl CountBlogLikeService for CountBlogLikeWebImpl {

}
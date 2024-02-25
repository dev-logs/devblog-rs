use leptos::*;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::entities::like::Like;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;
use crate::services::like::counting::service::{CountBlogLikeService, CountBlogLikeParams};

#[server(CountBlogLike, "/web")]
pub async fn count_blog_like(params: CountBlogLikeParams) -> Result<u32, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let service = ApiInjector::service_injector().get_counting_like_blog_service();

    Ok(service.execute(params).await?)
}

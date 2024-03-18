use leptos::*;
use surrealdb_id::relation::LinkRelation;
use crate::entities::blog::Blog;
use crate::entities::like::Like;
use crate::entities::user::User;
use crate::services::like::perform::service::LikeBlogParam;

#[server(LikeBlog, "/web")]
pub async fn like_blog(params: LikeBlogParam) -> Result<LinkRelation<User, Like, Blog>, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let like_blog_service = ApiInjector::service_injector().get_like_blog_service();

    Ok(like_blog_service.execute(params).await?)
}

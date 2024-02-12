use leptos::*;
use crate::entities::discussion::Discussion;
use crate::services::get_discussions::service::Params;

#[server(GetDiscussions, "/web")]
pub async fn get_discussions(params: Params) -> Result<Vec<Discussion>, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let discussion_service = ApiInjector::service_injector().get_get_discussions_service();

    let discussions = discussion_service.execute(params).await?;
    Ok(discussions)
}

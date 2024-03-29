use leptos::*;
use crate::entities::discussion::Discussion;
use crate::services::create_discussion::service::Params;

#[server(CreateDiscussion, "/web")]
pub async fn create_discussion(params: Params) -> Result<Discussion, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let discussion_service = ApiInjector::service_injector().get_create_discussion_service();

    let created_discussion = discussion_service.execute(params).await?;
    Ok(created_discussion)
}

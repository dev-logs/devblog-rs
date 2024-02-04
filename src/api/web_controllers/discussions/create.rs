use leptos::{server, ServerFnError};
use crate::core_services::api_di::*;
use crate::entities::discussion::Discussion;
use crate::services::create_discussion::service::Params;
use crate::services::base::service::*;

#[server(CreateDiscussion, "/web")]
pub async fn create_discussion(create_discussion_params: Params) -> Result<Discussion, ServerFnError> {
    let di = ApiInjector::service_injector();
    let discussion_service = di.get_create_discussion_service();

    let created_discussion = discussion_service.execute(create_discussion_params).await?;
    Ok(created_discussion)
}

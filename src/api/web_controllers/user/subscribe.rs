use leptos::{server, ServerFnError};
use crate::services::base::service::VoidResponse;
use crate::services::subscribe::service::Params;

#[server(Subscribe, "/web")]
pub async fn subscribe_user(params: Params) -> Result<VoidResponse, ServerFnError> {
    use crate::services::base::service::Service;
    use crate::services::subscribe::service::SubscribeService;
    use crate::core_services::api_di::*;

    let service = ApiInjector::service_injector().get_subscribe_service();
    Ok(service.execute(params).await?)
}

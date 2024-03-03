use leptos::*;
use crate::services::blog_detail::read::mark_read_service::Params;

#[server(MarkRead, "/web")]
pub async fn mark_read(params: Params) -> Result<(), ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let service = ApiInjector::service_injector().get_mark_read_service();

    let result = service.execute(params).await?;
    Ok(result)
}

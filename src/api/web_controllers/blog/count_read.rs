use leptos::*;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::services::blog_detail::count_read::service::Params;

#[server(CountRead, "/web")]
pub async fn count_read(params: Params) -> Result<usize, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let service = ApiInjector::service_injector().get_count_read_service();

    let result = service.execute(params).await?;
    Ok(result)
}

use leptos::*;
use web_sys::MouseEvent;
use crate::core_services::web_di::*;
use crate::services::blog_detail::min_read::service::Params;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::components::blogs::blog_item_container::BlogItemContainer;
use crate::web::components::blogs::blog_item_description::BlogItemDescription;
use crate::web::components::blogs::blog_item_title::BlogItemTitle;
use crate::web::components::read_more_button::ReadMoreButton;

#[component]
pub fn DeployFlutterWebBlogItem() -> impl IntoView {
    let blog = WebInjector::service_injector().get_blog_service().deploy_flutter_web();

    view! {
        <BlogItemContainer class="p-8 rounded-2xl justify-start" blog=blog>
        </BlogItemContainer>
    }
}

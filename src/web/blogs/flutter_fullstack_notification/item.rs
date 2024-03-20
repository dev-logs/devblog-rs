use leptos::*;
use crate::core_services::web_di::*;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::components::blogs::blog_item_container::BlogItemContainer;

#[component]
pub fn FlutterAndBackendNotificationManagementItem() -> impl IntoView {
    let blog = WebInjector::service_injector().get_blog_service().flutter_and_backend_notification_management();

    view! {
        <div class="glow-capture relative">
            <BlogItemContainer class="group p-8 rounded-2xl justify-start bg-zinc-900/50 opacity-90 border-2 border-white/5 rounded-2xl p-10 glow glow:ring-1 glow:border-glow glow:ring-glow glow:bg-glow/[.15]" blog=blog>
            </BlogItemContainer>
            <div class="glow-overlay" style="--glow-color: #717378"></div>
        </div>
    }
}

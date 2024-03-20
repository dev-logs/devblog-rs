use leptos::*;
use crate::core_services::web_di::*;
use crate::entities::blog::Blog;
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::app_context::signal_context::AppContextProvider;
use crate::web::components::blogs::blog_title::BlogTitle;
use crate::web::components::blogs::blog_image::BlogImage;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;

#[component]
fn Header(
    #[prop()]
    blog: Blog
) -> impl IntoView {
    view! {
        <BlogTitle class="">
            <BlogImage src="/assets/images/document/computer1.jpg" blended=true/>
        </BlogTitle>
    }
}

#[component]
pub fn FlutterFullStackPage() -> impl IntoView {
    let blogs_provider = WebInjector::service_injector().get_blog_service();
    BlogPostContext::new(blogs_provider.flutter_and_backend_notification_management()).attach();

    let blog_context = use_context::<BlogPostContext>().unwrap();
    let blog = blog_context.get_selected_blog().clone();

    view! {

    }
}

use leptos::*;
use web_sys::MouseEvent;
use crate::core_services::web_di::*;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::components::blogs::blog_item_container::BlogItemContainer;
use crate::web::components::blogs::blog_item_description::BlogItemDescription;
use crate::web::components::blogs::blog_item_title::BlogItemTitle;
use crate::web::components::read_more_button::ReadMoreButton;

#[component]
pub fn DeployFlutterWebBlogItem() -> impl IntoView {
    let blog = WebInjector::service_injector().get_blog_service().deploy_flutter_web();
    let title = blog.title.clone();
    let description = blog.description.clone();

    let on_item_clicked: Callback<MouseEvent> = (move |_| {
        let window = web_sys::window().expect("Window should be available");
        let location = window.location();
        let _ = location.set_href("/blogs/deploy-flutter-web");
    }).into();

    view! {
        <BlogItemContainer onclick=on_item_clicked class="bg-gray-900 p-8 rounded-2xl justify-start">
            <BlogItemTitle>{title}</BlogItemTitle>
            <BlogItemDescription class="mt-8">{description}</BlogItemDescription>
            <img class="rounded-2xl my-20 aspect-auto" src="assets/images/document/web-cicd.jpeg"/>
            <div class="flex flex-row justify-end">
                <ReadMoreButton onclick=on_item_clicked class="right-4 top-2">Read more</ReadMoreButton>
            </div>
        </BlogItemContainer>
    }
}

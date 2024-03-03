use leptos::*;
use web_sys::MouseEvent;
use crate::services::base::service::Service;
use crate::core_services::web_di::*;
use crate::entities::blog::Blog;
use crate::services::blog_detail::min_read::service::Params;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::components::blogs::blog_item_description::BlogItemDescription;
use crate::web::components::blogs::blog_item_title::BlogItemTitle;
use crate::web::components::read_more_button::ReadMoreButton;

#[component]
pub fn BlogItemContainer(
    #[prop()]
    blog: Blog,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = "{}")]
    style: &'static str
) -> impl IntoView {
    let title = blog.title.clone();
    let description = blog.description.clone();

    let on_item_clicked: Callback<MouseEvent> = (move |_| {
        let window = web_sys::window().expect("Window should be available");
        let location = window.location();
        let _ = location.set_href("/blogs/deploy-flutter-web");
    }).into();

    let min_read_action = {
        let blog = blog.clone();
        create_action(move |e: &()| {
            let blog = blog.clone();
            async move {
                let service = WebInjector::service_injector().get_count_read_minutes_service();
                service.execute(Params {blog}).await.unwrap()
            }
        })
    };

    create_effect(move |e| {
       min_read_action.dispatch(());
    });

    view! {
        <div on:click={on_item_clicked.clone()} class={format!("cursor-pointer flex flex-col m-5 {}", class)} style={style}>
            <div class="body flex flex-col">
                <BlogItemTitle>{title}</BlogItemTitle>
                <BlogItemDescription class="mt-8">{description}</BlogItemDescription>
                <div class="flex flex-row justify-between mt-4">
                    <div>
                        <p class="font-main text-md">{move || {min_read_action.value().get().unwrap_or(0)}} mins read</p>
                    </div>
                    <ReadMoreButton onclick=on_item_clicked class="right-4 top-2">Read more</ReadMoreButton>
                </div>
            </div>
        </div>
    }
}

mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::core_services::web_di::*;
use crate::services::create_discussion::service::*;
use crate::web::discussion::discussion::UserDiscussion;
use crate::services::base::service::Service;
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::discussion::edittext::{DiscussionSubmitEvent, EditText};

#[component]
pub fn Discussion () -> impl IntoView {
    let context = use_context::<BlogPostContext>().expect("Expect inside a blog to be comment");
    let create_discussion = create_action(move |event: &DiscussionSubmitEvent| {
        let current_blog = context.get_selected_blog();

        WebInjector::service_injector().get_create_discussion_service().execute(
            Params {
                content: event.content.clone(),
                display_name: String::from("tiendvlp"),
                blog_title: String::from(&current_blog.title)
            }
        )
    });

    let callback: Callback<DiscussionSubmitEvent> = Callback::new(move |e: DiscussionSubmitEvent| {
        create_discussion.dispatch(e);
    });

    return view! {
        <div class="flex flex-col antialiased w-full">
            <EditText callback=callback/>
            <UserDiscussion/>
        </div>
    }
}

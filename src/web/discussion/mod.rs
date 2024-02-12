mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::core_services::web_di::*;
use crate::services::create_discussion::service::*;
use crate::web::discussion::discussion::UserDiscussion;
use crate::services::base::service::{Resolve, Service};
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::components::modals::login_modal::LoginModal;
use crate::web::discussion::edittext::{DiscussionSubmitEvent, EditText};
use crate::web::local_storage::user::UserStorage;

#[component]
pub fn Discussion () -> impl IntoView {
    let context = use_context::<BlogPostContext>().expect("Expect inside a blog to be comment");
    let (show_login_modal, set_show_login_modal) = create_signal(false);

    let create_discussion = create_action(move |event: &(DiscussionSubmitEvent, String)| {
        let current_blog = context.get_selected_blog();

        WebInjector::service_injector().get_create_discussion_service().execute(
            Params {
                content: (&event.0.content).to_string(),
                display_name: String::from(&event.1),
                blog_title: String::from(&current_blog.title)
            }
        )
    });

    let callback: Callback<DiscussionSubmitEvent> = Callback::new(move |e: DiscussionSubmitEvent| {
        let user_storage = UserStorage::new();
        if user_storage.data.is_none() {
            set_show_login_modal(true);
        }
        else {
            create_discussion.dispatch((e, user_storage.data.unwrap().display_name));
        }
    });

    return view! {
        <div class="flex flex-col antialiased w-full">
            {move || view!{<LoginModal is_show={show_login_modal.get()}/>}}
            <EditText callback=callback/>
            <UserDiscussion/>
        </div>
    }
}

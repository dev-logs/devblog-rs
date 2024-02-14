mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::core_services::web_di::*;
use crate::services::create_discussion::service::*;
use crate::services::base::service::{Service};
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::components::modals::login_modal::LoginModal;
use crate::web::discussion::edittext::{DiscussionSubmitEvent, EditText};
use crate::web::local_storage::user::UserStorage;
use crate::services::get_discussions::service::Params as GetDiscussionParams;

#[component]
pub fn Discussion () -> impl IntoView {
    let context = use_context::<BlogPostContext>().expect("Expect inside a blog to be comment");
    let (show_login_modal, set_show_login_modal) = create_signal(false);

    let create_discussion = create_action(move |event: &(String, String, String)| {
        WebInjector::service_injector().get_create_discussion_service().execute(
            Params {
                content: (&event.2).to_string(),
                blog_title: event.0.clone(),
                display_name: String::from(&event.1),
            }
        )
    });

    let fetch_all_discussions = create_action(|title: &String| {
        let title = title.clone();
        (move || async {
            WebInjector::service_injector().get_get_discussions_service().execute(GetDiscussionParams {
                blog_title: Some(title)
            }).await
        })()
    });

    {
        let context = context.clone();
        let fetch_all_discussions = fetch_all_discussions.clone();
        create_effect(move |_| {
           fetch_all_discussions.dispatch(context.clone().get_selected_blog().title.clone());
        });
    }

    let callback = {
        let context = context.clone();
        let blog = context.get_selected_blog().clone();
        Callback::new(move |e: DiscussionSubmitEvent| {
            let user_storage = UserStorage::new();
            if user_storage.data.is_none() {
                set_show_login_modal(true);
            } else {
                create_discussion.dispatch((blog.title.clone(), e.content, user_storage.data.unwrap().display_name));
            }
        })
    };

    return view! {
        <div class="flex flex-col antialiased w-full">
            {move || view!{<LoginModal is_show={show_login_modal.get()}/>}}
            <EditText callback=callback/>
            {move || {
                let value = fetch_all_discussions.value();
                if let Some(result) = value.get() {
                    view! {
                        <div></div>
                    }
                }
                else {
                    view! {
                        <div></div>
                    }
                }
            }}
        </div>
    }
}

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
use crate::web::discussion::discussion::UserDiscussion;

#[component]
pub fn Discussion () -> impl IntoView {
    let context = use_context::<BlogPostContext>().expect("Expect inside a blog to be comment");
    let (show_login_modal, set_show_login_modal) = create_signal(false);

    let fetch_all_discussions = create_action(|title: &String| {
        let title = title.clone();
        (move || async {
            WebInjector::service_injector().get_get_discussions_service().execute(GetDiscussionParams {
                blog_title: title
            }).await
        })()
    });

    let create_discussion = {
        let fetch_all_discussions = fetch_all_discussions.clone();
        let context = context.clone();

        create_action(move |event: &(String, String, String)| {
            let event = event.clone();
            let context = context.clone();
            async move {
                let result = WebInjector::service_injector().get_create_discussion_service().execute(
                    Params {
                        content: event.1.clone(),
                        blog_title: event.0.clone(),
                        display_name: String::from(event.2),
                    }
                ).await;

                fetch_all_discussions.dispatch(context.get_selected_blog().title.clone());
                result
            }
        })
    };

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
        <div id="Discussions" class="flex flex-col antialiased w-full">
            {move || view!{<LoginModal is_show={show_login_modal.get()}/>}}
            <EditText callback=callback/>
            {move || {
                let value = fetch_all_discussions.value();
                if let Some(result) = value.get() {
                    if let Ok(discussions) = result {
                        if discussions.is_empty() {
                            view! {
                                <div><p class="text-xl font-main-bold">There is no discussions yet, make your first discussion</p></div>
                            }
                        }
                        else {
                            view! {
                                <div>
                                    {discussions.iter().map(|discussion| view! {
                                        <UserDiscussion discussion={discussion.clone()}/>
                                    }).collect_view()}
                                </div>
                            }
                        }
                    }
                    else {
                        view! {
                            <div><p class="text-xl font-main-bold">Failed to load discussions</p></div>
                        }
                    }
                }
                else {
                    view! {
                        <div><p class="text-xl font-main-bold">Loading</p></div>
                    }
                }
            }}
        </div>
    }
}

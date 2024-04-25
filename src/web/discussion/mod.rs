mod discussion;
mod edittext;
mod user_name;

use crate::core_services::surrealdb::user_tbl::UserId;
use crate::core_services::web_di::*;
use crate::services::base::service::{PagingParam, Service};
use crate::services::create_discussion::service::*;
use crate::services::get_discussions::service::Params as GetDiscussionParams;
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::components::modals::login_modal::LoginModal;
use crate::web::components::pagination::pagination::Pagination;
use crate::web::discussion::discussion::UserDiscussion;
use crate::web::discussion::edittext::{DiscussionSubmitEvent, EditText};
use crate::web::local_storage::user::UserStorage;
use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn Discussion() -> impl IntoView {
    let context = use_context::<BlogPostContext>().expect("Expect inside a blog to be comment");
    let (show_login_modal, set_show_login_modal) = create_signal(false);
    let (paging_param, set_paging_param) = create_signal(PagingParam { page: 1 });

    let fetch_next_discussions = {
        let paging_param = paging_param.clone();
        let title = context.get_selected_blog().title.clone();

        create_action(move |_: &()| {
            let title = title.clone();
            let paging_param = paging_param.clone();
            async move {
                WebInjector::service_injector()
                    .get_get_discussions_service()
                    .execute(GetDiscussionParams {
                        blog_title: title,
                        paging: paging_param.get(),
                    })
                    .await
            }
        })
    };

    let next_page: Callback<MouseEvent> = {
        let paging_param = paging_param.clone();
        Callback::new(move |_| {
            let total_page = fetch_next_discussions
                .value()
                .get_untracked()
                .unwrap()
                .unwrap()
                .total_page;
            let page = paging_param.get_untracked().page + 1;
            if page <= total_page {
                set_paging_param(PagingParam { page });

                fetch_next_discussions.dispatch(());
            }
        })
    };

    let prev_page: Callback<MouseEvent> = {
        let paging_param = paging_param.clone();
        let fetch_next_discussions = fetch_next_discussions.clone();
        Callback::new(move |_| {
            let page = paging_param.get_untracked().page - 1;
            if page >= 1 {
                set_paging_param(PagingParam { page });

                fetch_next_discussions.dispatch(());
            }
        })
    };

    let create_discussion = {
        let fetch_all_discussions = fetch_next_discussions.clone();

        create_action(move |event: &(String, String, String)| {
            let event = event.clone();
            async move {
                let result = WebInjector::service_injector()
                    .get_create_discussion_service()
                    .execute(Params {
                        content: event.1.clone(),
                        blog_title: event.0.clone(),
                        display_name: String::from(event.2),
                    })
                    .await;

                fetch_all_discussions.dispatch(());
                result
            }
        })
    };

    {
        let fetch_next_discussions = fetch_next_discussions.clone();
        create_effect(move |_| {
            fetch_next_discussions.dispatch(());
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
                create_discussion.dispatch((
                    blog.title.clone(),
                    e.content,
                    user_storage.data.unwrap().display_name,
                ));
            }
        })
    };

    return view! {
        <div id="Discussions" class="flex flex-col antialiased w-full">
            {move || view!{<LoginModal is_show={show_login_modal.get()}/>}}
            <EditText callback=callback/>
            {move || {
                let value = fetch_next_discussions.value();
                if let Some(result) = value.get() {
                    if let Ok(discussions) = result { view!{
                        <div>
                            {if discussions.data.is_empty() {
                                view! {
                                    <div><p class="text-xl font-main.jsx-bold">There is no discussions yet, make your first discussion</p></div>
                                }
                            }
                            else {
                                view! {
                                    <div>
                                        {discussions.data.iter().map(|discussion| view! {
                                            <UserDiscussion discussion={discussion.clone()}/>
                                        }).collect_view()}
                                    </div>
                                }
                            }}
                            <Pagination class="pt-10" total={discussions.total_record} current_page={discussions.page.clone()} on_next=next_page on_prev=prev_page/>
                        </div>
                    }}
                    else {
                        view! {
                            <div><p class="text-xl font-main.jsx-bold">Failed to load discussions</p></div>
                        }
                    }
                }
                else {
                    view! {
                        <div><p class="text-xl font-main.jsx-bold">Loading</p></div>
                    }
                }
            }}
        </div>
    };
}

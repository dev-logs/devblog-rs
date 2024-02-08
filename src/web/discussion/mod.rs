mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::core_services::web_di::*;
use crate::services::create_discussion::service::*;
use crate::web::discussion::discussion::UserDiscussion;
use crate::services::base::service::Service;
use crate::web::discussion::edittext::{DiscussionSubmitEvent, EditText};

#[component]
pub fn Discussion () -> impl IntoView {
    let create_discussion = create_action(|event: &DiscussionSubmitEvent| {
        WebInjector::service_injector().get_create_discussion_service().execute(Params {
            content: event.content.clone(),
            email: String::from("tiendvlp@gmail.com")
        })
    });

    let x2 = create_discussion.clone();
    let x3: Callback<DiscussionSubmitEvent> = Callback::new(move |e: DiscussionSubmitEvent| {
        x2.dispatch(e);
    });

    return view! {
        <div class="flex flex-col antialiased w-full">
            <EditText callback=x3/>
            <UserDiscussion/>
        </div>
    }
}

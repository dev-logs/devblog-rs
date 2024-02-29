use chrono::{DateTime, Utc};
use leptos::*;
use crate::core_services::date_time::display_local_time;
use crate::entities::discussion::{Discussion};
use crate::entities::user::User;

#[component]
pub fn UserDiscussion(
    #[prop()]
    discussion: Discussion
) -> impl IntoView {

    view! {
        <div class="flex flex-col">
            <Header user={(*discussion.owner.clone()).clone()} datetime={discussion.created_at.clone()}/>
            <p class="text-gray-500 dark:text-gray-400">{discussion.content}</p>
            <div class="flex items-center mt-4 space-x-4">
                <button
                    type="button"
                    class="flex items-center text-sm text-gray-500 hover:underline dark:text-gray-400 font-medium">
                    <svg class="mr-1.5 w-3.5 h-3.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5h5M5 8h2m6-3h2m-5 3h6m2-7H2a1 1 0 0 0-1 1v9a1 1 0 0 0 1 1h3v5l5-5h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1Z"/>
                    </svg>
                    Reply
                </button>
            </div>
        </div>
    }
}

#[component]
fn Header(
    user: User,
    datetime: DateTime<Utc>
) -> impl IntoView {
    view! {
        <div class="flex items-center">
            <p class="inline-flex items-center mr-3 text-sm text-gray-900 dark:text-gray-200 font-semibold">
                <img class="mr-2 w-10 h-10 aspect-square rounded-full" src={&user.avatar_url.unwrap_or(String::from(""))} alt="No avatar"/>
                {&user.display_name}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">
               <time pubdate datetime="2022-02-08" title="February 8th, 2022">{display_local_time(datetime)}</time>
            </p>
        </div>
    }
}

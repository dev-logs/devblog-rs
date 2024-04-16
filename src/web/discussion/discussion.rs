use chrono::{DateTime, Utc};
use leptos::*;
use crate::core_services::date_time::display_local_time;
use crate::entities::discussion::{Discussion};
use crate::entities::user::User;
use crate::js_context;

#[component]
pub fn UserDiscussion(
    #[prop()]
    discussion: Discussion
) -> impl IntoView {

    view! {
        <div class="flex flex-col">
            <Header user={(*discussion.owner).clone()} datetime={discussion.created_at.clone()}/>
            <p class="text-gray-500 dark:text-gray-400">{discussion.content}</p>
        </div>
    }
}

#[component]
fn Header(
    user: User,
    datetime: DateTime<Utc>
) -> impl IntoView {
    let js_code = js_context! ({
        import THREE from "three"
        const mesh = new THREE.BasicMesh(geometry, material);
        mesh.position
    }, {});

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

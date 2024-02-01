mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::web::components::discussion::discussion::UserDiscussion;
use crate::web::components::discussion::edittext::EditText;

#[component]
pub fn Discussion () -> impl IntoView {
    return view! {
        <div class="flex flex-col antialiased w-full">
            <EditText/>
            <UserDiscussion/>
        </div>
    }
}

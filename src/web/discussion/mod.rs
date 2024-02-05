mod user_name;
mod edittext;
mod discussion;

use leptos::*;
use crate::web::discussion::discussion::UserDiscussion;
use crate::web::discussion::edittext::EditText;

#[component]
pub fn Discussion () -> impl IntoView {
    return view! {
        <div class="flex flex-col antialiased w-full">
            <EditText callback=(move |e| {})/>
            <UserDiscussion/>
        </div>
    }
}

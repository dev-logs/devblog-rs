use leptos::*;
use crate::entities::author::Author;
use crate::entities::user::User;

#[component]
pub fn AuthorAvatar(
    author: Author
) -> impl IntoView {
    return view! {
        <div class="flex flex-row items-center text-gray-200">
          <a class="bg-primaryC py-2 px-3 font-bold rounded-xl">Subscribe @devlog.studio</a>
        </div>
    }
}

use leptos::*;
use crate::entities::author::Author;

#[component]
pub fn AuthorAvatar(
    author: Author
) -> impl IntoView {
    return view! {
        <div class="flex flex-row items-center text-gray-200">
          <a class="bg-primaryC py-2 px-3 font-main.jsx-bold rounded-xl">Subscribe @devlog.studio</a>
        </div>
    }
}

use leptos::*;

#[component]
pub fn UserName(
    #[prop(default = "")]
    user_name: &'static str,
    #[prop(default = "")]
    avatar_url: &'static str
) -> impl IntoView {
    view! {
        <div class="flex items-center">
            <p class="inline-flex items-center mr-3 text-sm text-gray-900 dark:text-white font-semibold">
                <img class="mr-2 w-6 h-6 rounded-full" src=avatar_url alt=user_name/>
                {user_name}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">
               <time pubdate datetime="2022-02-08" title="February 8th, 2022">Feb. 8, 2022</time>
            </p>
        </div>
    }
}
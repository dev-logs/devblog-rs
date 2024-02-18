use leptos::*;

#[component]
pub fn SearchHeader() -> impl IntoView {
    view! {
       <div class="flex flex-row justify-start space-x-1/6 w-full pt-8 pl-4 pb-2 pr-1.5">
            <a href="/" class="flex flex-row items-center">
                <img class="w-10 rounded" src="/assets/images/ic_devlog.png"/>
                <p class="font-main-bold ml-2 text-xl">Devlog Studio</p>
            </a>
            <form class="flex flex-row bg-gray-500 border border-gray-800 p-2 rounded-lg w-50 bg-onBackgroundC">
                <input type="search" id="default-search" class="block w-full p-2 ps-10 text-sm placeholder-gray-400 text-black bg-transparent" placeholder="Anything..."/>
                <button type="submit" class="ml-4 text-gray-200 w-44 font-black rounded-lg text-sm bg-secondaryC text-onSecondaryC">Search</button>
            </form>
            <div class="empty"></div>
        </div>
    }
}
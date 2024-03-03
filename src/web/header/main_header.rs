use leptos::*;
use crate::web::home::navigation::{HomeNavigationTab, NavigationItem};

#[component()]
pub fn MainHeader(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    view! {
       <nav class=format!("flex flex-row justify-around space-x-1/6 w-full border-gray-200 px-4 h-24 lg:px-6 py-4 bg-gray-950 {class}")>
            <a href="/" class="flex flex-row items-center">
                <img class="w-10 rounded" src="/assets/images/ic_devlog.png"/>
                <div class="flex flex-col">
                    <p class="font-main ml-2 text-lg">Devlog Studio</p>
                </div>
            </a>
            <form class="flex flex-row bg-gray-500 border border-gray-800 p-2 max-h-14 rounded-lg w-50 bg-onBackgroundC">
                <input type="search" id="default-search" class="block w-full p-2 ps-10 text-sm placeholder-gray-400 text-black bg-transparent" placeholder="Anything..."/>
                <button type="submit" class="ml-4 text-gray-200 w-44 font-black rounded-lg text-sm bg-secondaryC text-onSecondaryC">Search</button>
            </form>
            <div class="menu flex flex-row h-20">
                <NavigationItem tab={HomeNavigationTab::Blog}/>
                <NavigationItem tab={HomeNavigationTab::Products}/>
            </div>
        </nav>
    }
}

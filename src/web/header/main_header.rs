use leptos::*;
use crate::web::components::supported::cracking::CrackingBackground;
use crate::web::home::navigation::{HomeNavigationTab, NavigationItem};

#[component()]
pub fn MainHeader(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    view! {
        <star-sky-3d class="flex flex-col justify-between" style:height="40vh">
        <nav class=format!("flex flex-row justify-around items-center space-x-1/6 w-full border-gray-200 px-4 h-24 lg:px-6 py-4 {class}")>
            <a href="/" class="flex flex-row items-center justify-center">
                <img class="w-10 rounded" src="/assets/images/ic_devlog.png"/>
                <div class="flex flex-col">
                    <p class="font-main ml-2 text-lg">Devlog Studio</p>
                </div>
            </a>
            <form class="sm:flex sm:flex-row justify-center items-center hidden bg-gray-500 border border-gray-800 p-2 max-h-14 rounded-lg w-50 bg-onBackgroundC">
                <input type="search" id="default-search" class="block w-full p-2 ps-10 text-sm placeholder-gray-400 text-black bg-transparent" placeholder="Anything..."/>
                <button type="submit" class="ml-4 text-gray-200 w-44 font-black rounded-lg text-sm bg-secondaryC text-onSecondaryC py-2">Search</button>
            </form>
            <div class="menu flex flex-row h-20 justify-center items-center">
                <NavigationItem tab={HomeNavigationTab::Blog}/>
                <NavigationItem tab={HomeNavigationTab::Products}/>
            </div>
            <script type="module" src="/assets/js/threejs/index.js"></script>
        </nav>
        <CrackingBackground width="100%" height="250px"/>
        </star-sky-3d>
    }
}

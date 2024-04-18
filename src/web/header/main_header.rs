use leptos::*;
use leptos::leptos_dom::log;
use serde_json::json;
use crate::entities::discussion::Discussion;
use crate::web::components::supported::cracking::CrackingBackground;
use crate::web::home::navigation::{HomeNavigationTab, NavigationItem};

#[component()]
pub fn MainHeader(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    view! {
         <nav class=format!("flex flex-row justify-around items-center space-x-1/6 w-full border-gray-200 px-4 h-24 lg:px-6 py-4 {class}")>
            <a href="/" class="flex flex-row items-center justify-center">
                <img class="w-10 rounded" src="/assets/images/ic_devlog.png"/>
                <div class="flex flex-col">
                    <p class="font-main-bold ml-2 text-lg text-blue-600">Devlog Studio</p>
                </div>
            </a>
            <div class="menu flex flex-row h-20 justify-center items-center">
                <NavigationItem tab={HomeNavigationTab::Blog}/>
                <NavigationItem tab={HomeNavigationTab::Products}/>
            </div>
        </nav>
    }
}

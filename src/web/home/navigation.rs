use std::collections::HashMap;
use std::fmt::Debug;
use leptos::*;
use web_sys::js_sys::eval;
use crate::core_services::web_di::*;
use crate::{js_context, include_js};
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::components::author_avatar::AuthorAvatar;
use crate::web::components::icons::light_bulb::LightBulb;

#[derive(Clone, Debug)]
pub enum HomeNavigationTab {
    Blog,
    Products,
    Services
}

impl Default for HomeNavigationTab {
    fn default() -> Self {
        Self::Blog
    }
}

#[component]
pub fn NavigationItem(
    tab: HomeNavigationTab,
) -> impl IntoView {
    let navigation_context = use_context::<HomeNavigationSignalContext>().unwrap();

    let icon = match tab {
        HomeNavigationTab::Blog => {
            let mut context = navigation_context.clone();
            view! {
                <div class="flex flex-row items-center w-full justify-between p-3">
                    <button class=format!("font-main text-md item-{:?}", tab) on:click=move |_| {context.set_tab(HomeNavigationTab::Blog)}>Blogs</button>
                </div>
            }
        }
        HomeNavigationTab::Products => {
            let mut context = navigation_context.clone();
            view! {
                <div class="flex flex-row items-center w-full justify-between p-3">
                    <button class=format!("font-main text-md item-{:?}", tab) on:click=move |_| {context.set_tab(HomeNavigationTab::Products)}>Products</button>
                </div>
            }
        }
        HomeNavigationTab::Services => {
            let mut context = navigation_context.clone();
            view! {
                <div class="flex flex-row items-center pt-4 pb-1/4 w-full justify-between p-3">
                    <button class=format!("item-{:?}", tab) on:click=move |_| {context.set_tab(HomeNavigationTab::Services)} class="text-xl font-black">Services</button>
                    <LightBulb color="white"/>
                </div>
            }
        }
    };

    view! {
       <div class="container flex flex-row items-center justify-start relative z-10">
           {icon}
       </div>
    }
}

#[component]
pub fn HomeNavigation() -> impl IntoView {
    let navigation_context = use_context::<HomeNavigationSignalContext>().unwrap();
    let selected_tab = navigation_context.signal.read();

    let provider = WebInjector::service_injector();
    let authors_provider = provider.get_author_service();

    let mut context = HashMap::<&str, String>::new();
    context.insert("selectedTab", format!("{:?}", selected_tab.get_untracked()));
    create_effect(move |_| {
        eval(js_context! ({
           let selected_item_selector = b".item-${context.selected_tab}";
           let animation = {
                y: y - initialHandRect.top,
                duration: 0.1,
                ease: "bounce.inOut"
            };

            const hand = document.querySelector(".selection-hand");
            hand.initialRect = hand.initialRect || hand.getBoundingClientRect()
            const initialHandRect = hand.initialRect
            const selectedElement = document.querySelector("selected_item_selector")
            const rect = selectedElement.getBoundingClientRect()
            const y = rect.top + rect.height / 2 - initialHandRect.height / 2
            gsap.to(".selection-hand", animation)
        }, context).as_str()).unwrap();
    });

    view! {
        <div class="navigation-header flex flex-col justify-between pl-8 pr-8 pt-10 h-full bg-secondaryC max-w-220px" style="max-width=200px">
            <AuthorAvatar author={authors_provider.tiendang()}/>
            <div class="navigation-body flex-col mt-10">
                <NavigationItem tab={HomeNavigationTab::Blog}/>
                <NavigationItem tab={HomeNavigationTab::Products}/>
                <NavigationItem tab={HomeNavigationTab::Services}/>
                <div class="selection-hand relative z-0 justify-start over w-full rounded-2xl bg-gray-800 h-50px"/>
            </div>
            <div class="navigation-footer mb-10">
            </div>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.5/gsap.min.js">
            </script>
        </div>
    }
}

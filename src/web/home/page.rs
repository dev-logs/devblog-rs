use leptos::*;
use crate::core_services::web_di::*;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::app_context::signal_context::UseAppSignal;
use crate::web::blogs::deploy_flutter_web::item::DeployFlutterWebBlogItem;
use crate::web::home::header::SearchHeader;
use crate::web::home::navigation::{HomeNavigation, HomeNavigationTab};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-screen h-screen w-full flex flex-row bg-primaryC">
            <div class="navigation-bar-wrapper h-full">
                <HomeNavigation/>
            </div>
            <div class="home-content-wrapper flex flex-col pl-8">
                <Content/>
            </div>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    let home_navigation_context = use_context::<HomeNavigationSignalContext>().unwrap();

    let ContentView = move || match home_navigation_context.read().get() {
        HomeNavigationTab::Blog => {
            view! {
                <div>
                    <BlogList/>
                </div>
            }
        }
        HomeNavigationTab::Products => {
            view! {
                <div></div>
            }
        }
        HomeNavigationTab::Products => {
            view! {
                <div></div>
            }
        }
        _ => {
            view! {
                <div></div>
            }
        }
    };

    view! {
        <div class="flex-col justify-center w-full h-full pt-2">
            <SearchHeader/>
            {ContentView}
        </div>
    }
}

#[component]
fn BlogList() -> impl IntoView {
    let di = WebInjector::service_injector();
    let provider = di.get_blog_service();
    let blogs = provider.list();

    view! {
        <div class="grid lg:grid-cols-1 xl:grid-cols-4 gap-5 mt-8">
            {
                blogs.into_iter().map(|_|
                    view! {
                        <DeployFlutterWebBlogItem/>
                    }
                ).collect_view()
            }
        </div>
    }
}
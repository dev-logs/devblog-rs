use leptos::*;
use crate::core_services::web_di::*;
use crate::js_context;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::blogs::deploy_flutter_web::item::DeployFlutterWebBlogItem;
use crate::web::home::navigation::HomeNavigationTab;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-screen h-screen flex flex-row w-full justify-center items-start bg-backgroundC">
            <Content/>
            <script type="module" src="/assets/js/support/index.js"></script>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    let home_navigation_context = use_context::<HomeNavigationSignalContext>().unwrap();

    let ContentView = move || match home_navigation_context.signal.read().get() {
        HomeNavigationTab::Blog => {
            view! {
                <div class="w-full h-full">
                    <r3f-header-1/>
                </div>
            }
        }
        HomeNavigationTab::Products => {
            view! {
                <div></div>
            }
        }
        HomeNavigationTab::Services=> {
            view! {
                <div></div>
            }
        }
    };

    view! {
        {ContentView}
    }
}

#[component]
fn BlogList() -> impl IntoView {
    let provider = WebInjector::service_injector().get_blog_service();
    let blogs = provider.list();

    view! {
        <div class="flex flex-col justify-start max-w-2xl h-full">
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
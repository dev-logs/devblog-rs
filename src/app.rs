use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::app_context::signal_context::UseAppSignal;
use crate::web::blogs::deploy_flutter_web::page::DeployFlutterWebPage;
use crate::web::home::page::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    HomeNavigationSignalContext::attach();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/blogs/deploy-flutter-web" view=DeployFlutterWebPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial api-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the api
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a api function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

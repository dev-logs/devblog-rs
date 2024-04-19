use leptos::*;
use leptos::leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::json;
use crate::read_json;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::app_context::signal_context::{AppContextProvider};
use crate::web::blogs::deploy_flutter_web::page::DeployFlutterWebPage;
use crate::web::footer::main_footer::MainFooter;
use crate::web::home::page::Home;
use crate::web::header::main_header::MainHeader;

#[derive(Serialize, Deserialize, Debug)]
struct ViteBuildReport {
   js_file: String
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    HomeNavigationSignalContext::new().attach();
    let report_file = include_str!("../3d/build-report.json");
    let build_report: ViteBuildReport = serde_json::from_str(report_file).expect("Failed to read vite build result");
    log!("Vite build info {:?}", build_report);

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <MainHeader/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/blogs/deploy-flutter-web" view=DeployFlutterWebPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
                <script type="module" src={build_report.js_file}></script>
            </main>
        </Router>
        <MainFooter/>
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

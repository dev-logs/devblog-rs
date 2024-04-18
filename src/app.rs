use leptos::*;
use leptos::leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use serde_json::json;
use crate::read_json;
use crate::web::app_context::home_navigation_context::HomeNavigationSignalContext;
use crate::web::app_context::signal_context::{AppContextProvider};
use crate::web::blogs::deploy_flutter_web::page::DeployFlutterWebPage;
use crate::web::footer::main_footer::MainFooter;
use crate::web::home::page::Home;
use crate::web::header::main_header::MainHeader;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    HomeNavigationSignalContext::new().attach();
    let import_map = include_str!("../assets/js/react/deno.json");
    // let import_map = json! ({
    //     "imports": {
    //         "three": "https://esm.sh/three@0.162.0",
    //         "three/": "https://esm.sh/three@0.162.0/",
    //         "@react-three/fiber": "https://esm.sh/@react-three/fiber@8.15.11?deps=three@0.162.0",
    //         "@react-three/drei": "https://esm.sh/@react-three/drei@9.105.4?deps=@react-three/fiber@8.15.11&deps=react@18.2.0",
    //         "htm": "https://esm.sh/htm",
    //         "react": "https://esm.sh/react@18.2.0?deps=react-dom@18.2.0",
    //         "react-dom": "https://esm.sh/react-dom?deps=react@18.2.0"
    //     }
    // }).to_string();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <Script type_="importmap">
            {import_map}
        </Script>
        <MainHeader/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/blogs/deploy-flutter-web" view=DeployFlutterWebPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
                <script type="module" src="/assets/js/react/index.js"></script>
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

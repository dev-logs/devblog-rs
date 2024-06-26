use leptos::logging::log;
use web_sys::HtmlElement;
use devblog_rs::services::base::service::Resolve;
use devblog_rs::services::base::service::Service;
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use devblog_rs::app::*;
    use devblog_rs::core_services::surrealdb::connect::connect_surrealdb;

    // connect_surrealdb().await;

    // start_migration().await.map_err(|e| {
    //     log!("Failed to perform migration {:?}", e)
    // }).unwrap();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    log!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/web/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve source code of 3d project under buildDir 3d-dist
            .service(Files::new("/assets-3d", format!("{site_root}/3d-dist")))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
async fn start_migration() -> Resolve<()> {
    use devblog_rs::services::migration_services::service::BlogPostMigrationParams;
    use devblog_rs::services::migration_services::service::AuthorMigrationParams;
    use devblog_rs::core_services::api_di::ApiServicesInjector;
    use devblog_rs::core_services::api_di::ApiInjector;

    let ns = "api-migration";
    let blog_migration = ApiInjector::service_injector().get_blog_migration_service(ns);
    let author_migration = ApiInjector::service_injector().get_author_migration_service(ns);

    author_migration.execute(AuthorMigrationParams {}).await?;
    blog_migration.execute(BlogPostMigrationParams {}).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main.jsx function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main.jsx function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use devblog_rs::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

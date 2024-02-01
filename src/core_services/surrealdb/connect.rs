use log::{info, log};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use crate::api::environments::ENVIRONMENT;
use crate::core_services::surrealdb::DB;

pub async fn connect_surrealdb() {
    let namespace: &str = "api-connect-surrealdb";
    info!(target: namespace, "Connecting to SurrealDB...");

    DB.connect::<Ws>(ENVIRONMENT.surreal_db.socket_address.clone()).await.expect("Failed while connecting to surreal db");
    DB.use_ns(ENVIRONMENT.surreal_db.namespace.clone()).use_db(ENVIRONMENT.surreal_db.db_name.clone()).await.unwrap();
    DB.signin(Root {
        username: ENVIRONMENT.surreal_db.db_username.clone().as_str(),
        password: ENVIRONMENT.surreal_db.db_password.clone().as_str()
    }).await.unwrap();

    let db_version = DB.version().await.expect("Failed to get the surreal db version");

    info!(target: namespace, "Connected to SurrealDB {} {}", db_version, ENVIRONMENT.surreal_db.socket_address);
}
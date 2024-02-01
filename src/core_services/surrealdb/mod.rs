use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use once_cell::sync::Lazy;
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
pub mod connect;
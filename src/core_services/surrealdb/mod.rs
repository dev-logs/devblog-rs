use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use once_cell::sync::Lazy;

pub type Db = Surreal<Client>;
pub static DB: Lazy<Db> = Lazy::new(Surreal::init);
pub mod connect;
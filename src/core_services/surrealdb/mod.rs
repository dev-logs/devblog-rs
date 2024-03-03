use surrealdb::Surreal;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::Client;

#[cfg(feature = "ssr")]
pub type Db = Surreal<Client>;
#[cfg(feature = "ssr")]
pub static DB: Lazy<Db> = Lazy::new(Surreal::init);
#[cfg(feature = "ssr")]
pub mod connect;
pub mod adaptive_relation;
pub mod user_tbl;
pub mod blog_tbl;
pub mod author_tbl;
#[cfg(feature="ssr")]
pub mod result_handler;

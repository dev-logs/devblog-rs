use surrealdb::Surreal;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::RecordId;

#[cfg(feature = "ssr")]
pub type Db = Surreal<Client>;
#[cfg(feature = "ssr")]
pub static DB: Lazy<Db> = Lazy::new(Surreal::init);
#[cfg(feature = "ssr")]
pub mod connect;
pub mod user_tbl;
pub mod blog_tbl;
pub mod author_tbl;
#[cfg(feature="ssr")]
pub mod result_handler;
pub mod discussion_relation;
pub mod view_relation;

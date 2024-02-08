pub mod web_di;
#[cfg(feature = "ssr")]
pub mod api_di;
pub mod surrealdb;
#[cfg(feature = "ssr")]
pub mod logger;

pub mod web_di;
#[cfg(feature = "ssr")]
mod api_di;
#[cfg(feature = "ssr")]
pub mod surrealdb;
#[cfg(feature = "ssr")]
pub mod errors;
#[cfg(feature = "ssr")]
pub mod logger;

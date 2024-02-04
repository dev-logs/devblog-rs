use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
#[cfg(feature = "ssr")]
use surrealdb::opt::RecordId;
#[cfg(feature = "ssr")]
use surrealdb::sql::{Id, Thing};
use crate::entities::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Discussion {
    pub owner: User,
    pub content: String,
    pub created_at: DateTime<Utc>
}

#[cfg(feature = "ssr")]
impl Into<Thing> for Discussion {
    fn into(self) -> Thing {
        RecordId::from(("discussion", Id::uuid()))
    }
}
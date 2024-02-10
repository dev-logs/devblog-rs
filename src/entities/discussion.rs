use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::blog::Blog;
use crate::entities::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Discussion {
    pub id: RecordId,
    pub owner: AdaptiveRelation<User>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub blog: AdaptiveRelation<Blog>
}

#[cfg(feature = "ssr")]
impl Into<Thing> for Discussion {
    fn into(self) -> Thing {
        self.id.clone()
    }
}

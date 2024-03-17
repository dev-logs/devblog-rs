use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;
use surrealdb_id::link::Link;
use crate::core_services::surrealdb::discussion_relation::DiscussionId;
use crate::entities::blog::Blog;
use crate::entities::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Discussion {
    pub owner: Link<User>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub blog: Link<Blog>
}

#[cfg(feature = "ssr")]
impl Into<Thing> for Discussion {
    fn into(self) -> Thing {
        DiscussionId(self.created_at).into()
    }
}

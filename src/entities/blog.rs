use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;
use surrealdb_id::link::Link;
use crate::core_services::surrealdb::blog_tbl::BlogId;
use crate::entities::author::Author;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Blog {
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub author: Link<Author>
}

impl Blog {
    pub fn new (
        url: &'static str,
        title: &'static str,
        description: &'static str,
        author: Author,
        created_at: DateTime<Utc>
    ) -> Self {

        Self {
            url: url.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            created_at,
            author: Link::<Author>::Record(author),
        }
    }
}

impl Into<Thing> for Blog {
    fn into(self) -> RecordId {
        BlogId { title: self.title }.into()
    }
}

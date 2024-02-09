use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::author::Author;
use crate::entities::user::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Blog {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub author: Author
}

impl Blog {
    pub fn new (
        url: &'static str,
        title: &'static str,
        description: &'static str,
        author: Author,
        created_at: DateTime<Utc>
    ) -> Self {
        let id = AdaptiveRelation::<Blog>::new().id();
        Self {
            id,
            url: url.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            created_at,
            author,
        }
    }
}

impl Into<RecordId> for Blog {
    fn into(self) -> RecordId {
        self.id.clone()
    }
}

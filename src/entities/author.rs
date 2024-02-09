use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Author {
    pub id: RecordId,
    pub email: String,
    pub avatar_url: String,
    pub full_name: String,
    pub display_name: Option<String>
}

impl Into<RecordId> for Author {
    fn into(self) -> RecordId {
        AdaptiveRelation::<Author>::new(&self.email).id()
    }
}

impl Author {
    pub fn new (email: &str, avatar_url: &str, full_name: &str, display_name: Option<String>) -> Self {
        let id = AdaptiveRelation::<Author>::new(email).id();
        Self {
            id,
            email: email.to_string(),
            avatar_url: avatar_url.to_string(),
            full_name: full_name.to_string(),
            display_name,
        }
    }
}

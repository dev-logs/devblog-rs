use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct User {
    pub email: String,
    pub id: RecordId
}

impl User {
    pub fn new (email: &str) -> Self {
        let id = AdaptiveRelation::<User>::new(email.clone());
        Self {
            email: email.to_string(),
            id: id.into()
        }
    }
}

impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        self.id.clone()
    }
}

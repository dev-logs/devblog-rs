use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct User {
    pub id: RecordId,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>
}

impl User {
    pub fn new (email: Option<&str>, avatar_url: Option<&str>, display_name: &str) -> Self {
        let id = AdaptiveRelation::<User>::new(display_name);
        Self {
            email: email.map(|e| e.to_owned()),
            avatar_url: avatar_url.map(|v| v.to_owned()),
            id: id.into(),
            display_name: display_name.to_string()
        }
    }
}

impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        self.id.clone()
    }
}

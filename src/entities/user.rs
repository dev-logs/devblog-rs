use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
#[cfg(feature = "ssr")]
use surrealdb::opt::RecordId;


#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct User {
    pub email: String
}

impl User {
    pub fn new (email: &str) -> Self {
        Self {
            email: email.to_string()
        }
    }
}

#[cfg(feature = "ssr")]
impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        RecordId::from(("user", self.email.as_str()))
    }
}

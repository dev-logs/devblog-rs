use crate::core_services::surrealdb::author_tbl::AuthorId;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Author {
    pub email: String,
    pub avatar_url: String,
    pub full_name: String,
    pub display_name: Option<String>,
}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        let other_id: RecordId = AuthorId {
            email: other.email.to_string(),
        }
        .into();
        let my_id: RecordId = AuthorId {
            email: self.email.to_string(),
        }
        .into();

        my_id == other_id
    }

    fn ne(&self, other: &Self) -> bool {
        !other.eq(self)
    }
}

impl Into<RecordId> for Author {
    fn into(self) -> RecordId {
        AuthorId {
            email: self.email.clone(),
        }
        .into()
    }
}

impl Author {
    pub fn new(
        email: &str,
        avatar_url: &str,
        full_name: &str,
        display_name: Option<String>,
    ) -> Self {
        Self {
            email: email.to_string(),
            avatar_url: avatar_url.to_string(),
            full_name: full_name.to_string(),
            display_name,
        }
    }
}

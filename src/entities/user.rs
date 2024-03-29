use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::user_tbl::UserId;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct User {
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>
}

impl User {
    pub fn new (email: Option<&str>, avatar_url: Option<&str>, display_name: &str) -> Self {
        Self {
            email: email.map(|e| e.to_owned()),
            avatar_url: avatar_url.map(|v| v.to_owned()),
            display_name: display_name.to_string()
        }
    }

    pub fn noone() -> Self {
        Self {
            email: None,
            avatar_url: Some("/assets/images/avatars/avatar_9.png".to_owned()),
            display_name: "noone".to_string()
        }
    }

    pub fn is_noone(&self) -> bool {
        self.display_name.eq("noone")
    }

    pub fn is_someone(&self) -> bool {
        !self.is_noone()
    }
}

impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        UserId { display_name: self.display_name.clone() }.into()
    }
}

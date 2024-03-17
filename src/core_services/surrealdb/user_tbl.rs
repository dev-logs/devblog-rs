use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb_id::link::{Link, NewLink};
use crate::entities::user::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId {
    pub display_name: String
}

impl From<UserId> for RecordId {
    fn from(value: UserId) -> Self {
        Self::from(("user", value.display_name.as_str()))
    }
}

impl NewLink<User, String> for Link<User> {
    fn new(params: String) -> Link<User> {
        Link::<User>::from(UserId { display_name: params })
    }
}
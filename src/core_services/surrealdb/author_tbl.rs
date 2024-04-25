use crate::entities::author::Author;
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb_id::link::Link;

pub trait NewAuthorId {
    fn new(email: String) -> Link<Author>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorId {
    pub email: String,
}

impl From<AuthorId> for RecordId {
    fn from(value: AuthorId) -> Self {
        RecordId::from(("author", value.email.as_str()))
    }
}

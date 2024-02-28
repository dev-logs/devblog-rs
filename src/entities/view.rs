use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {

}

impl Into<RecordId> for View {
    fn into(self) -> RecordId {
        ("view", Id::uuid()).into()
    }
}
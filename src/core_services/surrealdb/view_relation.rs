use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViewId {
   pub view_at: DateTime<Utc>
}

impl From<ViewId> for RecordId {
    fn from(value: ViewId) -> Self {
        Self::from(("view", Id::Number(value.view_at.timestamp_millis())))
    }
}

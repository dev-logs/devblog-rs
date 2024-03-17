use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscussionId(pub DateTime<Utc>);

impl From<DiscussionId> for RecordId {
    fn from(value: DiscussionId) -> Self {
        RecordId::from(("discuss", Id::Number(value.0.timestamp_millis())))
    }
}
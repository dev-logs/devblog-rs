use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct View {
    pub view_at: DateTime<Utc>
}

impl Into<RecordId> for View {
    fn into(self) -> RecordId {
        View { view_at: self.view_at }.into()
    }
}
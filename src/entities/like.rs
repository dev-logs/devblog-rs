use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::blog::Blog;
use crate::entities::relation::r#trait::IntoRelation;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Like {
    pub count: u32
}

impl Into<RecordId> for Like {
    fn into(self) -> RecordId {
        RecordId::from(("like", Id::uuid()))
    }
}

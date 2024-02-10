use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::user::User;

impl AdaptiveRelation<User> {
    pub fn new (display_name: &str) -> Self {
        Self::Id(RecordId::from(("user", display_name)))
    }
}

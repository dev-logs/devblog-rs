use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::user::User;

impl AdaptiveRelation<User> {
    pub fn new (email: &str) -> Self {
        Self::Id(RecordId::from(("user", email)))
    }
}

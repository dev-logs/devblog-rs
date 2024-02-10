use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::author::Author;

impl AdaptiveRelation<Author> {
    pub fn new (email: &str) -> Self {
        Self::Id(RecordId::from(("author", email)))
    }
}

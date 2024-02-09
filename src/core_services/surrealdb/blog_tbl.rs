use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::blog::Blog;

impl AdaptiveRelation<Blog> {
    pub fn new () -> Self {
        Self::Id(RecordId::from(("blog", Id::uuid())))
    }
}

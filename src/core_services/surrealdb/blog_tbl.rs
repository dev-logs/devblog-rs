use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::blog::Blog;

impl AdaptiveRelation<Blog> {
    pub fn new (title: &str) -> Self {
        Self::Id(RecordId::from(("blog", Id::String(title.to_string()))))
    }
}

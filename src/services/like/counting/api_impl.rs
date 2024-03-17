use serde_json::Value;
use surreal_derive_plus::surreal_quote;
use surrealdb_id::link::{Link, NewLink};
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::counting::service::{CountBlogLikeParams, CountBlogLikeService};

pub struct CountBlogLikeApiImpl {
    pub db: Db
}

impl Service<CountBlogLikeParams, u32> for CountBlogLikeApiImpl {
    async fn execute(self, params: CountBlogLikeParams) -> Resolve<u32> {
        let blog_relation = Link::<Blog>::new(params.title);
        let blog_id = blog_relation.id();

        let result: Option<Value> = self.db.query(surreal_quote!("SELECT out as blog, math::sum(count) as total_count from #val(&blog_id)<-like GROUP BY blog")).await?.take(0)?;
        if result.is_none() {
            return Ok(0);
        }

        let count = result.unwrap().as_object().unwrap().get("total_count").unwrap().as_u64();

        Ok(count.unwrap() as u32)
    }
}

impl CountBlogLikeService for CountBlogLikeApiImpl {

}
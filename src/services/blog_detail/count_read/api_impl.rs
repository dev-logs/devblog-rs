use surreal_derive_plus::surreal_quote;
use surrealdb_id::link::{Link, NewLink};
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::services::base::service::{Resolve, Service};
use crate::services::blog_detail::count_read::service::{CountReadService, Params};

pub struct CountReadServiceApiImpl {
    pub db: Db
}

impl Service<Params, usize> for CountReadServiceApiImpl {
    async fn execute(self, params: Params) -> Resolve<usize> {
        let blog_relation = Link::<Blog>::new(params.blog_title);
        let blog_id = blog_relation.id();
        let view_count: Option<usize>= self.db.query(surreal_quote!("COUNT(SELECT out FROM #val(&blog_id)<-view)")).await?.take(0)?;

        Ok(view_count.unwrap())
    }
}

impl CountReadService for CountReadServiceApiImpl {}

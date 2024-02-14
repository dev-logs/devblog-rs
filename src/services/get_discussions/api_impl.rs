use leptos::logging::log;
use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::services::base::service::{Resolve, Service};
use crate::services::get_discussions::service::{GetDiscussionsService, Params};

pub struct GetDiscussionsApiImpl {
    pub(crate) db: Db
}

impl Service<Params, Vec<Discussion>> for GetDiscussionsApiImpl {
    async fn execute(self, params: Params) -> Resolve<Vec<Discussion>> {
        let mut where_statement: String = "WHERE true".to_string();
        if let Some(title) = params.blog_title {
            let blog_relation = AdaptiveRelation::<Blog>::new(title.as_str());
            let blog_id = blog_relation.id();
            let query: String = surreal_quote!("blog = #val(&blog_id)");
            where_statement = format!("{where_statement} AND {query}")
        }

        let main_query = format!("SELECT * from discussion {where_statement} fetch owner");
        let discussions: Vec<Discussion> = self.db.query(main_query).await?.take(0)?;

        Ok(discussions)
    }
}

impl GetDiscussionsService for GetDiscussionsApiImpl {

}

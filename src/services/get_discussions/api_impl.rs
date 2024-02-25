use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::services::base::service::{Resolve, Service};
use crate::services::get_discussions::service::{GetDiscussionsService, Params};

pub struct GetDiscussionsApiImpl {
    pub(crate) db: Db
}

impl Service<Params, Vec<Discussion>> for GetDiscussionsApiImpl {
    async fn execute(self, params: Params) -> Resolve<Vec<Discussion>> {
        let blog_relation = AdaptiveRelation::<Blog>::new(params.blog_title.as_str());
        let blog_id = blog_relation.id();

        let main_query = surreal_quote!("SELECT *, in as owner from #val(&blog_id)<-discuss fetch owner");
        let discussions: Vec<Discussion> = self.db.query(main_query).await?.take(0)?;

        Ok(discussions)
    }
}

impl GetDiscussionsService for GetDiscussionsApiImpl {

}

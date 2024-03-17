use serde_json::Value;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb_id::link::Link;
use crate::core_services::surrealdb::blog_tbl::BlogId;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::services::base::service::{PageResponse, Resolve, Service};
use crate::services::get_discussions::service::{GetDiscussionsService, Params};

pub struct GetDiscussionsApiImpl {
    pub db: Db
}

impl Service<Params, PageResponse<Discussion>> for GetDiscussionsApiImpl {
    async fn execute(self, params: Params) -> Resolve<PageResponse<Discussion>> {
        let blog_relation = Link::<Blog>::from(BlogId { title: params.blog_title.clone() });
        let blog_id: RecordId = blog_relation.id();

        let row_per_page = 10;
        let total_page_query: Option<Value> = self.db.query(surreal_quote!("SELECT count(), out FROM #val(&blog_id)<-discuss group by out")).await?.take(0)?;
        let total_items = total_page_query.unwrap().as_object().unwrap().get("count").unwrap().as_i64().unwrap() as i32;
        let total_page: f64 = total_items as f64 / row_per_page as f64;
        let total_page = total_page.ceil() as i32;
        let start_index = row_per_page * (params.paging.page - 1);

        let main_query = surreal_quote!(r##"
            SELECT *, in as owner from #val(&blog_id)<-discuss
            ORDER BY created_at DESC
            LIMIT #row_per_page START #start_index
            FETCH owner
        "##);

        let discussions: Vec<Discussion> = self.db.query(main_query).await?.take(0)?;

        let response = PageResponse {
            data: discussions,
            page: params.paging.page.clone(),
            total_page,
            total_record: total_items,
        };

        Ok(response)
    }
}

impl GetDiscussionsService for GetDiscussionsApiImpl {

}

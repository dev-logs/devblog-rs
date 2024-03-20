use leptos::logging::log;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb::Response;
use surrealdb::sql::Thing;
use crate::core_services::surrealdb::Db;
use crate::core_services::surrealdb::result_handler::UniformSurrealResult;
use crate::entities::blog::Blog;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::migration_services::service::{BlogPostMigrationParams, BlogPostMigrationService};

pub struct BlogPostMigrationServiceImpl<T> where T: BlogProviderService {
    pub db: Db,
    pub post_provider: T,
    pub ns: String
}

impl<T> Service<BlogPostMigrationParams, VoidResponse> for BlogPostMigrationServiceImpl<T> where T: BlogProviderService {
    async fn execute(self, _: BlogPostMigrationParams) -> Resolve<VoidResponse> {
        let ns = format!("{}-blog-post-migration-service", self.ns);

        let all_posts = self.post_provider.list();
        let migrated_posts: Vec<Blog> = self.db
            .query("SELECT * FROM blog")
            .await?
            .take::<Vec<Blog>>(0)?;

        let not_migrated_posts: Vec<&Blog> = all_posts
            .iter()
            .filter(|post|
                migrated_posts.iter().find(|migrated_post| {
                    Into::<RecordId>::into(migrated_post.clone())
                        .eq(&post.clone().into())
                }).is_none())
            .collect();

        if not_migrated_posts.is_empty() {
            log!("{ns} All blogs has been migrated successfully");
            return Ok(())
        }

        log!( "{ns} Migrating blog post into db {:?}", not_migrated_posts);
        let mut migrated_response: Response = self.db
            .query(surreal_quote!("CREATE #multi(&not_migrated_posts)"))
            .await?;

        let result_handler = UniformSurrealResult::<Blog>::try_from(&mut migrated_response)?;
        let complete_migrated_posts = result_handler.0;

        log!( "{ns} Migrated posts: {:?}", &complete_migrated_posts);

        Ok(())
    }
}

impl<T> BlogPostMigrationService for BlogPostMigrationServiceImpl<T> where T: BlogProviderService  {}

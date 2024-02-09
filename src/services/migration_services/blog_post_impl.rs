use log::info;
use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::migration_services::service::{BlogPostMigrationParams, BlogPostMigrationService};

pub struct BlogPostMigrationServiceImpl<T> where T: BlogProviderService {
    db: Db,
    post_provider: T,
    ns: String
}

impl<T> Service<BlogPostMigrationParams, VoidResponse> for BlogPostMigrationServiceImpl<T> where T: BlogProviderService {
    async fn execute(self, params: BlogPostMigrationParams) -> Resolve<VoidResponse> {
        let ns = format!("{}-blog-post-migration-service", self.ns);

        let all_posts = self.post_provider.list();
        let migrated_posts: Option<Vec<Blog>> = self.db.query("SELECT * FROM blog").await?.take(0)?;
        let not_migrated_posts: Vec<&Blog> = all_posts
            .iter()
            .filter(|post|
                migrated_posts.as_ref().unwrap().iter().find(|migrated_post| migrated_post.id.eq(&post.id)).is_none())
            .collect();

        info!(target: &ns, "Migrating blog post into db {:?}", not_migrated_posts);
        let complete_migrated_posts: Option<Vec<Blog>> = self.db
            .query(surreal_quote!("CREATE #array(&not_migrated_posts)"))
            .await?
            .take(0)?;

        info!(target: &ns, "Migrated posts: {:?}", &complete_migrated_posts.expect("Failed to migrate posts"));

        Ok(())

    }
}

impl<T> BlogPostMigrationService for BlogPostMigrationServiceImpl<T> where T: BlogProviderService  {}
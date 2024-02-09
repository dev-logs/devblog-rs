use log::info;
use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::Db;
use crate::entities::author::Author;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::migration_services::service::{AuthorMigrationParams, AuthorMigrationService};

pub struct AuthorMigrationServiceImpl<T> where T: AuthorProviderService {
    pub author_provider_service: T,
    pub db: Db,
    pub ns: String,
}

impl<T> Service<AuthorMigrationParams, VoidResponse> for AuthorMigrationServiceImpl<T> where T: AuthorProviderService {
    async fn execute(self, params: AuthorMigrationParams) -> Resolve<VoidResponse> {
        let ns = format!("{}-author-migration", self.ns);
        let all_authors = self.author_provider_service.list();
        let migrated_authors : Option<Vec<Author>> = self.db
            .query(surreal_quote!("SELECT * FROM author"))
            .await?
            .take(0)?;

        let not_migrated_authors: Vec<&Author> = all_authors.iter().filter(|author| {
            migrated_authors.as_ref().unwrap().iter().find(|migrated_author| migrated_author.id.eq(&author.id)).is_none()
        }).collect();

        info!(target: ns.as_str(), "Start migrating for authors: {:?}", &not_migrated_authors);

        let complete_authors: Option<Vec<Author>> = self.db.query(surreal_quote!(r#"
            BEGIN TRANSACTION
            CREATE #array(&not_migrated_authors)
            COMMIT TRANSACTION
        "#)).await?.take(0)?;

        info!(target: &ns, "Complete migrated authors: {:?}", complete_authors.expect("Failed to migrate authors"));

        Ok(())
    }
}

impl<T> AuthorMigrationService for AuthorMigrationServiceImpl<T> where T: AuthorProviderService {}
use std::ops::Deref;
use leptos::logging::log;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb::Response;
use crate::core_services::surrealdb::Db;
use crate::core_services::surrealdb::result_handler::UniformSurrealResult;
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
    async fn execute(self, _: AuthorMigrationParams) -> Resolve<VoidResponse> {
        let ns = format!("{}-author-migration", self.ns);
        let all_authors = self.author_provider_service.list();
        let migrated_authors: Vec<Author> = self.db
            .query(surreal_quote!("SELECT * FROM author"))
            .await?
            .take::<Vec<Author>>(0)?;

        let not_migrated_authors: Vec<&Author> = all_authors.iter().filter(|author| {
            migrated_authors.iter().find(|migrated_author| Into::<RecordId>::into(migrated_author.clone()).eq(&Into::<RecordId>::into(author.clone()))).is_none()
        }).collect();

        if not_migrated_authors.is_empty() {
            log!( "{ns} All authors has been migrated successfully");
            return Ok(())
        }

        log!("{ns} Start migrating for authors: {:?}", &not_migrated_authors);

        let mut complete_authors: Response = self.db.query(surreal_quote!(r#"
            CREATE #multi(&not_migrated_authors)
        "#)).await?;

        let result_handler = UniformSurrealResult::<Author>::try_from(&mut complete_authors)?;
        let complete_migrated_authors = result_handler.0;

        log!( "{ns}Complete migrated authors: {:?}", complete_migrated_authors);

        Ok(())
    }
}

impl<T> AuthorMigrationService for AuthorMigrationServiceImpl<T> where T: AuthorProviderService {}
use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::core_services::surrealdb::DB;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::api_impl::CreateDiscussionApiImpl;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::migration_services::author_impl::AuthorMigrationServiceImpl;
use crate::services::migration_services::blog_post_impl::BlogPostMigrationServiceImpl;
use crate::services::migration_services::service::{AuthorMigrationService, BlogPostMigrationService};

pub trait ApiServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;

    fn get_author_migration_service(&self, ns: &str) -> impl AuthorMigrationService;

    fn get_blog_migration_service(&self, ns: &str) -> impl BlogPostMigrationService;
}

pub struct ApiInjector;

impl ApiServicesInjector for ApiInjector {
    fn get_author_service(&self) -> impl AuthorProviderService {
        AuthorProviderServiceImpl::new()
    }

    fn get_blog_service(&self) -> impl BlogProviderService {
        BlogProviderServiceImpl::new(self.get_author_service())
    }

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService {
        CreateDiscussionApiImpl { db: DB.clone() }
    }

    fn get_author_migration_service(&self, ns: &str) -> impl AuthorMigrationService {
        return AuthorMigrationServiceImpl {
            author_provider_service: self.get_author_service(),
            db: DB.clone(),
            ns: ns.to_string(),
        }
    }

    fn get_blog_migration_service(&self, ns: &str) -> impl BlogPostMigrationService {
        return BlogPostMigrationServiceImpl {
            post_provider: self.get_blog_service(),
            db: DB.clone(),
            ns: ns.to_string(),
        }
    }
}

static ADI: Lazy<ApiInjector> = Lazy::new(|| ApiInjector {});

impl ApiInjector {
    pub fn service_injector() ->  &'static impl ApiServicesInjector {
        ADI.deref()
    }
}

use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::core_services::surrealdb::DB;
use crate::core_services::web_di::{WebInjector};
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::api_impl::CreateDiscussionApiImpl;
use crate::services::create_discussion::service::CreateDiscussionService;

pub trait ApiServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;
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
}

static ADI: Lazy<ApiInjector> = Lazy::new(|| ApiInjector {});

impl ApiInjector {
    pub fn service_injector() ->  &'static impl ApiServicesInjector {
        ADI.deref()
    }
}


use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_discussion::web_impl::CreateDiscussionWebImpl;

pub trait WebServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;
}

pub struct WebInjector;

impl WebServicesInjector for WebInjector {
    fn get_author_service(&self) -> impl AuthorProviderService {
        AuthorProviderServiceImpl::new()
    }

    fn get_blog_service(&self) -> impl BlogProviderService {
        BlogProviderServiceImpl::new(self.get_author_service())
    }

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService {
        CreateDiscussionWebImpl {}
    }
}

impl WebInjector {
    pub fn service_injector() -> impl WebServicesInjector {
        Self {}
    }
}

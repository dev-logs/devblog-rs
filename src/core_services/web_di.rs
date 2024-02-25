use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_discussion::web_impl::CreateDiscussionWebImpl;
use crate::services::create_guess_user::service::{CreateGuestUserService, Params};
use crate::services::create_guess_user::web_impl::CreateGuestUserWebImpl;
use crate::services::get_discussions::service::GetDiscussionsService;
use crate::services::get_discussions::web_impl::GetDiscussionsWebImpl;

pub trait WebServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;

    fn get_get_discussions_service(&self) -> impl GetDiscussionsService;

    fn get_create_guest_user_service(&self) -> impl CreateGuestUserService;
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

    fn get_get_discussions_service(&self) -> impl GetDiscussionsService {
        GetDiscussionsWebImpl {}
    }

    fn get_create_guest_user_service(&self) -> impl CreateGuestUserService {
        CreateGuestUserWebImpl {}
    }
}

impl WebInjector {
    pub fn service_injector() -> &'static impl WebServicesInjector {
        WDI.deref()
    }
}

static WDI: Lazy<WebInjector> = Lazy::new(move || WebInjector {});

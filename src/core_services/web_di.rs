use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_detail::count_read::service::CountReadService;
use crate::services::blog_detail::count_read::web_impl::CountReadServiceWebImpl;
use crate::services::blog_detail::min_read::service::CountReadMinutesService;
use crate::services::blog_detail::min_read::web_impl::CountReadMinutesServiceWebImpl;
use crate::services::blog_detail::read::mark_read_service::MarkReadService;
use crate::services::blog_detail::read::web_impl::{MarkReadServiceWebImpl, MarkReadServiceWithTimeoutWebImpl};
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_discussion::web_impl::CreateDiscussionWebImpl;
use crate::services::create_guess_user::service::CreateGuestUserService;
use crate::services::create_guess_user::web_impl::CreateGuestUserWebImpl;
use crate::services::get_discussions::service::GetDiscussionsService;
use crate::services::get_discussions::web_impl::GetDiscussionsWebImpl;
use crate::services::like::counting::service::CountBlogLikeService;
use crate::services::like::counting::web_impl::CountBlogLikeWebImpl;
use crate::services::like::perform::service::LikeBlogService;
use crate::services::like::perform::web_impl::LikeBlogServiceWebImpl;
use crate::services::subscribe::service::SubscribeService;
use crate::services::subscribe::web_impl::SubscribeServiceWebImpl;

pub trait WebServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;

    fn get_get_discussions_service(&self) -> impl GetDiscussionsService;

    fn get_create_guest_user_service(&self) -> impl CreateGuestUserService;

    fn get_like_blog_service(&self) -> impl LikeBlogService;

    fn get_count_blog_like_service(&self) -> impl CountBlogLikeService;
    fn get_count_read_minutes_service(&self) -> impl CountReadMinutesService;
    fn get_mark_read_service_with_duration(&self) -> impl MarkReadService;

    fn get_mark_read_service(&self) -> impl MarkReadService;

    fn get_count_read_service(&self) -> impl CountReadService;

    fn get_subscribe_service(&self) -> impl SubscribeService;
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

    fn get_like_blog_service(&self) -> impl LikeBlogService {
        LikeBlogServiceWebImpl {}
    }

    fn get_count_blog_like_service(&self) -> impl CountBlogLikeService {
        CountBlogLikeWebImpl {}
    }

    fn get_count_read_minutes_service(&self) -> impl CountReadMinutesService {
        CountReadMinutesServiceWebImpl {}
    }

    fn get_mark_read_service_with_duration(&self) -> impl MarkReadService {
        return MarkReadServiceWithTimeoutWebImpl {}
    }

    fn get_mark_read_service(&self) -> impl MarkReadService {
        return MarkReadServiceWebImpl {}
    }

    fn get_count_read_service(&self) -> impl CountReadService {
        CountReadServiceWebImpl {}
    }

    fn get_subscribe_service(&self) -> impl SubscribeService {
        SubscribeServiceWebImpl {}
    }
}

impl WebInjector {
    pub fn service_injector() -> &'static impl WebServicesInjector {
        WDI.deref()
    }
}

static WDI: Lazy<WebInjector> = Lazy::new(move || WebInjector {});

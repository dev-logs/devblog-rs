use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::core_services::surrealdb::DB;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::author_provider_service::author_provider_impl::AuthorProviderServiceImpl;
use crate::services::blog_detail::count_read::api_impl::CountReadServiceApiImpl;
use crate::services::blog_detail::count_read::service::CountReadService;
use crate::services::blog_detail::read::api_impl::MarkReadServiceApiImpl;
use crate::services::blog_detail::read::mark_read_service::MarkReadService;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::services::blog_provider_service::blog_provider_service_impl::BlogProviderServiceImpl;
use crate::services::create_discussion::api_impl::CreateDiscussionApiImpl;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_guess_user::api_impl::CreateGuestUserApiImpl;
use crate::services::create_guess_user::service::CreateGuestUserService;
use crate::services::get_discussions::api_impl::GetDiscussionsApiImpl;
use crate::services::get_discussions::service::GetDiscussionsService;
use crate::services::like::counting::api_impl::CountBlogLikeApiImpl;
use crate::services::like::counting::service::CountBlogLikeService;
use crate::services::like::perform::api_impl::LikeBlogServiceApiImpl;
use crate::services::like::perform::service::LikeBlogService;
use crate::services::migration_services::author_impl::AuthorMigrationServiceImpl;
use crate::services::migration_services::blog_post_impl::BlogPostMigrationServiceImpl;
use crate::services::migration_services::service::{AuthorMigrationService, BlogPostMigrationService};
use crate::services::subscribe::api_impl::SubscribeServiceApiImpl;
use crate::services::subscribe::service::SubscribeService;
use crate::services::user_default_avatar::api_impl::RandomUserAvatarServiceApiImpl;
use crate::services::user_default_avatar::service::RandomUserDefaultAvatarService;

pub trait ApiServicesInjector {
    fn get_author_service(&self) -> impl AuthorProviderService;

    fn get_blog_service(&self) -> impl BlogProviderService;

    fn get_create_discussion_service(&self) -> impl CreateDiscussionService;

    fn get_author_migration_service(&self, ns: &str) -> impl AuthorMigrationService;

    fn get_blog_migration_service(&self, ns: &str) -> impl BlogPostMigrationService;

    fn get_create_guest_user_service(&self) -> impl CreateGuestUserService;

    fn get_get_discussions_service(&self) -> impl GetDiscussionsService;

    fn get_generate_random_avatar_url_service(&self) -> impl RandomUserDefaultAvatarService;

    fn get_like_blog_service(&self) -> impl LikeBlogService;

    fn get_counting_like_blog_service(&self) -> impl CountBlogLikeService;

    fn get_mark_read_service(&self) -> impl MarkReadService;

    fn get_count_read_service(&self) -> impl CountReadService;

    fn get_subscribe_service(&self) -> impl SubscribeService;
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
        CreateDiscussionApiImpl { db: DB.clone(), create_guess_user: self.get_create_guest_user_service() }
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

    fn get_create_guest_user_service(&self) -> impl CreateGuestUserService {
        return CreateGuestUserApiImpl { db: DB.clone(), random_avatar: self.get_generate_random_avatar_url_service() }
    }

    fn get_get_discussions_service(&self) -> impl GetDiscussionsService {
        return GetDiscussionsApiImpl { db: DB.clone() }
    }

    fn get_generate_random_avatar_url_service(&self) -> impl RandomUserDefaultAvatarService {
        RandomUserAvatarServiceApiImpl {}
    }

    fn get_like_blog_service(&self) -> impl LikeBlogService {
        LikeBlogServiceApiImpl {
            db: DB.clone()
        }
    }

    fn get_counting_like_blog_service(&self) -> impl CountBlogLikeService {
        CountBlogLikeApiImpl { db: DB.clone() }
    }

    fn get_mark_read_service(&self) -> impl MarkReadService {
        MarkReadServiceApiImpl { db: DB.clone() }
    }

    fn get_count_read_service(&self) -> impl CountReadService {
        return CountReadServiceApiImpl {
            db: DB.clone()
        }
    }

    fn get_subscribe_service(&self) -> impl SubscribeService {
        return SubscribeServiceApiImpl { db: DB.clone() }
    }
}

static ADI: Lazy<ApiInjector> = Lazy::new(|| ApiInjector {});

impl ApiInjector {
    pub fn service_injector() ->  &'static impl ApiServicesInjector {
        ADI.deref()
    }
}

use chrono::DateTime;
use crate::entities::blog::Blog;
use crate::services::author_provider_service::author_provider::AuthorProviderService;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;

pub struct BlogProviderServiceImpl <APS: AuthorProviderService> {
    author_provider: APS
}

impl<APS: AuthorProviderService> BlogProviderServiceImpl<APS> {
    pub fn new (author_provider: APS) -> Self {
        Self {
            author_provider
        }
    }
}

impl<APS: AuthorProviderService> BlogProviderService for BlogProviderServiceImpl<APS> {
    fn deploy_flutter_web(&self) -> Blog {
        return Blog::new(
            "blogs/deploy-flutter-web",
           "Deploy Flutter web with Github Action",
            "Flutter for Web is more stable and powerful, but you need to deploy it, here is how. Using Github Action, Docker and Digital Ocean.",
            self.author_provider.tiendang(),
            DateTime::default()
        );
    }

    fn flutter_and_backend_notification_management(&self) -> Blog {
        return Blog::new(
            "blogs/flutter-notification",
            "Flutter and backend notification management",
            "",
            self.author_provider.tiendang(),
            DateTime::default()
        )
    }

    fn list(&self) -> Vec<Blog> {
        vec![
            self.deploy_flutter_web(),
            self.flutter_and_backend_notification_management()
        ]
    }
}
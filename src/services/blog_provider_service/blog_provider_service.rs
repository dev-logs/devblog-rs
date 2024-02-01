use crate::entities::blog::Blog;

pub trait BlogProviderService {
    fn deploy_flutter_web(&self) -> Blog;

    fn list(&self) -> Vec<Blog>;
}
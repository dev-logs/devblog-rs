use crate::entities::blog::Blog;
use crate::web::app_context::signal_context::AppContext;

#[derive(Debug, Clone)]
pub struct BlogPostContext {
    blog: Blog
}

impl AppContext for BlogPostContext {}

impl BlogPostContext {
    pub fn new(selected_blog: Blog) -> Self {
        Self { blog: selected_blog }
    }

    pub fn get_selected_blog(&self) -> &Blog {
        &self.blog
    }
}

use crate::entities::blog::Blog;
use crate::web::app_context::signal_context::AppContext;

#[derive(Debug, Clone)]
struct BlogPostContext {
    blog: Blog
}

impl AppContext for BlogPostContext {}

impl BlogPostContext {
    pub fn new(selected_blog: Blog) -> Self {
        Self { blog: selected_blog }
    }
}

impl BlogPostContext {
}
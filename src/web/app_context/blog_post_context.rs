use leptos::{create_signal, SignalGet};
use crate::entities::blog::Blog;
use crate::web::app_context::signal_context::{AppContext, AppSignal};

#[derive(Debug, Clone)]
pub struct BlogPostContext {
    blog: AppSignal<Blog>
}

impl AppContext for BlogPostContext {}

impl BlogPostContext {
    pub fn new(selected_blog: Blog) -> Self {
        let (read, write) = create_signal(selected_blog);
        Self { blog: AppSignal::new(read, write) }
    }

    pub fn get_selected_blog(&self) -> Blog {
       self.blog.read().get().clone()
    }
}

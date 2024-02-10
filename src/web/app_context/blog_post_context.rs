use leptos::{create_signal, provide_context, ReadSignal, SignalGet, use_context};
use crate::entities::blog::Blog;
use crate::web::app_context::signal_context::{AppSignal, AppContext};

#[derive(Debug, Clone)]
struct BlogPostContextData {
    pub blog: Option<Blog>
}

impl Default for BlogPostContextData {
    fn default() -> Self {
        Self {
            blog: None
        }
    }
}

#[derive(Debug, Clone)]
struct BlogPostContext {
    signal: AppSignal<BlogPostContextData>
}

impl AppContext for BlogPostContext {
    fn new() -> Self {
        todo!()
    }
}

impl BlogPostContext {
}
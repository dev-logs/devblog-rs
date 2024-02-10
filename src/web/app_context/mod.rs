use leptos::{create_signal, provide_context, use_context};
use crate::web::app_context::signal_context::AppSignal;
use crate::web::home::navigation::HomeNavigationTab;

pub mod signal_context;
pub mod home_navigation_context;
mod blog_post_context;

pub fn provide_navigation_context() {
    if use_context::<AppSignal<HomeNavigationTab>>().is_none() {
        let (read, write) = create_signal(HomeNavigationTab::Blog);
        provide_context(AppSignal::<HomeNavigationTab>::new(read, write));
    }
}

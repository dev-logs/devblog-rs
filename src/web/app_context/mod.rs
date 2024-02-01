use std::io::Write;
use leptos::{create_signal, provide_context, SignalGet, use_context};
use crate::web::app_context::signal_context::AppSignalContext;
use crate::web::home::navigation::HomeNavigationTab;

pub mod signal_context;
pub mod home_navigation_context;

pub fn provide_navigation_context() {
    if use_context::<AppSignalContext<HomeNavigationTab>>().is_none() {
        let (read, write) = create_signal(HomeNavigationTab::Blog);
        provide_context(AppSignalContext::<HomeNavigationTab>::new(read, write));
    }
}

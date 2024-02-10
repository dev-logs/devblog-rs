use leptos::create_signal;
use crate::web::app_context::signal_context::{AppSignal, AppContext};
use crate::web::home::navigation::HomeNavigationTab;

#[derive(Debug, Clone)]
pub struct HomeNavigationSignalContext {
    pub signal: AppSignal<HomeNavigationTab>
}

impl AppContext for HomeNavigationSignalContext {
    fn new() -> Self {
        let (read, write) = create_signal(HomeNavigationTab::default());
        Self {
            signal: AppSignal::new(read, write)
        }
    }
}

impl HomeNavigationSignalContext {
    pub fn set_tab(&mut self, tab: HomeNavigationTab) {
        self.signal.write()(tab);
    }
}

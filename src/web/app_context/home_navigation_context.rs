use leptos::{create_signal, provide_context, ReadSignal, server, ServerFnError, use_context};
use crate::web::app_context::signal_context::{AppSignalContext, UseAppSignal};
use crate::web::home::navigation::HomeNavigationTab;

#[derive(Debug, Clone)]
pub struct HomeNavigationSignalContext {
    signal: AppSignalContext<HomeNavigationTab>
}

impl UseAppSignal<HomeNavigationTab> for HomeNavigationSignalContext {
    fn attach() -> Self {
        use_context::<HomeNavigationSignalContext>().unwrap_or_else(|| {
            let (read, write) = create_signal(HomeNavigationTab::default());
            let context = HomeNavigationSignalContext {
                signal: AppSignalContext::new(read, write)
            };

            provide_context(context);
            use_context::<HomeNavigationSignalContext>().unwrap()
        })
    }

    fn read(&self) -> ReadSignal<HomeNavigationTab> {
        self.signal.read()
    }
}

impl HomeNavigationSignalContext {
    pub fn set_tab(&mut self, tab: HomeNavigationTab) {
        self.signal.write()(tab);
    }
}

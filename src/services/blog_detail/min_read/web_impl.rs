use leptos::window;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::blog_detail::min_read::service::{CountReadMinutesService, Params};

pub struct CountReadMinutesServiceWebImpl {

}

impl Service<Params, usize> for CountReadMinutesServiceWebImpl {
    async fn execute(self, params: Params) -> Resolve<usize> {
        let texts= window().document().unwrap().body().unwrap().text_content().unwrap_or(String::new());
        let words: Vec<&str> = texts.split_whitespace().collect();

        Ok(words.len() / 160)
    }
}

impl CountReadMinutesService for CountReadMinutesServiceWebImpl {

}
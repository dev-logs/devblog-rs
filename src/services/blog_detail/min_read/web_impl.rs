use leptos::logging::log;
use reqwest::{Client};
use web_sys::{DomParser, SupportedType, window};
use crate::entities::errors::Errors;
use crate::services::base::service::{Resolve, Service};
use crate::services::blog_detail::min_read::service::{CountReadMinutesService, Params};

pub struct CountReadMinutesServiceWebImpl {

}

impl Service<Params, usize> for CountReadMinutesServiceWebImpl {
    async fn execute(self, params: Params) -> Resolve<usize> {
        let window = window().expect("no global `window` exists");
        let location = window.location();
        let protocol = location.protocol().unwrap();
        let host = location.host().expect("couldn't get host");
        let url = format!("{protocol}//{host}/{}", params.blog.url);

        let client = Client::new();
        let response = client.get(url).send().await.map_err(|e| Errors::InternalServerError(format!("Failed to load blog content {:?}", e)))?;
        if !response.status().is_success() {
            return Err(Errors::InternalServerError("".to_string()));
        }


        let htmlSource = response.text().await.unwrap();
        let parser = DomParser::new().unwrap();
        let document = parser.parse_from_string(&htmlSource, SupportedType::TextHtml).expect("Unable to parse html source to document");
        let texts = document.document_element().unwrap().text_content().expect("Can not get the text content from document");

        let words: Vec<&str> = texts.split_whitespace().collect();
        let words_count = words.len();
        let average_human_read_word_per_min = 160;

        Ok(words_count / average_human_read_word_per_min)
    }
}

impl CountReadMinutesService for CountReadMinutesServiceWebImpl {

}
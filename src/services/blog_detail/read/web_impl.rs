use std::time::Duration;
use leptos::logging::log;
use tokio::time::{sleep};
use leptos::window;
use surrealdb::key::root::all::new;
use crate::api::web_controllers::blog::mark_read::mark_read;
use crate::entities::errors::Errors;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::blog_detail::read::mark_read_service::{MarkReadService, Params};
use crate::web::utils::delay::delay;

pub struct MarkReadSerivceWebImpl {}

impl Service<Params, VoidResponse> for MarkReadSerivceWebImpl {
    async fn execute(self, params: Params) -> Resolve<VoidResponse> {
        let current_url = window().location().href()?;

        delay(Duration::from_secs(30)).await;
        let new_url = window().location().href()?;
        if new_url.eq(current_url.as_str()) {
            log!("User still on screen {} mark read", new_url);
            mark_read(params).await?;
            return Ok(());
        }

        Err(Errors::WebError("User switch no another page, cancel read".to_string()))
    }
}

impl MarkReadService for MarkReadSerivceWebImpl {}
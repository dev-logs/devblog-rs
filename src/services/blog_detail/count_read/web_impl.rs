use crate::api::web_controllers::blog::count_read::count_read;
use crate::services::base::service::{Resolve, Service};
use crate::services::blog_detail::count_read::service::{CountReadService, Params};

pub struct CountReadServiceWebImpl {}

impl Service<Params, usize> for CountReadServiceWebImpl {
    async fn execute(self, params: Params) -> Resolve<usize> {
        Ok(count_read(params).await?)
    }
}

impl CountReadService for CountReadServiceWebImpl {

}
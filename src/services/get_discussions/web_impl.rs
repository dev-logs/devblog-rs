use crate::api::web_controllers::discussions::get::get_discussions;
use crate::entities::discussion::Discussion;
use crate::services::base::service::{PageResponse, Resolve, Service};
use crate::services::get_discussions::service::{GetDiscussionsService, Params};

pub struct GetDiscussionsWebImpl {}

impl Service<Params, PageResponse<Discussion>> for GetDiscussionsWebImpl {
    async fn execute(self, params: Params) -> Resolve<PageResponse<Discussion>> {
        Ok(get_discussions(params).await?)
    }
}

impl GetDiscussionsService for GetDiscussionsWebImpl {

}

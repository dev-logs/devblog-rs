use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service::{CreateDiscussionService, Params, Response};

pub struct CreateDiscussionWebImpl {}

#[async_trait::async_trait]
impl Service<Params, Response> for CreateDiscussionWebImpl {
    async fn execute(self, params: Params) -> Resolve<Response> {
        todo!()
    }
}

impl CreateDiscussionService for CreateDiscussionWebImpl {}

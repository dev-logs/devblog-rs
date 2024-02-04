use crate::api::web_controllers::discussions::create::create_discussion;
use crate::entities::discussion::Discussion;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service::{CreateDiscussionService, Params};

pub struct CreateDiscussionWebImpl {}

impl Service<Params, Discussion> for CreateDiscussionWebImpl {
    async fn execute(self, params: Params) -> Resolve<Discussion> {
        let result = create_discussion(params).await?;
        Ok(result)
    }
}

impl CreateDiscussionService for CreateDiscussionWebImpl {}

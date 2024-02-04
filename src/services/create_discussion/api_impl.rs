use service::{Params, Response};
use crate::core_services::surrealdb::Db;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service;
use crate::services::create_discussion::service::CreateDiscussionService;

pub struct CreateDiscussionApiImpl {
    db: Db
}

impl CreateDiscussionApiImpl {
    pub fn new(db: Db) -> Self {
        Self {
            db
        }
    }
}

#[async_trait::async_trait]
impl Service<Params, Response> for CreateDiscussionApiImpl {
    async fn execute(self, params: Params) -> Resolve<Response> {
        Ok(Response {})
    }
}

impl CreateDiscussionService for CreateDiscussionApiImpl {}

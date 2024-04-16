use crate::api::web_controllers::discussions::create::create_discussion;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service::{CreateDiscussionService, Params};
use crate::web::local_storage::user::UserStorage;

pub struct CreateDiscussionWebImpl {}

impl Service<Params, Discussion> for CreateDiscussionWebImpl {
    async fn execute(self, params: Params) -> Resolve<Discussion> {
        let result = create_discussion(params).await;
        if (&result).is_err() {
            let error = result.as_ref().err().unwrap();
            if let Errors::UnAuthorization(_) = Errors::from(error.clone()) {
                UserStorage::new().delete();
            }
            else {
                return Err(error.into());
            }
        }

        Ok(result.unwrap())
    }
}

impl CreateDiscussionService for CreateDiscussionWebImpl {}

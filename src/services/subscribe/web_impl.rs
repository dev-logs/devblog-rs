use crate::api::web_controllers::user::subscribe::subscribe_user;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::subscribe::service::{Params, SubscribeService};

pub struct SubscribeServiceWebImpl {}

impl Service<Params, VoidResponse> for SubscribeServiceWebImpl {
    async fn execute(self, params: Params) -> Resolve<VoidResponse> {
        Ok(subscribe_user(params).await?)
    }
}

impl SubscribeService for SubscribeServiceWebImpl {}
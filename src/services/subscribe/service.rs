use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::{Service, VoidResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub(crate) email: String
}

pub trait SubscribeService : Service<Params, VoidResponse> {}
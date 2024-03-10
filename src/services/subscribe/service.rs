use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::{Service, VoidResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub email: String,
    pub display_name: Option<String>
}

pub trait SubscribeService : Service<Params, VoidResponse> {}
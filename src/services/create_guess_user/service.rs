use serde_derive::{Deserialize, Serialize};
use crate::entities::user::User;
use crate::services::base::service::Service;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub display_name: String
}

pub trait CreateGuestUserService : Service<Params, User> {}

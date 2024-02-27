use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::{Service, VoidResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {}

pub trait CountReadMinutesService : Service<Params, usize> {}
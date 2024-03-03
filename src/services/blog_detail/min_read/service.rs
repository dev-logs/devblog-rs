use serde_derive::{Deserialize, Serialize};
use crate::entities::blog::Blog;
use crate::services::base::service::Service;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
    pub(crate) blog: Blog
}

pub trait CountReadMinutesService : Service<Params, usize> {}
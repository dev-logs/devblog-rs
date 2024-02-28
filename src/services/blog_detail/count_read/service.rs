use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::Service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub blog_title: String
}

pub trait CountReadService: Service<Params, usize> {}
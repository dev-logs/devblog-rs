use serde_derive::{Deserialize, Serialize};
use crate::entities::discussion::Discussion;
use crate::services::base::service::{PageResponse, PagingParam, Service};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub blog_title: String,
    pub paging: PagingParam
}

pub trait GetDiscussionsService: Service<Params, PageResponse<Discussion>> {}

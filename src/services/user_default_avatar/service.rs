use crate::services::base::service::{NoParam, Service};

pub trait RandomUserDefaultAvatarService : Service<NoParam, String> {}

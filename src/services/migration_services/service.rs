use serde_derive::{Deserialize, Serialize};
use crate::services::base::service::{Service, VoidResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPostMigrationParams;

pub trait BlogPostMigrationService: Service<BlogPostMigrationParams, VoidResponse> + Sized {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorMigrationParams;

pub trait AuthorMigrationService: Service<AuthorMigrationParams, VoidResponse> + Sized {}

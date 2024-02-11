use leptos::{ServerFnError};
use serde_derive::{Deserialize, Serialize};
use surrealdb::Error;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Errors {
    #[error("You are not authorized to access/perform this resource/action")]
    UnAuthorization,
    #[error("Internal server error {}", .0)]
    InternalServerError(String)
}

impl From<serde_json::Error> for Errors {
    fn from(_: serde_json::Error) -> Self {
        Self::InternalServerError("Invalid format for json".to_string())
    }
}

impl From<surrealdb::Error> for Errors {
    fn from(value: Error) -> Self {
        return Self::InternalServerError(value.to_string())
    }
}

impl From<ServerFnError> for Errors {
    fn from(value: ServerFnError) -> Self {
        match value {
            ServerFnError::ServerError(body) => body.parse().unwrap_or_else(|e| Errors::from(e)),
            _ => Self::InternalServerError(String::from(""))
        }
    }
}

impl std::str::FromStr for Errors {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

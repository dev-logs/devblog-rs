use leptos::{ServerFnError};
use leptos::logging::log;
use serde_derive::{Deserialize, Serialize};
use surrealdb::Error;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Errors {
    #[error("You are not authorized to access/perform this resource/action")]
    UnAuthorization,
    #[error("Internal server error {}", .0)]
    InternalServerError(String),
    #[error("Already exist {}", .0)]
    AlreadyExist(String),
    #[error("Not found {}", .0)]
    NotFound(String),
    #[error("Error in client {}", .0)]
    WebError(String)
}

impl From<JsValue> for Errors {
    fn from(value: JsValue) -> Self {
        Self::WebError(value.as_string().unwrap_or("Unknown error".to_string()))
    }
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
            ServerFnError::ServerError(body) => {
                Errors::InternalServerError(body)
            },
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

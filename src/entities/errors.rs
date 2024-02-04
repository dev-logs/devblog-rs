#[cfg(feature = "ssr")]
use axum::http::StatusCode;

#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("You are not authorized to access/perform this resource/action")]
    UnAuthorization,
    #[error("Internal server error {}", .0)]
    #[cfg(feature = "ssr")]
    DbError(#[from] surrealdb::Error),
    #[error("Internal server error")]
    InternalServerError{ message: String },
}

#[cfg(feature = "ssr")]
impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        match self {
            Errors::UnAuthorization => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Errors::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            Errors::InternalServerError { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

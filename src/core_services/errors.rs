use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OurErrors {
    #[error("You are not authorized to access/perform this resource/action")]
    UnAuthorization,
    #[error("Internal server error {}", .0)]
    DbError(#[from] surrealdb::Error),
    #[error("Internal server error")]
    InternalServerError{ message: String },
}

impl IntoResponse for OurErrors {
    fn into_response(self) -> axum::response::Response {
        match self {
            OurErrors::UnAuthorization => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            OurErrors::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            OurErrors::InternalServerError { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

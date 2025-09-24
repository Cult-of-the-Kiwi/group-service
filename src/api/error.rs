use crate::{app, application::error::AppError};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    InternalServerError(#[from] AppError),
}

// TODO(Sa4dUs): Implement this properly
impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::BadRequest(str) => (StatusCode::BAD_REQUEST, axum::Json(str)),
            HandlerError::NotFound => (StatusCode::BAD_REQUEST, axum::Json("".to_string())),
            HandlerError::InternalServerError(app_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json("".to_string()),
            ),
        }
        .into_response()
    }
}

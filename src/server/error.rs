use crate::api::common::ApiResponse;
use axum::http::{Method, StatusCode, Uri};
use axum::BoxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("internal error")]
    InternalError,
}

pub async fn handle_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, ApiResponse<String>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        ApiResponse::new(-1, format!("{} {}failed", method, uri), Option::from(err.to_string()))
    )
}
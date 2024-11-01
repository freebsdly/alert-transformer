use std::io;
use axum::BoxError;
use axum::http::{Method, StatusCode, Uri};
use thiserror::Error;
use crate::common::ApiResponse;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown error")]
    Unknown,
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
        ApiResponse::new(-1, "failed".to_string(), Option::from(err.to_string()))
    )
}
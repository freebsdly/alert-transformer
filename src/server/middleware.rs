use crate::common::ApiResponse;
use axum::http::{Method, StatusCode, Uri};

pub async fn handler_404(method: Method, uri: Uri) -> (StatusCode, ApiResponse<String>) {
    (StatusCode::NOT_FOUND, ApiResponse::err(-1, format!("{} {} Not Found", method, uri)))
}
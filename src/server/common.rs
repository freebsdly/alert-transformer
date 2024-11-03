use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        Self { code, message, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, String::from("successful"), Some(data))
    }

    pub fn err(code: i32, message: String) -> Self {
        Self::new(code, message, None)
    }
}

impl<T> IntoResponse for ApiResponse<T>
where Json<ApiResponse<T>>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}


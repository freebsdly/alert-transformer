use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
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


// // Make our own error that wraps `anyhow::Error`.
// pub struct AppError(anyhow::Error);
//
// // Tell axum how to convert `AppError` into a response.
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         Json(ApiResponse::<T>::err(-1, self.0.to_string())).into_response()
//     }
// }
//
// // This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// // `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for AppError
// where
//     E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }


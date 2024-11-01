use crate::common::ApiResponse;

pub async fn root() -> ApiResponse<String> {
    ApiResponse::ok("good".to_string())
}
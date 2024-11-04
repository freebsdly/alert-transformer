use crate::api::common::{ApiResponse};
use crate::{AppState, PostVo};
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use std::sync::Arc;

pub async fn root() -> ApiResponse<String> {
    ApiResponse::ok("good".to_string())
}

#[debug_handler]
pub async fn get_post(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> ApiResponse<PostVo> {
    let data = state.api.get_post(id).await;
    match data {
        Ok(post) => {
            ApiResponse::ok(post)
        }
        Err(err) => {
            ApiResponse::err(-1, err.to_string())
        }
    }
}

#[debug_handler]
pub async fn get_posts(State(state): State<Arc<AppState>>) -> ApiResponse<Vec<PostVo>> {
    let data = state.api.get_posts().await;
    match data {
        Ok(post) => {
            ApiResponse::ok(post)
        }
        Err(err) => {
            ApiResponse::err(-1, err.to_string())
        }
    }
}
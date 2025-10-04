use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
};

use crate::{
    AppState,
    common::api::{APIResponse, AppResponse},
    models::task,
    services::task::TaskServices,
};
use uuid;

pub fn tasks_route() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(get_user_tasks).post(create_task))
        .route("/tasks/{id}", patch(update_task).delete(delete_task))
}

pub async fn get_user_tasks(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> AppResponse<Vec<task::Task>> {
    tracing::info!("Creating task for user: {:?}", user_id);

    match TaskServices::get_tasks(&app_state, user_id).await {
        Ok(response) => return Ok(APIResponse::success(response)),
        Err(err) => Err(err),
    }
}
pub async fn create_task() {}
pub async fn update_task() {}
pub async fn delete_task() {}

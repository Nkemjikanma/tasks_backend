use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
};

use uuid::Uuid;

use crate::{
    AppState,
    common::api::{APIResponse, AppResponse},
    middleware::AuthenticatedUser,
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
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResponse<Vec<task::Task>> {
    tracing::info!("getting all tasks for user: {:?}", user.username);

    match TaskServices::get_tasks(&app_state, user.user_id).await {
        Ok(response) => Ok(APIResponse::success(response)),
        Err(err) => Err(err),
    }
}
pub async fn create_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(task): Json<task::CreateTaskPayload>,
) -> AppResponse<task::TasksResponse> {
    tracing::info!("creating task for user: {:?}", user.username);

    match TaskServices::create_task(&app_state, user.user_id, Json(task)).await {
        Ok(response) => Ok(APIResponse::success(response)),
        Err(err) => Err(err),
    }
}
pub async fn update_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(task_id): Path<Uuid>,
    Json(update_fields): Json<task::UpdateTaskPayload>,
) -> AppResponse<task::TasksResponse> {
    tracing::info!("Edditing task {}", user.username);

    match TaskServices::update(&app_state, user.user_id, update_fields, task_id).await {
        Ok(response) => Ok(APIResponse::success(response)),
        Err(err) => Err(err),
    }
}
pub async fn delete_task() {}

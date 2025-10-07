pub struct TaskServices;
use crate::{AppState, common::errors::AppError, models::task};
use axum::Json;

use chrono::{DateTime, Utc};
use uuid::Uuid;
impl TaskServices {
    pub async fn get_tasks(
        app_state: &AppState,
        user_id: i64,
    ) -> Result<Vec<task::Task>, AppError> {
        tracing::info!("Loading all tasks by user");

        let user_tasks = sqlx::query_as::<_, task::Task>(
            r#"SELECT * FROM tasks WHERE user_id = $1 ORDER BY updated_at"#,
        )
        .bind(user_id)
        .fetch_all(&app_state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching user's tasks: {:?}", e);
            AppError::ErrorFetchingTasks
        })?;

        Ok(user_tasks)
    }

    pub async fn create_task(
        app_state: &AppState,
        user_id: i64,
        Json(task): Json<task::CreateTaskPayload>,
    ) -> Result<task::TasksResponse, AppError> {
        tracing::info!("Adding {} to db", task.title);

        let status = task.status.unwrap_or(task::TaskStatus::Pending);

        let new_task = sqlx::query_as!(task::TasksResponse,
            r#"
                INSERT INTO tasks (id, title, description, status, due_date, user_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING  title, description, status, due_date
        "#,
        Uuid::new_v4(),
        task.title,
        task.description,
        status.to_string(),
        task.due_date,
        user_id as i64,
        Utc::now(),
        Utc::now()
        )
            .fetch_one(&app_state.pool)
            .await.map_err(|e| {
                tracing::info!("Error creating task");
                AppError::TaskCreationFailed
            })?;

        Ok(new_task)
    }
}

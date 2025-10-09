use crate::{AppState, common::errors::AppError, models::task};
use axum::Json;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TaskServices;

impl TaskServices {
    pub async fn update(
        app_state: &AppState,
        user_id: i64,
        update_fields: task::UpdateTaskPayload,
        task_id: Uuid,
    ) -> Result<task::TasksResponse, AppError> {
        tracing::info!("Editing fileds now");

        let old_task = sqlx::query_as!(task::Task, r#"SELECT * FROM tasks WHERE id = $1"#, task_id)
            .fetch_one(&app_state.pool)
            .await
            .map_err(|e| {
                tracing::error!("Task not found");
                AppError::TaskNotFound
            })?;

        let updated_task = task::Task {
            id: old_task.id,
            title: update_fields.title.unwrap_or(old_task.title),
            description: update_fields.description.or(old_task.description),
            status: update_fields.status.unwrap_or(old_task.status),
            due_date: update_fields.due_date.unwrap_or(old_task.due_date),
            user_id,
            created_at: old_task.created_at,
            updated_at: Utc::now(),
        };

        let saved_task = sqlx::query_as!(task::TasksResponse, r#"UPDATE tasks SET title = $1, description = $2, status = $3, due_date = $4, updated_at = $5 WHERE id = $6 RETURNING title, description, status, due_date"#,updated_task.title,
        updated_task.description,
        updated_task.status,
        updated_task.due_date,
        updated_task.updated_at,
        task_id )
            .fetch_one(&app_state.pool)
            .await
            .map_err(|e| {
  tracing::error!(error = ?e, "Failed to update task");
        AppError::ErrorFetchingTasks
            })?;

        Ok(saved_task)
    }

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

    pub async fn delete_task(app_state: &AppState, task_id: Uuid) -> Result<String, AppError> {
        tracing::info!("Deleteing task...");

        let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
            .execute(&app_state.pool)
            .await
            .map_err(|e| {
                tracing::error!(error =?e, "Error deleting task");
                AppError::ErrorDeletingTask
            })?;

        if result.rows_affected() == 0 {
            tracing::warn!(%task_id, "NO task found to delete");
            return Err(AppError::NotFound);
        }

        Ok("Sucess deleting task".to_string())
    }
}

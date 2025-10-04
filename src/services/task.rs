pub struct TaskServices;
use crate::{AppState, common::errors::AppError, models::task};

impl TaskServices {
    pub async fn get_tasks(
        app_state: &AppState,
        user_id: i64,
    ) -> Result<Vec<task::Task>, AppError> {
        tracing::info!("Loading all tasks by user");

        // TODO: how do I get the user_id of the requester
        //
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
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use ::sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskPayload {
    pub title: String,
    pub description: Option<String>,
    pub due_date: DateTime<Utc>,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub due_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")] // Tells sqlx it is stored as varchar
#[serde(rename_all = "lowercase")] // serializes to "pending" not PENDING
pub enum TaskStatus {
    Pending,
    #[serde(rename = "in_progress")]
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_date: DateTime<Utc>,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// #[derive(debug, serialize, deserialize, fromrow)]
// pub struct TasksResponse {}

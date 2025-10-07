use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use ::sqlx::{FromRow, Type};
use uuid::Uuid;

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

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_date: DateTime<Utc>,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_date: DateTime<Utc>,
}

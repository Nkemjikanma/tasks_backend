use thiserror::Error;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Sign up failed")]
    SignupFailed,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Password hasing failed")]
    PasswordHashingFailed,

    #[error("Database query failed")]
    DatabaseQueryFailed,

    #[error("Invalid login credentials")]
    InvalidUserCredentials,

    #[error("JWT creation failed")]
    JWTCreationFailed,

    #[error("Error fetching tasks")]
    ErrorFetchingTasks,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid Token")]
    InvalidToken,

    #[error("Task creation failed")]
    TaskCreationFailed,

    #[error("Task not found")]
    TaskNotFound,

    #[error("Error updating task")]
    ErrorUpdatingTask,

    #[error("Error deleting task")]
    ErrorDeletingTask,

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            // --- User related ---
            Self::SignupFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Sign-up process failed unexpectedly",
            ),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "The user already exists"),
            Self::PasswordHashingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Passoword hashing failed",
            ),
            Self::InvalidUserCredentials => (StatusCode::UNAUTHORIZED, "Invalid email or password"),
            Self::JWTCreationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create authentication token(JWT)",
            ),
            Self::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized to access"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),

            // --- Task-related ---
            Self::ErrorFetchingTasks => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getching all user tasks",
            ),
            Self::TaskCreationFailed => (StatusCode::BAD_REQUEST, "Failed to create task"),
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            Self::ErrorUpdatingTask => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating task"),
            Self::ErrorDeletingTask => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting task"),

            // --- General ---
            Self::DatabaseQueryFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found"),
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

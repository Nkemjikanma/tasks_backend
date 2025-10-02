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
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::SignupFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"),
            Self::UserAlreadyExists => (StatusCode::FORBIDDEN, "The user already exists"),
            Self::PasswordHashingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Passoword hashing failed",
            ),
            Self::DatabaseQueryFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed")
            }
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

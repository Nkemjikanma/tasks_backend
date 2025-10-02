use crate::{
    common::{errors::AppError, utils::PasswordUtils},
    models::user,
};
use chrono::Utc;
use sqlx::PgPool;
pub struct UserService;

impl UserService {
    pub async fn create_user(
        pool: &PgPool,
        request: user::SignupAndLoginPayload,
    ) -> Result<i64, AppError> {
        tracing::info!("Creating user: {}", request.username);

        // check for existing username
        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(&request.username)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Database error checking username existence '{}': {:?}",
                        request.username,
                        e
                    );
                    AppError::DatabaseQueryFailed
                })?;

        if user_exists {
            return Err(AppError::UserAlreadyExists);
        }

        let mut tx = pool.begin().await.map_err(|_| {
            tracing::error!("Database error starting to pool for user creation");
            AppError::DatabaseQueryFailed
        })?;

        // TODO: hash password
        let hashed_password = PasswordUtils::hash_password(&request.password)?;

        // insert user and hashed password into DB
        // Create user
        let new_user_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO users (username, password_hash, created_at)
             VALUES ($1, $2, $3)
             RETURNING id",
        )
        .bind(&request.username)
        .bind(&hashed_password)
        .bind(Utc::now().naive_utc())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(
                "Database error creating user '{}': {:?}",
                request.username,
                e
            );
            AppError::DatabaseQueryFailed
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!(
                "Database error committing user creation transaction: {:?}",
                e
            );
            AppError::DatabaseQueryFailed
        })?;

        Ok(new_user_id)
    }
}

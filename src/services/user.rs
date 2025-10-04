use crate::{
    AppState,
    common::{errors::AppError, jwt, utils::PasswordUtils},
    models::user,
};
use chrono::Utc;
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn create_user(
        app_state: &AppState,
        request: user::SignupAndLoginPayload,
    ) -> Result<i64, AppError> {
        tracing::info!("Creating user: {}", request.username);

        // check for existing username
        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(&request.username)
                .fetch_one(&app_state.pool)
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

        let mut tx = app_state.pool.begin().await.map_err(|_| {
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

    pub async fn login(
        app_state: &AppState,
        request: user::SignupAndLoginPayload,
    ) -> Result<user::LoginResponse, AppError> {
        tracing::info!("Attempting login");

        let user = sqlx::query_as::<_, user::DBUserQuery>(
            "SELECT id, username, password_hash FROM users WHERE username=$1",
        )
        .bind(&request.username)
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error signing user in'{}': {:?}", request.username, e);
            AppError::DatabaseQueryFailed
        })?;

        let user = match user {
            Some(u) => u,
            None => return Err(AppError::InvalidUserCredentials),
        };

        if !PasswordUtils::verfify_passowrd(&request.password.to_string(), &user.password_hash) {
            tracing::warn!("Invalid login attempt");

            return Err(AppError::InvalidUserCredentials);
        }

        tracing::info!("Login successful");

        // generate JWT
        let token =
            jwt::generate_token(&user.username, user.id, &app_state.jwt_config).map_err(|_| {
                tracing::error!("Error generating jwt for user {}", user.username);

                AppError::JWTCreationFailed
            })?;

        tracing::info!("JWT successfully created for: {}", &user.username);
        tracing::info!("{} was successfully logged in", &user.username);

        Ok(user::LoginResponse {
            token,
            username: request.username,
            user_id: user.id,
        })
    }
}

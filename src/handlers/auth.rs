use axum::{
    Json, Router,
    extract::{ConnectInfo, State},
    http::HeaderMap,
    routing::{get, post},
};

use crate::common::api::AppResponse;
use crate::{
    AppState, common::api::APIResponse, models::user, models::user::LoginResponse,
    services::user::UserService,
};
use std::{net::SocketAddr, time::Instant};

use tracing::instrument;

use sqlx::PgPool;
pub fn public_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/sign_up", post(sign_up_handler))
        .route("/login", post(login_handler))
}

pub fn protected_auth_routes() -> Router<AppState> {
    Router::new().route("/logout", post(logout_handler))
}

#[instrument(skip(app_state, payload))]
pub async fn sign_up_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<user::SignupAndLoginPayload>,
) -> AppResponse<i64> {
    tracing::info!("Creating user: {}", payload.username);

    let user_id = UserService::create_user(&app_state, payload).await?;

    tracing::info!("Successfully updated user");

    Ok(APIResponse::success(user_id))
}

async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<user::SignupAndLoginPayload>,
) -> AppResponse<LoginResponse> {
    tracing::info!("Starting login process for {:?}", payload.username);

    match UserService::login(&app_state, payload).await {
        Ok(response) => Ok(APIResponse::success(response)),
        Err(err) => Err(err),
    }
}

async fn logout_handler() -> &'static str {
    "Logout handler not yet created"
}

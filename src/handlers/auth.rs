use axum::{
    Json, Router,
    extract::{ConnectInfo, State},
    http::HeaderMap,
    routing::{get, post},
};

use crate::common::api::AppResponse;
use crate::{common::api::APIResponse, models::user, services::user::UserService};
use std::{net::SocketAddr, time::Instant};

use tracing::instrument;

use sqlx::PgPool;
pub fn public_auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/sign_up", post(sign_up_handler))
        .route("/login", post(login_handler))
}

pub fn protected_auth_routes() -> Router<PgPool> {
    Router::new().route("/logout", post(logout_handler))
}

#[axum::debug_handler]
#[instrument(skip(pool, payload))]
pub async fn sign_up_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<user::SignupAndLoginPayload>,
) -> AppResponse<i64> {
    tracing::info!("Creating user: {}", payload.username);

    let user_id = UserService::create_user(&pool, payload).await?;

    tracing::info!("Successfully updated user");

    Ok(APIResponse::success(user_id))
}

#[axum::debug_handler]
async fn login_handler(
    State(_pool): State<PgPool>,
    Json(_payload): Json<user::SignupAndLoginPayload>,
) -> &'static str {
    "Longin handler not yet implemented"
}

async fn logout_handler() -> &'static str {
    "Logout handler not yet created"
}

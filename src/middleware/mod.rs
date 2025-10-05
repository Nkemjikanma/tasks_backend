use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::common::{errors::AppError, jwt};

use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

impl AuthenticatedUser {
    pub fn new(username: String, user_id: i64) -> Self {
        Self { user_id, username }
    }
}

// impl<S> FromRequestParts<S> for AuthenticatedUser
// where
//     S: Send + Sync,
// {
//     type Rejection = AppError;
//
//     fn from_request_parts(
//         parts: &mut Parts,
//         state: &S,
//     ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
//         async move {
//             parts.extensions.get::<AuthenticatedUser>().cloned().ok_or_else(|| {
//                 tracing::error!("Authenticated user not found - missing middleware or user not authenticated");
//                 AppError::InvalidToken
//             })
//         }
//     }
// }

pub async fn middleware_auth(
    State(app_state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let (mut parts, body) = req.into_parts();
    let token = parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| {
            tracing::error!("Missing/invalid auth header for {}", parts.uri.path());
            AppError::InvalidToken
        })?;

    // Verify jwt and extract claims
    let claims = jwt::verify_token(token, &app_state.jwt_config).map_err(|e| {
        tracing::error!("JWT verification failed for {}: {:?}", parts.uri.path(), e);

        AppError::InvalidToken
    })?;

    tracing::debug!(
        "JWT verified for {} ({}) accessing {}",
        claims.user_id,
        claims.username,
        parts.uri.path()
    );

    // inject current user into db pool and request extensions
    let authenticated_user = AuthenticatedUser::new(claims.username.clone(), claims.user_id);

    parts.extensions.insert(authenticated_user);

    let req = Request::from_parts(parts, body);

    tracing::info!(
        "Auth completed for user {} ({})",
        claims.user_id,
        claims.username
    );

    Ok(next.run(req).await)
}

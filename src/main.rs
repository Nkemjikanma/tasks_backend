mod common;
mod config;
mod database;
mod handlers;
mod models;
mod services;

use crate::{
    config::{Config, JWTConfig},
    database::connection::create_pool,
    handlers::auth::{protected_auth_routes, public_auth_routes},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    },
    routing::{get, post},
};

use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_config: JWTConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = Config::default().map_err(|e| format!("Config error: {:?}", e))?;
    start_server(config).await?;

    Ok(())
}

pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing db connection");
    let pool = create_pool(config.database).await?;

    let app_state = AppState {
        pool,
        jwt_config: config.jwt_config,
    };

    let cors_layer = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT, ORIGIN]);

    tracing::info!("Setting up routes");

    let protected_api = Router::new().merge(protected_auth_routes());
    let public_api = Router::new().merge(public_auth_routes());

    let app = Router::new()
        .route("/", get(root))
        .nest("/api", public_api.merge(protected_api))
        .layer(cors_layer)
        .with_state(app_state) // new way of sharing state
        .into_make_service_with_connect_info::<SocketAddr>();

    let server_address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    tracing::info!("Server has started");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

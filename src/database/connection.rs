use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::config::DBConfig;

#[tracing::instrument(name = "pool", skip_all)]
pub async fn create_pool(config: DBConfig) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Creating database pool");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connection_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&config.url)
        .await?;

    tracing::info!("Database connection pool created");
    Ok(pool)
}

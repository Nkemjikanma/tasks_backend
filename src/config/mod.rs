use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct JWTConfig {
    // The sercret for signing and verifying tokens
    pub secret: String,

    // the duration in secs for which it is valid
    pub expiration: i64,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DBConfig,
    pub server: ServerConfig,
    pub jwt_config: JWTConfig,
}

impl Config {
    pub fn default() -> Result<Self, ConfigError> {
        Ok(Self {
            database: DBConfig {
                url: std::env::var("DATABASE_URL").expect("Database URL must be set in .env"),
                max_connections: 5,
                min_connections: 1,
                connection_timeout: Duration::from_secs(200),
                idle_timeout: Duration::from_secs(300),
            },
            server: ServerConfig {
                host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("APP_PORT").unwrap_or_else(|_| "8000".to_string()),
            },
            jwt_config: JWTConfig {
                secret: std::env::var("SECRET").unwrap_or_else(|_| {
                    tracing::warn!("Using very unsafe secret to generate jwt");

                    "very_long_but_unsafe_secret_in_the_air".to_string()
                }),
                expiration: std::env::var("EXPIRATION")
                    .map(|exp| {
                        exp.parse::<i64>().unwrap_or_else(|_| {
                            tracing::warn!("JWT not parsed so using default");

                            3600 // 1hr
                        })
                    })
                    .unwrap_or(3600),
            },
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(&'static str),
}

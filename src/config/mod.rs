use std::time::Duration;
#[derive(Debug, Clone)]
pub struct Config {
    pub database: DBConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
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
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(&'static str),
}

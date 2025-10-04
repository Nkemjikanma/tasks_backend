use crate::{config::JWTConfig, models};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use chrono::{Duration, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_token(
    username: &str,
    user_id: i64,
    config: &JWTConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = (now + Duration::seconds(config.expiration)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        user_id,
        username: username.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

pub fn verify_token(
    token: &str,
    config: &JWTConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config_secret = config.secret.as_bytes();
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config_secret),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(decoded.claims)
}

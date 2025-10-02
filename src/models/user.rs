// id SERIAL PRIMARY KEY,
// username TEXT UNIQUE NOT NULL,
// password_hash TEXT NOT NULL, -- store a bcrypt/argon2 hash, never plaintext!
// created_at TIMESTAMP NOT NULL DEFAULT NOW()
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupAndLoginPayload {
    pub username: String,
    pub password: String,
}

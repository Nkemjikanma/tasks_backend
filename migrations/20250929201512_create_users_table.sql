-- Add migration script here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL, -- store a bcrypt/argon2 hash, never plaintext!
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

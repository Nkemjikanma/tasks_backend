-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), --unique ID
    title TEXT NOT NULL,            -- task title
    description TEXT,               -- optional
    status VARCHAR(20) NOT NULL,    -- e.g. "pending", "in_progress", "done"
    due_date TIMESTAMPTZ NOT NULL,  -- stores date/time in UTC
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add migration script here
CREATE TABLE IF NOT EXISTS shorteners (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    original_url TEXT NOT NULL,
    short_url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Add authentication method and social ID columns to users table
ALTER TABLE users ADD COLUMN auth_method TEXT NOT NULL DEFAULT 'Email';
ALTER TABLE users ADD COLUMN social_id TEXT;

-- Create index for faster social login lookups
CREATE INDEX idx_users_auth_method_social_id ON users(auth_method, social_id);

-- Create one_time_tokens table for passwordless authentication
CREATE TABLE one_time_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    used BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index for token lookups
CREATE INDEX idx_one_time_tokens_token_hash ON one_time_tokens(token_hash);
CREATE INDEX idx_one_time_tokens_user_id ON one_time_tokens(user_id);
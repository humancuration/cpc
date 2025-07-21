-- Create nodes table
CREATE TABLE nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    public_key BYTEA NOT NULL,
    last_seen TIMESTAMPTZ NOT NULL,
    resources JSONB NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('online', 'offline', 'degraded')),
    endpoint VARCHAR(255) NOT NULL
);

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    public_key BYTEA NOT NULL,
    friends UUID[] DEFAULT '{}',
    permissions JSONB NOT NULL DEFAULT '{}'
);

-- Create content table
CREATE TABLE content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    tags VARCHAR(50)[] DEFAULT '{}',
    availability INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create index for faster searches
CREATE INDEX idx_content_tags ON content USING GIN(tags);
CREATE INDEX idx_content_title ON content(title);

-- Create session tracking table
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    jwt_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
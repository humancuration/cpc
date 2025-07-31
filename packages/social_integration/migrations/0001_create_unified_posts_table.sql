CREATE TABLE unified_posts (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL,
    source VARCHAR(20) NOT NULL,
    content JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    upvotes INTEGER NOT NULL DEFAULT 0,
    comments INTEGER NOT NULL DEFAULT 0,
    shares INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_unified_posts_author_id ON unified_posts (author_id);
CREATE INDEX idx_unified_posts_source ON unified_posts (source);
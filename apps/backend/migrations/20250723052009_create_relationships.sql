-- migrations/20250723052009_create_relationships.sql

CREATE TABLE relationships (
    id UUID PRIMARY KEY NOT NULL,
    follower_id UUID NOT NULL,
    followed_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now')),
    FOREIGN KEY(follower_id) REFERENCES users(id),
    FOREIGN KEY(followed_id) REFERENCES users(id),
    UNIQUE(follower_id, followed_id)
);
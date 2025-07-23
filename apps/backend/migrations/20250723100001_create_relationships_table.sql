-- Create relationships table
CREATE TABLE relationships (
    id UUID PRIMARY KEY NOT NULL,
    follower_id UUID NOT NULL,
    followed_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now')),
    UNIQUE(follower_id, followed_id)
);
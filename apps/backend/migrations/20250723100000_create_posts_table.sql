-- Create posts table
CREATE TABLE posts (
    id UUID PRIMARY KEY NOT NULL,
    author_id UUID NOT NULL,
    content TEXT NOT NULL,
    visibility TEXT NOT NULL CHECK(visibility IN ('PUBLIC', 'COOPERATIVE', 'PRIVATE')),
    cooperative_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now')),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now'))
);
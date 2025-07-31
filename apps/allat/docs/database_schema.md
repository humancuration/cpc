# Allat Database Schema

## Overview
This document describes the PostgreSQL database schema for the Allat forum application.

**Note**: This is a local schema definition for Allat. For the complete cross-app database schema, see [social_integration/docs/database_schema.md](../../social_integration/docs/database_schema.md).

## Tables

### communities
```sql
CREATE TABLE communities (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    rules TEXT[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

### users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    karma INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

### posts
```sql
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    community_id UUID NOT NULL REFERENCES communities(id),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    parent_id UUID REFERENCES posts(id)  -- For threaded comments
);
```

### media_assets
```sql
CREATE TABLE media_assets (
    id UUID PRIMARY KEY,
    post_id UUID NOT NULL REFERENCES posts(id),
    url VARCHAR(255) NOT NULL,
    thumbnail_url VARCHAR(255),
    media_type VARCHAR(50) NOT NULL,  -- 'Image' or 'Video'
    alt_text TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

### votes
```sql
CREATE TABLE votes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    post_id UUID NOT NULL REFERENCES posts(id),
    vote_type VARCHAR(50) NOT NULL,  -- 'Upvote' or 'Downvote'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, post_id)
);
```

### comments
```sql
-- Comments are stored in the posts table with parent_id referencing the post they belong to
-- This schema uses the same table for posts and comments to support nested threading
```

## Indexes

```sql
-- Indexes for better query performance
CREATE INDEX idx_posts_community_id ON posts(community_id);
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_parent_id ON posts(parent_id);
CREATE INDEX idx_posts_created_at ON posts(created_at);
CREATE INDEX idx_media_assets_post_id ON media_assets(post_id);
CREATE INDEX idx_votes_post_id ON votes(post_id);
CREATE INDEX idx_votes_user_id ON votes(user_id);

## Testing
To run repository tests:
```sh
cd apps/allat
cargo test
```

Tests use Testcontainers to spin up a temporary PostgreSQL instance.
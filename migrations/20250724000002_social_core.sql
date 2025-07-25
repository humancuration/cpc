-- Create enums for social models
CREATE TYPE visibility AS ENUM ('PUBLIC', 'COOPERATIVE', 'PRIVATE');
CREATE TYPE media_type AS ENUM ('IMAGE', 'VIDEO', 'AUDIO', 'UNKNOWN');
CREATE TYPE processing_status AS ENUM ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED');
CREATE TYPE like_target_type AS ENUM ('POST', 'COMMENT');
CREATE TYPE share_type AS ENUM ('DIRECT', 'MESSAGE', 'EXTERNAL');

-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    visibility visibility NOT NULL DEFAULT 'PUBLIC',
    cooperative_id UUID,
    feed_position INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags TEXT[] DEFAULT '{}',
    mentions UUID[] DEFAULT '{}',
    reply_to_post_id UUID REFERENCES posts(id) ON DELETE SET NULL,
    repost_of_post_id UUID REFERENCES posts(id) ON DELETE SET NULL
);

-- Create media_items table
CREATE TABLE IF NOT EXISTS media_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    media_type media_type NOT NULL,
    processing_status processing_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    file_size BIGINT,
    duration INTEGER,
    width INTEGER,
    height INTEGER,
    thumbnail_url TEXT,
    alt_text TEXT
);

-- Create comments table
CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    parent_comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    thread_depth INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    mentions UUID[] DEFAULT '{}'
);

-- Create replies table (for post-to-post replies)
CREATE TABLE IF NOT EXISTS replies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    reply_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(original_post_id, reply_post_id)
);

-- Create likes table
CREATE TABLE IF NOT EXISTS likes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type like_target_type NOT NULL,
    target_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, target_type, target_id)
);

-- Create shares table
CREATE TABLE IF NOT EXISTS shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    share_message TEXT,
    share_type share_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create reposts table
CREATE TABLE IF NOT EXISTS reposts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    original_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    repost_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, original_post_id)
);

-- Create post_edits table for edit history
CREATE TABLE IF NOT EXISTS post_edits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    previous_content TEXT NOT NULL,
    edit_reason TEXT,
    edited_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create comment_edits table for edit history
CREATE TABLE IF NOT EXISTS comment_edits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id UUID NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
    previous_content TEXT NOT NULL,
    edit_reason TEXT,
    edited_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create relationships table (simplified follow relationship)
CREATE TABLE IF NOT EXISTS relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, followed_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_visibility ON posts(visibility);
CREATE INDEX IF NOT EXISTS idx_posts_reply_to ON posts(reply_to_post_id);
CREATE INDEX IF NOT EXISTS idx_posts_repost_of ON posts(repost_of_post_id);

CREATE INDEX IF NOT EXISTS idx_media_items_post_id ON media_items(post_id);
CREATE INDEX IF NOT EXISTS idx_media_items_processing_status ON media_items(processing_status);

CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);
CREATE INDEX IF NOT EXISTS idx_comments_author_id ON comments(author_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_comment_id);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_replies_original_post_id ON replies(original_post_id);
CREATE INDEX IF NOT EXISTS idx_replies_reply_post_id ON replies(reply_post_id);
CREATE INDEX IF NOT EXISTS idx_replies_author_id ON replies(author_id);

CREATE INDEX IF NOT EXISTS idx_likes_user_id ON likes(user_id);
CREATE INDEX IF NOT EXISTS idx_likes_target ON likes(target_type, target_id);

CREATE INDEX IF NOT EXISTS idx_shares_user_id ON shares(user_id);
CREATE INDEX IF NOT EXISTS idx_shares_post_id ON shares(post_id);

CREATE INDEX IF NOT EXISTS idx_reposts_user_id ON reposts(user_id);
CREATE INDEX IF NOT EXISTS idx_reposts_original_post_id ON reposts(original_post_id);

CREATE INDEX IF NOT EXISTS idx_relationships_follower_id ON relationships(follower_id);
CREATE INDEX IF NOT EXISTS idx_relationships_followed_id ON relationships(followed_id);
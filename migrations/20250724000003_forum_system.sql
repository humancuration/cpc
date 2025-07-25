-- Create enums for forum system
CREATE TYPE post_type AS ENUM ('SOCIAL', 'THREAD', 'REPLY');
CREATE TYPE forum_category AS ENUM ('GENERAL', 'DISCUSSION', 'QANDA', 'ANNOUNCEMENTS', 'SUPPORT', 'FEEDBACK', 'OFFTOPIC', 'CUSTOM');
CREATE TYPE community_role AS ENUM ('OWNER', 'MODERATOR', 'MEMBER', 'CONTRIBUTOR', 'VIEWER');

-- Create communities table
CREATE TABLE IF NOT EXISTS communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    icon_url TEXT,
    banner_url TEXT,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    moderator_ids UUID[] DEFAULT '{}',
    member_count BIGINT NOT NULL DEFAULT 0,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    is_nsfw BOOLEAN NOT NULL DEFAULT FALSE,
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create community_rules table
CREATE TABLE IF NOT EXISTS community_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create forums table
CREATE TABLE IF NOT EXISTS forums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    category forum_category NOT NULL DEFAULT 'GENERAL',
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    thread_count BIGINT NOT NULL DEFAULT 0,
    post_count BIGINT NOT NULL DEFAULT 0,
    last_activity_at TIMESTAMPTZ,
    moderator_ids UUID[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create moderation_settings table
CREATE TABLE IF NOT EXISTS moderation_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    require_approval BOOLEAN NOT NULL DEFAULT FALSE,
    auto_lock_after_days INTEGER,
    max_thread_length INTEGER,
    allow_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    rate_limit_posts INTEGER,
    banned_words TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create threads table
CREATE TABLE IF NOT EXISTS threads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    reply_count BIGINT NOT NULL DEFAULT 0,
    view_count BIGINT NOT NULL DEFAULT 0,
    upvote_count BIGINT NOT NULL DEFAULT 0,
    downvote_count BIGINT NOT NULL DEFAULT 0,
    last_reply_at TIMESTAMPTZ,
    last_reply_by UUID REFERENCES users(id) ON DELETE SET NULL,
    tags TEXT[] DEFAULT '{}',
    flair VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create thread_replies table
CREATE TABLE IF NOT EXISTS thread_replies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    thread_id UUID NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    parent_reply_id UUID REFERENCES thread_replies(id) ON DELETE CASCADE,
    thread_depth INTEGER NOT NULL DEFAULT 0,
    upvote_count BIGINT NOT NULL DEFAULT 0,
    downvote_count BIGINT NOT NULL DEFAULT 0,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    is_moderator_reply BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create thread_reply_edits table
CREATE TABLE IF NOT EXISTS thread_reply_edits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reply_id UUID NOT NULL REFERENCES thread_replies(id) ON DELETE CASCADE,
    previous_content TEXT NOT NULL,
    edit_reason TEXT,
    edited_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create community_memberships table
CREATE TABLE IF NOT EXISTS community_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role community_role NOT NULL DEFAULT 'MEMBER',
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    ban_reason TEXT,
    ban_expires_at TIMESTAMPTZ,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(community_id, user_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_communities_owner_id ON communities(owner_id);
CREATE INDEX IF NOT EXISTS idx_communities_name ON communities(name);
CREATE INDEX IF NOT EXISTS idx_communities_created_at ON communities(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_community_rules_community_id ON community_rules(community_id);
CREATE INDEX IF NOT EXISTS idx_community_rules_order ON community_rules(community_id, order_index);

CREATE INDEX IF NOT EXISTS idx_forums_community_id ON forums(community_id);
CREATE INDEX IF NOT EXISTS idx_forums_category ON forums(category);
CREATE INDEX IF NOT EXISTS idx_forums_last_activity ON forums(last_activity_at DESC);

CREATE INDEX IF NOT EXISTS idx_moderation_settings_forum_id ON moderation_settings(forum_id);

CREATE INDEX IF NOT EXISTS idx_threads_forum_id ON threads(forum_id);
CREATE INDEX IF NOT EXISTS idx_threads_community_id ON threads(community_id);
CREATE INDEX IF NOT EXISTS idx_threads_author_id ON threads(author_id);
CREATE INDEX IF NOT EXISTS idx_threads_created_at ON threads(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_threads_last_reply_at ON threads(last_reply_at DESC);
CREATE INDEX IF NOT EXISTS idx_threads_upvote_count ON threads(upvote_count DESC);

CREATE INDEX IF NOT EXISTS idx_thread_replies_thread_id ON thread_replies(thread_id);
CREATE INDEX IF NOT EXISTS idx_thread_replies_author_id ON thread_replies(author_id);
CREATE INDEX IF NOT EXISTS idx_thread_replies_parent_id ON thread_replies(parent_reply_id);
CREATE INDEX IF NOT EXISTS idx_thread_replies_created_at ON thread_replies(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_community_memberships_community_id ON community_memberships(community_id);
CREATE INDEX IF NOT EXISTS idx_community_memberships_user_id ON community_memberships(user_id);
CREATE INDEX IF NOT EXISTS idx_community_memberships_role ON community_memberships(role);
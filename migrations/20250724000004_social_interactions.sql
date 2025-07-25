-- Create enums for social interactions
CREATE TYPE mute_type AS ENUM ('ALL', 'POSTS', 'COMMENTS', 'MENTIONS');
CREATE TYPE notification_type AS ENUM (
    'FOLLOW', 'UNFOLLOW', 'POST_LIKE', 'COMMENT_LIKE', 'POST_COMMENT', 'POST_REPLY', 
    'POST_SHARE', 'POST_REPOST', 'MENTION', 'THREAD_REPLY', 'THREAD_UPVOTE', 
    'THREAD_DOWNVOTE', 'THREAD_PIN', 'THREAD_LOCK', 'FORUM_MENTION', 
    'COMMUNITY_INVITE', 'COMMUNITY_JOIN', 'COMMUNITY_LEAVE', 'COMMUNITY_ROLE_CHANGE', 
    'COMMUNITY_BAN', 'SYSTEM_ANNOUNCEMENT', 'SECURITY_ALERT', 'POLICY_UPDATE'
);
CREATE TYPE notification_priority AS ENUM ('LOW', 'NORMAL', 'HIGH', 'URGENT');
CREATE TYPE feed_type AS ENUM ('HOME', 'DISCOVER', 'COMMUNITY', 'FORUM', 'TRENDING', 'RECENT');
CREATE TYPE feed_algorithm AS ENUM ('CHRONOLOGICAL', 'ENGAGEMENT', 'RELEVANCE', 'COOPERATIVE', 'MIXED');
CREATE TYPE feed_content_type AS ENUM ('POST', 'THREAD', 'COMMENT', 'COMMUNITY', 'USER');
CREATE TYPE vote_target_type AS ENUM ('THREAD', 'THREAD_REPLY');
CREATE TYPE vote_type AS ENUM ('UPVOTE', 'DOWNVOTE');
CREATE TYPE moderation_target_type AS ENUM ('POST', 'COMMENT', 'THREAD', 'THREAD_REPLY', 'USER', 'COMMUNITY', 'FORUM');
CREATE TYPE moderation_action_type AS ENUM ('PIN', 'UNPIN', 'LOCK', 'UNLOCK', 'DELETE', 'RESTORE', 'HIDE', 'UNHIDE', 'BAN', 'UNBAN', 'MUTE', 'UNMUTE', 'WARN', 'REMOVE');
CREATE TYPE activity_type AS ENUM ('VIEW', 'LIKE', 'COMMENT', 'SHARE', 'FOLLOW', 'UPVOTE', 'DOWNVOTE', 'JOIN', 'LEAVE', 'CREATE', 'EDIT', 'DELETE');

-- Create follows table (enhanced relationship tracking)
CREATE TABLE IF NOT EXISTS follows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_mutual BOOLEAN NOT NULL DEFAULT FALSE,
    notification_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, followed_id)
);

-- Create blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    blocker_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    block_reason TEXT,
    is_permanent BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_id, blocked_id)
);

-- Create mutes table
CREATE TABLE IF NOT EXISTS mutes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    muter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    muted_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    mute_type mute_type NOT NULL DEFAULT 'ALL',
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(muter_id, muted_id, mute_type)
);

-- Create notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipient_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(id) ON DELETE SET NULL,
    notification_type notification_type NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    data JSONB DEFAULT '{}',
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    is_dismissed BOOLEAN NOT NULL DEFAULT FALSE,
    priority notification_priority NOT NULL DEFAULT 'NORMAL',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_at TIMESTAMPTZ
);

-- Create feeds table
CREATE TABLE IF NOT EXISTS feeds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feed_type feed_type NOT NULL,
    algorithm feed_algorithm NOT NULL DEFAULT 'CHRONOLOGICAL',
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, feed_type)
);

-- Create feed_items table
CREATE TABLE IF NOT EXISTS feed_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feed_id UUID NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    content_type feed_content_type NOT NULL,
    content_id UUID NOT NULL,
    score REAL NOT NULL DEFAULT 0.0,
    position INTEGER NOT NULL,
    reason VARCHAR(255) NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create feed_settings table
CREATE TABLE IF NOT EXISTS feed_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    show_reposts BOOLEAN NOT NULL DEFAULT TRUE,
    show_likes BOOLEAN NOT NULL DEFAULT TRUE,
    show_follows BOOLEAN NOT NULL DEFAULT TRUE,
    nsfw_filter BOOLEAN NOT NULL DEFAULT TRUE,
    language_filter TEXT[] DEFAULT '{}',
    blocked_keywords TEXT[] DEFAULT '{}',
    preferred_content_types feed_content_type[] DEFAULT '{POST,THREAD}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

-- Create votes table (for forum voting)
CREATE TABLE IF NOT EXISTS votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type vote_target_type NOT NULL,
    target_id UUID NOT NULL,
    vote_type vote_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, target_type, target_id)
);

-- Create moderation_actions table
CREATE TABLE IF NOT EXISTS moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    moderator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type moderation_target_type NOT NULL,
    target_id UUID NOT NULL,
    action_type moderation_action_type NOT NULL,
    reason TEXT,
    duration INTEGER, -- Duration in hours for temporary actions
    is_reversed BOOLEAN NOT NULL DEFAULT FALSE,
    reversed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    reversed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create user_activities table
CREATE TABLE IF NOT EXISTS user_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_type activity_type NOT NULL,
    target_type VARCHAR(255) NOT NULL,
    target_id UUID NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_followed_id ON follows(followed_id);
CREATE INDEX IF NOT EXISTS idx_follows_mutual ON follows(is_mutual);

CREATE INDEX IF NOT EXISTS idx_blocks_blocker_id ON blocks(blocker_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocked_id ON blocks(blocked_id);
CREATE INDEX IF NOT EXISTS idx_blocks_expires_at ON blocks(expires_at);

CREATE INDEX IF NOT EXISTS idx_mutes_muter_id ON mutes(muter_id);
CREATE INDEX IF NOT EXISTS idx_mutes_muted_id ON mutes(muted_id);
CREATE INDEX IF NOT EXISTS idx_mutes_type ON mutes(mute_type);

CREATE INDEX IF NOT EXISTS idx_notifications_recipient_id ON notifications(recipient_id);
CREATE INDEX IF NOT EXISTS idx_notifications_sender_id ON notifications(sender_id);
CREATE INDEX IF NOT EXISTS idx_notifications_type ON notifications(notification_type);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notifications_unread ON notifications(recipient_id, is_read) WHERE is_read = FALSE;

CREATE INDEX IF NOT EXISTS idx_feeds_user_id ON feeds(user_id);
CREATE INDEX IF NOT EXISTS idx_feeds_type ON feeds(feed_type);
CREATE INDEX IF NOT EXISTS idx_feeds_last_updated ON feeds(last_updated DESC);

CREATE INDEX IF NOT EXISTS idx_feed_items_feed_id ON feed_items(feed_id);
CREATE INDEX IF NOT EXISTS idx_feed_items_content ON feed_items(content_type, content_id);
CREATE INDEX IF NOT EXISTS idx_feed_items_position ON feed_items(feed_id, position);
CREATE INDEX IF NOT EXISTS idx_feed_items_score ON feed_items(score DESC);

CREATE INDEX IF NOT EXISTS idx_feed_settings_user_id ON feed_settings(user_id);

CREATE INDEX IF NOT EXISTS idx_votes_user_id ON votes(user_id);
CREATE INDEX IF NOT EXISTS idx_votes_target ON votes(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_votes_type ON votes(vote_type);

CREATE INDEX IF NOT EXISTS idx_moderation_actions_moderator_id ON moderation_actions(moderator_id);
CREATE INDEX IF NOT EXISTS idx_moderation_actions_target ON moderation_actions(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_moderation_actions_type ON moderation_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_moderation_actions_created_at ON moderation_actions(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_user_activities_user_id ON user_activities(user_id);
CREATE INDEX IF NOT EXISTS idx_user_activities_type ON user_activities(activity_type);
CREATE INDEX IF NOT EXISTS idx_user_activities_target ON user_activities(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_user_activities_created_at ON user_activities(created_at DESC);
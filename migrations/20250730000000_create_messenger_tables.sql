-- Create tables for the Messenger application

-- Conversations table
CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_group BOOLEAN NOT NULL DEFAULT false,
    group_name VARCHAR(255)
);

-- Participants table
CREATE TABLE IF NOT EXISTS participants (
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_read_message_id UUID,
    can_send_messages BOOLEAN NOT NULL DEFAULT true,
    can_manage_participants BOOLEAN NOT NULL DEFAULT false,
    can_change_settings BOOLEAN NOT NULL DEFAULT false,
    can_delete_messages BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (conversation_id, user_id)
);

-- Index for faster participant lookups
CREATE INDEX IF NOT EXISTS idx_participants_user_id ON participants(user_id);

-- Messages table
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL,
    content_type VARCHAR(20) NOT NULL, -- 'text', 'media', 'system'
    content_text TEXT,
    media_id UUID,
    sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    status_code INTEGER NOT NULL DEFAULT 0, -- 0: Pending, 1: Sent, 2: Delivered, 3: Read
    delivered_at TIMESTAMP WITH TIME ZONE,
    read_at TIMESTAMP WITH TIME ZONE
);

-- Index for faster message lookups
CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_sender_id ON messages(sender_id);
CREATE INDEX IF NOT EXISTS idx_messages_sent_at ON messages(sent_at);

-- Media table
CREATE TABLE IF NOT EXISTS media (
    id UUID PRIMARY KEY,
    media_type VARCHAR(20) NOT NULL, -- 'Image', 'Document', 'Audio', 'Video'
    storage_location TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    filename VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for faster media lookups
CREATE INDEX IF NOT EXISTS idx_media_created_at ON media(created_at);

-- Enhance media table with encryption fields
ALTER TABLE media
  ADD COLUMN encryption_key BYTEA,
  ADD COLUMN iv BYTEA,
  ADD COLUMN thumbnail_id UUID,
  ADD COLUMN original_filename VARCHAR(255);

-- Reactions table for message interactions
CREATE TABLE IF NOT EXISTS reactions (
    id UUID PRIMARY KEY,
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    reaction_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reactions_message_id ON reactions(message_id);
CREATE INDEX IF NOT EXISTS idx_reactions_user_id ON reactions(user_id);

-- Message threading table
CREATE TABLE IF NOT EXISTS message_threads (
    id UUID PRIMARY KEY,
    parent_message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    child_message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    depth INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_message_threads_unique
  ON message_threads (parent_message_id, child_message_id);
CREATE INDEX IF NOT EXISTS idx_message_threads_parent ON message_threads(parent_message_id);
CREATE INDEX IF NOT EXISTS idx_message_threads_child ON message_threads(child_message_id);
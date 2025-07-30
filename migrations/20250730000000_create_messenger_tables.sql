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
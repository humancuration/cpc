-- Create reactions table for message reactions system
CREATE TABLE reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for efficient querying of reactions
CREATE INDEX idx_reactions_message_id ON reactions(message_id);
CREATE INDEX idx_reactions_user_id ON reactions(user_id);

-- Create message_threads table for threaded conversations
CREATE TABLE message_threads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    root_message_id UUID REFERENCES messages(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for efficient querying of threads
CREATE INDEX idx_message_threads_conversation_id ON message_threads(conversation_id);
CREATE INDEX idx_message_threads_parent_message_id ON message_threads(parent_message_id);

-- Add thread_id column to messages table for threaded conversations
ALTER TABLE messages ADD COLUMN thread_id UUID REFERENCES message_threads(id) ON DELETE SET NULL;

-- Create media table for media sharing pipeline
CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    media_type VARCHAR(20) NOT NULL,
    storage_path TEXT NOT NULL,
    encryption_key BYTEA NOT NULL,
    iv BYTEA NOT NULL,
    thumbnail_id UUID REFERENCES media(id) ON DELETE SET NULL,
    original_filename VARCHAR(255),
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for efficient querying of media
CREATE INDEX idx_media_owner_id ON media(owner_id);

-- Add enhanced permissions to participants table for advanced group management
ALTER TABLE participants ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_manage_participants BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_change_settings BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_delete_messages BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_moderate_content BOOLEAN NOT NULL DEFAULT FALSE;

-- Create indexes for efficient querying of participant permissions
CREATE INDEX idx_participants_conversation_id ON participants(conversation_id);
CREATE INDEX idx_participants_user_id ON participants(user_id);
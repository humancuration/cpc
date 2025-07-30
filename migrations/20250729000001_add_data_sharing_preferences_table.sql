-- Create data_sharing_preferences table
CREATE TABLE IF NOT EXISTS data_sharing_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    data_sharing_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    anonymized_data BOOLEAN NOT NULL DEFAULT FALSE,
    preferred_currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_data_sharing_preferences_user_id ON data_sharing_preferences(user_id);
CREATE INDEX IF NOT EXISTS idx_data_sharing_preferences_currency ON data_sharing_preferences(preferred_currency);
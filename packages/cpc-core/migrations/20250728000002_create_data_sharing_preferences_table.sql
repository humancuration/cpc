-- Create data_sharing_preferences table for privacy-preserving UBI integration
CREATE TABLE data_sharing_preferences (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    data_sharing_enabled BOOLEAN NOT NULL DEFAULT false,
    anonymized_data BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

CREATE INDEX idx_data_sharing_user ON data_sharing_preferences(user_id);
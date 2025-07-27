-- Up
CREATE TABLE IF NOT EXISTS user_consents (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    consent_type TEXT NOT NULL CHECK (consent_type IN ('playback', 'recommendations', 'social', 'following', 'offline_download')),
    granted BOOLEAN NOT NULL DEFAULT false,
    expires_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, consent_type)
);

CREATE INDEX IF NOT EXISTS idx_user_consents_user_id ON user_consents(user_id);
CREATE INDEX IF NOT EXISTS idx_user_consents_expires_at ON user_consents(expires_at) WHERE granted = true AND expires_at IS NOT NULL;

-- Down
DROP TABLE IF EXISTS user_consents;
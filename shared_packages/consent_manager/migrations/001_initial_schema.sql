-- Initial schema for consent manager

-- Consent profiles table
CREATE TABLE IF NOT EXISTS consent_profiles (
    user_id TEXT NOT NULL,
    domain TEXT NOT NULL,
    level TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, domain)
);

-- Index for faster user lookups
CREATE INDEX IF NOT EXISTS idx_consent_profiles_user_id ON consent_profiles (user_id);

-- Consent audit log table
CREATE TABLE IF NOT EXISTS consent_audit_log (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    domain TEXT NOT NULL,
    action TEXT NOT NULL,
    previous_level TEXT,
    new_level TEXT NOT NULL,
    actor_type TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for faster user lookups
CREATE INDEX IF NOT EXISTS idx_consent_audit_user_id ON consent_audit_log (user_id);

-- Index for faster timestamp lookups
CREATE INDEX IF NOT EXISTS idx_consent_audit_timestamp ON consent_audit_log (timestamp);
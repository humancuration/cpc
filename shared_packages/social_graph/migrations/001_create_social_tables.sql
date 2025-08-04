-- Migration script for creating social graph tables

-- Create users table
CREATE TABLE IF NOT EXISTS social_users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create relationships table
CREATE TABLE IF NOT EXISTS social_relationships (
    id UUID PRIMARY KEY,
    source_user_id UUID NOT NULL REFERENCES social_users(id),
    target_user_id UUID NOT NULL REFERENCES social_users(id),
    relationship_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Ensure a user can't have multiple relationships with the same target
    UNIQUE(source_user_id, target_user_id),
    
    -- Ensure source and target are different
    CHECK(source_user_id != target_user_id)
);

-- Create activities table
CREATE TABLE IF NOT EXISTS social_activities (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES social_users(id),
    activity_type VARCHAR(50) NOT NULL,
    target_id UUID,
    target_type VARCHAR(50),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_public BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_social_relationships_source ON social_relationships(source_user_id);
CREATE INDEX IF NOT EXISTS idx_social_relationships_target ON social_relationships(target_user_id);
CREATE INDEX IF NOT EXISTS idx_social_relationships_type ON social_relationships(relationship_type);
CREATE INDEX IF NOT EXISTS idx_social_activities_user ON social_activities(user_id);
CREATE INDEX IF NOT EXISTS idx_social_activities_type ON social_activities(activity_type);
CREATE INDEX IF NOT EXISTS idx_social_activities_created ON social_activities(created_at);
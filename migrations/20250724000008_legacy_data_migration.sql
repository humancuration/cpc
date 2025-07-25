-- Legacy Data Migration Script
-- This script provides the framework for migrating data from the legacy Android database format
-- to the new unified schema. Specific migration logic will need to be implemented based on
-- the actual legacy schema structure.

-- Create temporary tables for legacy data import if needed
-- These would be used to stage data from the legacy Android database

-- Example: Create temporary staging table for legacy users
CREATE TABLE IF NOT EXISTS temp_legacy_users (
    legacy_id VARCHAR(255),
    username VARCHAR(255),
    email VARCHAR(255),
    password_hash TEXT,
    display_name VARCHAR(255),
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    -- Add other legacy fields as needed
    legacy_data JSONB -- Store any additional legacy data
);

-- Example: Create temporary staging table for legacy posts/content
CREATE TABLE IF NOT EXISTS temp_legacy_posts (
    legacy_id VARCHAR(255),
    author_legacy_id VARCHAR(255),
    content TEXT,
    post_type VARCHAR(50),
    visibility VARCHAR(50),
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    legacy_data JSONB
);

-- Example: Create temporary staging table for legacy relationships
CREATE TABLE IF NOT EXISTS temp_legacy_relationships (
    follower_legacy_id VARCHAR(255),
    followed_legacy_id VARCHAR(255),
    relationship_type VARCHAR(50),
    created_at TIMESTAMPTZ,
    legacy_data JSONB
);

-- Create a mapping table to track legacy ID to new UUID mappings
CREATE TABLE IF NOT EXISTS legacy_id_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL, -- 'user', 'post', 'community', etc.
    legacy_id VARCHAR(255) NOT NULL,
    new_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(entity_type, legacy_id)
);

-- Function to generate UUID mapping for legacy IDs
CREATE OR REPLACE FUNCTION get_or_create_uuid_for_legacy_id(
    p_entity_type VARCHAR(50),
    p_legacy_id VARCHAR(255)
) RETURNS UUID AS $$
DECLARE
    v_uuid UUID;
BEGIN
    -- Try to get existing mapping
    SELECT new_id INTO v_uuid
    FROM legacy_id_mappings
    WHERE entity_type = p_entity_type AND legacy_id = p_legacy_id;
    
    -- If not found, create new mapping
    IF v_uuid IS NULL THEN
        v_uuid := gen_random_uuid();
        INSERT INTO legacy_id_mappings (entity_type, legacy_id, new_id)
        VALUES (p_entity_type, p_legacy_id, v_uuid);
    END IF;
    
    RETURN v_uuid;
END;
$$ LANGUAGE plpgsql;

-- Example migration functions (to be implemented based on actual legacy schema)

-- Function to migrate legacy users
CREATE OR REPLACE FUNCTION migrate_legacy_users() RETURNS INTEGER AS $$
DECLARE
    migration_count INTEGER := 0;
    legacy_user RECORD;
    new_user_id UUID;
BEGIN
    -- Iterate through legacy users and migrate them
    FOR legacy_user IN SELECT * FROM temp_legacy_users LOOP
        -- Get or create UUID for this legacy user
        new_user_id := get_or_create_uuid_for_legacy_id('user', legacy_user.legacy_id);
        
        -- Insert into users table (handle conflicts)
        INSERT INTO users (
            id, username, email, password_hash, display_name, bio, avatar_url,
            created_at, updated_at
        ) VALUES (
            new_user_id,
            legacy_user.username,
            legacy_user.email,
            legacy_user.password_hash,
            legacy_user.display_name,
            legacy_user.bio,
            legacy_user.avatar_url,
            COALESCE(legacy_user.created_at, NOW()),
            COALESCE(legacy_user.updated_at, NOW())
        ) ON CONFLICT (id) DO NOTHING;
        
        -- Create user profile if display_name exists
        IF legacy_user.display_name IS NOT NULL THEN
            INSERT INTO user_profiles (
                user_id, display_name, bio, avatar_url, created_at, updated_at
            ) VALUES (
                new_user_id,
                legacy_user.display_name,
                legacy_user.bio,
                legacy_user.avatar_url,
                COALESCE(legacy_user.created_at, NOW()),
                COALESCE(legacy_user.updated_at, NOW())
            ) ON CONFLICT (user_id) DO NOTHING;
        END IF;
        
        migration_count := migration_count + 1;
    END LOOP;
    
    RETURN migration_count;
END;
$$ LANGUAGE plpgsql;

-- Function to migrate legacy posts/content
CREATE OR REPLACE FUNCTION migrate_legacy_posts() RETURNS INTEGER AS $$
DECLARE
    migration_count INTEGER := 0;
    legacy_post RECORD;
    new_post_id UUID;
    author_id UUID;
BEGIN
    FOR legacy_post IN SELECT * FROM temp_legacy_posts LOOP
        -- Get UUIDs for post and author
        new_post_id := get_or_create_uuid_for_legacy_id('post', legacy_post.legacy_id);
        author_id := get_or_create_uuid_for_legacy_id('user', legacy_post.author_legacy_id);
        
        -- Insert into posts table
        INSERT INTO posts (
            id, author_id, content, visibility, created_at, updated_at
        ) VALUES (
            new_post_id,
            author_id,
            legacy_post.content,
            CASE 
                WHEN legacy_post.visibility = 'public' THEN 'PUBLIC'::visibility
                WHEN legacy_post.visibility = 'private' THEN 'PRIVATE'::visibility
                ELSE 'PUBLIC'::visibility
            END,
            COALESCE(legacy_post.created_at, NOW()),
            COALESCE(legacy_post.updated_at, NOW())
        ) ON CONFLICT (id) DO NOTHING;
        
        migration_count := migration_count + 1;
    END LOOP;
    
    RETURN migration_count;
END;
$$ LANGUAGE plpgsql;

-- Function to migrate legacy relationships
CREATE OR REPLACE FUNCTION migrate_legacy_relationships() RETURNS INTEGER AS $$
DECLARE
    migration_count INTEGER := 0;
    legacy_rel RECORD;
    follower_id UUID;
    followed_id UUID;
BEGIN
    FOR legacy_rel IN SELECT * FROM temp_legacy_relationships LOOP
        -- Get UUIDs for users
        follower_id := get_or_create_uuid_for_legacy_id('user', legacy_rel.follower_legacy_id);
        followed_id := get_or_create_uuid_for_legacy_id('user', legacy_rel.followed_legacy_id);
        
        -- Insert into appropriate relationship table based on type
        IF legacy_rel.relationship_type = 'follow' THEN
            INSERT INTO follows (follower_id, followed_id, created_at)
            VALUES (follower_id, followed_id, COALESCE(legacy_rel.created_at, NOW()))
            ON CONFLICT (follower_id, followed_id) DO NOTHING;
        ELSIF legacy_rel.relationship_type = 'block' THEN
            INSERT INTO blocks (blocker_id, blocked_id, created_at)
            VALUES (follower_id, followed_id, COALESCE(legacy_rel.created_at, NOW()))
            ON CONFLICT (blocker_id, blocked_id) DO NOTHING;
        END IF;
        
        migration_count := migration_count + 1;
    END LOOP;
    
    RETURN migration_count;
END;
$$ LANGUAGE plpgsql;

-- Main migration orchestration function
CREATE OR REPLACE FUNCTION run_legacy_data_migration() RETURNS TEXT AS $$
DECLARE
    user_count INTEGER;
    post_count INTEGER;
    relationship_count INTEGER;
    result_text TEXT;
BEGIN
    -- Run migrations in dependency order
    user_count := migrate_legacy_users();
    post_count := migrate_legacy_posts();
    relationship_count := migrate_legacy_relationships();
    
    result_text := format(
        'Legacy data migration completed: %s users, %s posts, %s relationships migrated',
        user_count, post_count, relationship_count
    );
    
    RETURN result_text;
END;
$$ LANGUAGE plpgsql;

-- Create indexes for legacy mapping table
CREATE INDEX IF NOT EXISTS idx_legacy_id_mappings_entity_type ON legacy_id_mappings(entity_type);
CREATE INDEX IF NOT EXISTS idx_legacy_id_mappings_legacy_id ON legacy_id_mappings(legacy_id);
CREATE INDEX IF NOT EXISTS idx_legacy_id_mappings_new_id ON legacy_id_mappings(new_id);

-- Add comments for documentation
COMMENT ON TABLE legacy_id_mappings IS 'Maps legacy Android database IDs to new UUIDs for data migration';
COMMENT ON FUNCTION get_or_create_uuid_for_legacy_id IS 'Gets existing UUID mapping or creates new one for legacy ID';
COMMENT ON FUNCTION run_legacy_data_migration IS 'Main function to orchestrate legacy data migration from Android database';

-- Note: To use this migration system:
-- 1. Load legacy data into temp_legacy_* tables
-- 2. Run SELECT run_legacy_data_migration();
-- 3. Verify migration results
-- 4. Drop temp tables when migration is complete
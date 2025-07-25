-- Database Schema Versioning and Upgrade System
-- This migration creates the infrastructure for tracking schema versions and managing upgrades

-- Create schema_versions table to track applied migrations
CREATE TABLE IF NOT EXISTS schema_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    migration_file VARCHAR(255) NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    applied_by VARCHAR(255) DEFAULT current_user,
    checksum VARCHAR(64), -- SHA-256 checksum of migration file
    execution_time_ms INTEGER,
    success BOOLEAN NOT NULL DEFAULT TRUE,
    error_message TEXT
);

-- Create schema_upgrade_log table for detailed upgrade tracking
CREATE TABLE IF NOT EXISTS schema_upgrade_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    upgrade_session_id UUID NOT NULL,
    from_version VARCHAR(50),
    to_version VARCHAR(50) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    success BOOLEAN,
    error_message TEXT,
    rollback_performed BOOLEAN DEFAULT FALSE,
    rollback_completed_at TIMESTAMPTZ
);

-- Create database_metadata table for general database information
CREATE TABLE IF NOT EXISTS database_metadata (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(255) DEFAULT current_user
);

-- Insert initial metadata
INSERT INTO database_metadata (key, value, description) VALUES
    ('schema_version', '20250724000009', 'Current database schema version'),
    ('created_at', NOW()::TEXT, 'Database creation timestamp'),
    ('migration_system_version', '1.0', 'Version of the migration system'),
    ('database_name', current_database(), 'Name of the database'),
    ('last_migration_check', NOW()::TEXT, 'Last time migrations were checked')
ON CONFLICT (key) DO UPDATE SET 
    value = EXCLUDED.value,
    updated_at = NOW();

-- Function to get current schema version
CREATE OR REPLACE FUNCTION get_current_schema_version() RETURNS VARCHAR(50) AS $$
BEGIN
    RETURN (SELECT value FROM database_metadata WHERE key = 'schema_version');
END;
$$ LANGUAGE plpgsql;

-- Function to record migration application
CREATE OR REPLACE FUNCTION record_migration(
    p_version VARCHAR(50),
    p_description TEXT,
    p_migration_file VARCHAR(255),
    p_checksum VARCHAR(64) DEFAULT NULL,
    p_execution_time_ms INTEGER DEFAULT NULL,
    p_success BOOLEAN DEFAULT TRUE,
    p_error_message TEXT DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    INSERT INTO schema_versions (
        version, description, migration_file, checksum, 
        execution_time_ms, success, error_message
    ) VALUES (
        p_version, p_description, p_migration_file, p_checksum,
        p_execution_time_ms, p_success, p_error_message
    );
    
    -- Update current schema version in metadata
    IF p_success THEN
        UPDATE database_metadata 
        SET value = p_version, updated_at = NOW()
        WHERE key = 'schema_version';
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to check if migration has been applied
CREATE OR REPLACE FUNCTION is_migration_applied(p_version VARCHAR(50)) RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM schema_versions 
        WHERE version = p_version AND success = TRUE
    );
END;
$$ LANGUAGE plpgsql;

-- Function to start upgrade session
CREATE OR REPLACE FUNCTION start_upgrade_session(
    p_from_version VARCHAR(50),
    p_to_version VARCHAR(50)
) RETURNS UUID AS $$
DECLARE
    session_id UUID;
BEGIN
    session_id := gen_random_uuid();
    
    INSERT INTO schema_upgrade_log (
        upgrade_session_id, from_version, to_version
    ) VALUES (
        session_id, p_from_version, p_to_version
    );
    
    RETURN session_id;
END;
$$ LANGUAGE plpgsql;

-- Function to complete upgrade session
CREATE OR REPLACE FUNCTION complete_upgrade_session(
    p_session_id UUID,
    p_success BOOLEAN,
    p_error_message TEXT DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    UPDATE schema_upgrade_log 
    SET 
        completed_at = NOW(),
        success = p_success,
        error_message = p_error_message
    WHERE upgrade_session_id = p_session_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get migration history
CREATE OR REPLACE FUNCTION get_migration_history() 
RETURNS TABLE(
    version VARCHAR(50),
    description TEXT,
    applied_at TIMESTAMPTZ,
    success BOOLEAN,
    execution_time_ms INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        sv.version,
        sv.description,
        sv.applied_at,
        sv.success,
        sv.execution_time_ms
    FROM schema_versions sv
    ORDER BY sv.applied_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Function to validate database integrity
CREATE OR REPLACE FUNCTION validate_database_integrity() RETURNS TEXT AS $$
DECLARE
    result_text TEXT := '';
    table_count INTEGER;
    index_count INTEGER;
    constraint_count INTEGER;
BEGIN
    -- Count tables
    SELECT COUNT(*) INTO table_count
    FROM information_schema.tables
    WHERE table_schema = 'public' AND table_type = 'BASE TABLE';
    
    -- Count indexes
    SELECT COUNT(*) INTO index_count
    FROM pg_indexes
    WHERE schemaname = 'public';
    
    -- Count constraints
    SELECT COUNT(*) INTO constraint_count
    FROM information_schema.table_constraints
    WHERE table_schema = 'public';
    
    result_text := format(
        'Database integrity check: %s tables, %s indexes, %s constraints',
        table_count, index_count, constraint_count
    );
    
    -- Update last check time
    UPDATE database_metadata 
    SET value = NOW()::TEXT, updated_at = NOW()
    WHERE key = 'last_migration_check';
    
    RETURN result_text;
END;
$$ LANGUAGE plpgsql;

-- Record all existing migrations in the schema_versions table
DO $$
DECLARE
    migration_record RECORD;
    migrations TEXT[][] := ARRAY[
        ['20250722000000', 'Initial auth tables', '20250722000000_auth_tables.sql'],
        ['20250724000001', 'User enhancements and relationships', '20250724000001_user_enhancements.sql'],
        ['20250724000002', 'Social core models', '20250724000002_social_core.sql'],
        ['20250724000003', 'Forum system', '20250724000003_forum_system.sql'],
        ['20250724000004', 'Social interactions', '20250724000004_social_interactions.sql'],
        ['20250724000005', 'Governance system', '20250724000005_governance_system.sql'],
        ['20250724000006', 'Product system', '20250724000006_product_system.sql'],
        ['20250724000007', 'Enhanced tokens', '20250724000007_enhanced_tokens.sql'],
        ['20250724000008', 'Legacy data migration', '20250724000008_legacy_data_migration.sql'],
        ['20250724000009', 'Schema versioning system', '20250724000009_schema_versioning.sql']
    ];
BEGIN
    FOREACH migration_record IN ARRAY migrations LOOP
        PERFORM record_migration(
            migration_record[1],
            migration_record[2],
            migration_record[3],
            NULL, -- checksum
            NULL, -- execution_time_ms
            TRUE, -- success
            NULL  -- error_message
        );
    END LOOP;
END $$;

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_schema_versions_version ON schema_versions(version);
CREATE INDEX IF NOT EXISTS idx_schema_versions_applied_at ON schema_versions(applied_at DESC);
CREATE INDEX IF NOT EXISTS idx_schema_versions_success ON schema_versions(success);

CREATE INDEX IF NOT EXISTS idx_schema_upgrade_log_session_id ON schema_upgrade_log(upgrade_session_id);
CREATE INDEX IF NOT EXISTS idx_schema_upgrade_log_started_at ON schema_upgrade_log(started_at DESC);

CREATE INDEX IF NOT EXISTS idx_database_metadata_key ON database_metadata(key);

-- Add comments for documentation
COMMENT ON TABLE schema_versions IS 'Tracks all applied database migrations with metadata';
COMMENT ON TABLE schema_upgrade_log IS 'Logs database upgrade sessions for auditing';
COMMENT ON TABLE database_metadata IS 'Stores general database metadata and configuration';
COMMENT ON FUNCTION get_current_schema_version IS 'Returns the current database schema version';
COMMENT ON FUNCTION record_migration IS 'Records a migration as applied with metadata';
COMMENT ON FUNCTION validate_database_integrity IS 'Performs basic database integrity checks';
-- Create module registry tables
CREATE TABLE IF NOT EXISTS module_registry (
    module_name TEXT PRIMARY KEY,
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    enabled_at TIMESTAMP,
    dependencies JSONB NOT NULL DEFAULT '{}',
    version TEXT NOT NULL DEFAULT '0.0.0'
);

CREATE TABLE IF NOT EXISTS module_migrations (
    module_name TEXT NOT NULL,
    version TEXT NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (module_name, version)
);
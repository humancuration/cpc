//! Modular migration system for handling module-specific database migrations
//!
//! This module provides functionality to run migrations for individual modules
//! based on their enabled status and dependencies.

use sqlx::{Pool, Postgres};
use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::module_registry::ModuleRegistry;

/// Modular migration system that handles migrations for individual modules
pub struct MigrationSystem {
    pool: Pool<Postgres>,
}

impl MigrationSystem {
    /// Create a new migration system
    pub async fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Run migrations for all enabled modules
    pub async fn run_migrations(&self, registry: &ModuleRegistry) -> Result<()> {
        // Create migration tracking table if it doesn't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS module_migrations (
                module_name TEXT NOT NULL,
                version TEXT NOT NULL,
                applied_at TIMESTAMP NOT NULL DEFAULT NOW(),
                PRIMARY KEY (module_name, version)
            )"
        )
        .execute(&self.pool)
        .await?;
        
        // For each enabled module
        for module_name in registry.enabled_modules() {
            let migration_dir = format!("apps/{}/migrations", module_name);
            if Path::new(&migration_dir).exists() {
                // Get already applied migrations for this module
                let applied: Vec<String> = sqlx::query_scalar(
                    "SELECT version FROM module_migrations WHERE module_name = $1 ORDER BY applied_at"
                )
                .bind(module_name)
                .fetch_all(&self.pool)
                .await?;
                
                // Run new migrations in order
                let mut entries: Vec<_> = fs::read_dir(&migration_dir)?
                    .filter_map(|entry| entry.ok())
                    .collect();
                
                // Sort by filename to ensure consistent order
                entries.sort_by_key(|entry| entry.file_name());
                
                for entry in entries {
                    let path = entry.path();
                    if path.is_file() {
                        let version = path.file_stem().unwrap().to_str().unwrap();
                        if !applied.contains(&version.to_string()) {
                            let sql = fs::read_to_string(&path)?;
                            sqlx::query(&sql).execute(&self.pool).await?;
                            
                            // Record as applied
                            sqlx::query(
                                "INSERT INTO module_migrations (module_name, version) VALUES ($1, $2)"
                            )
                            .bind(module_name)
                            .bind(version)
                            .execute(&self.pool)
                            .await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
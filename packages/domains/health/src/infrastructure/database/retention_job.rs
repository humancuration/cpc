//! Audit Log Retention Job
//!
//! This module implements the background job for managing audit log retention
//! according to HIPAA requirements: 6-year retention with archival after 1 year.

use crate::infrastructure::database::audit_log_repository::AuditLogRepository;
use async_trait::async_trait;
use thiserror::Error;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc, Duration};
use sqlx::PgPool;

/// Error types for retention job operations
#[derive(Debug, Error)]
pub enum RetentionJobError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Audit log repository error: {0}")]
    AuditLogError(String),
}

/// Trait for retention job operations
#[async_trait]
pub trait RetentionJob: Send + Sync {
    /// Run the retention job to manage audit log lifecycle
    async fn run(&self) -> Result<(), RetentionJobError>;
}

/// Implementation of RetentionJob for audit logs
pub struct AuditLogRetentionJob {
    pool: PgPool,
    audit_log_repository: Box<dyn AuditLogRepository>,
}

impl AuditLogRetentionJob {
    pub fn new(pool: PgPool, audit_log_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { pool, audit_log_repository }
    }
    
    /// Archive audit logs older than 1 year
    async fn archive_old_logs(&self) -> Result<usize, RetentionJobError> {
        let one_year_ago = Utc::now() - Duration::days(365);
        
        // In a real implementation, this would move old logs to an archive table
        // For now, we'll just log that archiving would happen
        info!("Would archive audit logs older than: {}", one_year_ago);
        
        // Placeholder implementation - in reality this would:
        // 1. Select logs older than one year
        // 2. Move them to an archive table
        // 3. Return the count of archived logs
        
        Ok(0)
    }
    
    /// Delete audit logs older than 6 years
    async fn delete_expired_logs(&self) -> Result<usize, RetentionJobError> {
        let six_years_ago = Utc::now() - Duration::days(365 * 6);
        
        // Delete expired logs from database
        let result = sqlx::query!(
            r#"
            DELETE FROM audit_logs
            WHERE created_at < $1
            "#,
            six_years_ago
        )
        .execute(&self.pool)
        .await?;
        
        let deleted_count = result.rows_affected() as usize;
        info!("Deleted {} audit logs older than 6 years", deleted_count);
        
        Ok(deleted_count)
    }
}

#[async_trait]
impl RetentionJob for AuditLogRetentionJob {
    async fn run(&self) -> Result<(), RetentionJobError> {
        info!("Starting audit log retention job");
        
        // Archive logs older than 1 year
        match self.archive_old_logs().await {
            Ok(count) => {
                info!("Archived {} audit logs", count);
            }
            Err(e) => {
                warn!("Failed to archive old audit logs: {}", e);
            }
        }
        
        // Delete logs older than 6 years
        match self.delete_expired_logs().await {
            Ok(count) => {
                info!("Deleted {} expired audit logs", count);
            }
            Err(e) => {
                error!("Failed to delete expired audit logs: {}", e);
                return Err(e);
            }
        }
        
        info!("Audit log retention job completed");
        Ok(())
    }
}

/// Configuration for the retention job
#[derive(Debug, Clone)]
pub struct RetentionJobConfig {
    /// How often to run the retention job (in hours)
    pub run_interval_hours: i64,
    
    /// Whether to enable archiving of old logs
    pub enable_archiving: bool,
    
    /// Whether to enable deletion of expired logs
    pub enable_deletion: bool,
}

impl Default for RetentionJobConfig {
    fn default() -> Self {
        Self {
            run_interval_hours: 24, // Run once per day
            enable_archiving: true,
            enable_deletion: true,
        }
    }
}
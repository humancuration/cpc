//! Audit Log Repository
//!
//! This module provides the repository implementation for audit logs,
//! handling database operations for HIPAA compliance logging with access control.

use crate::domain::audit_log::{AccessAttemptType, AuditLog, ValidationError};
use async_trait::async_trait;
use sqlx::PgPool;
use thiserror::Error;
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

/// Error types for audit log repository operations
#[derive(Debug, Error)]
pub enum AuditLogRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    
    #[error("Audit log not found")]
    NotFound,
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Authentication required")]
    AuthenticationRequired,
    
    #[error("Dual authentication required")]
    DualAuthenticationRequired,
}

/// Record of an authentication attempt for pattern detection
#[derive(Debug, Clone)]
pub struct AuthenticationAttemptRecord {
    pub user_id: Option<Uuid>,
    pub ip_address: Option<IpAddr>,
    pub device_fingerprint: Option<String>,
    pub attempt_type: AccessAttemptType,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Uuid,
}

/// Repository trait for audit log operations
#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// Create a new audit log entry
    async fn create(&self, audit_log: AuditLog) -> Result<AuditLog, AuditLogRepositoryError>;
    
    /// Log an authentication attempt (doesn't require authentication itself)
    async fn log_authentication_attempt(
        &self,
        user_id: Option<Uuid>,
        attempt_type: String,
        correlation_id: Uuid,
        risk_score: u8,
        failure_reason: Option<String>,
        source_ip: Option<String>,
        device_info: Option<String>,
    ) -> Result<Uuid, AuditLogRepositoryError>;
    
    /// Get recent authentication attempts for pattern detection
    async fn get_recent_authentication_attempts(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        window_seconds: u64,
    ) -> Result<Vec<AuthenticationAttemptRecord>, AuditLogRepositoryError>;
    
    /// Find audit logs by user ID (admin access with dual authentication required)
    async fn find_by_user_id(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, AuditLogRepositoryError>;
    
    /// Find audit logs by data type (admin access with dual authentication required)
    async fn find_by_data_type(&self, data_type: &str, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, AuditLogRepositoryError>;
    
    /// Find audit logs within a time range (admin access with dual authentication required)
    async fn find_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        requester_id: Uuid,
        is_admin: bool,
        has_dual_auth: bool
    ) -> Result<Vec<AuditLog>, AuditLogRepositoryError>;
    
    /// Get audit log by ID (admin access with dual authentication required)
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<AuditLog, AuditLogRepositoryError>;
}

/// Implementation of AuditLogRepository for PostgreSQL
pub struct AuditLogRepositoryImpl {
    pool: PgPool,
}

impl AuditLogRepositoryImpl {
    /// Create a new audit log repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Check if the requester has admin privileges
    fn check_admin_access(&self, requester_id: Uuid, is_admin: bool) -> Result<(), AuditLogRepositoryError> {
        if !is_admin {
            warn!("Non-admin user {} attempted to access audit logs", requester_id);
            return Err(AuditLogRepositoryError::AccessDenied(
                "Admin privileges required to access audit logs".to_string()
            ));
        }
        Ok(())
    }
    
    /// Log dual authentication requirement
    async fn check_dual_auth(
        &self,
        has_dual_auth: bool,
        user_id: Option<Uuid>,
        source_ip: Option<String>,
        device_info: Option<String>,
        correlation_id: Uuid,
    ) -> Result<(), AuditLogRepositoryError> {
        if !has_dual_auth {
            // Log the failed authentication attempt
            // Risk score will be calculated by the attempt monitor
            let _ = self.log_authentication_attempt(
                user_id,
                "FailedDualAuth".to_string(),
                correlation_id,
                0, // Placeholder - will be updated with calculated risk score
                Some("Dual authentication required but not provided".to_string()),
                source_ip,
                device_info,
            ).await;
            
            return Err(AuditLogRepositoryError::DualAuthenticationRequired);
        }
        Ok(())
    }
}

#[async_trait]
impl AuditLogRepository for AuditLogRepositoryImpl {
    async fn create(&self, audit_log: AuditLog) -> Result<AuditLog, AuditLogRepositoryError> {
        // Validate the audit log entry
        audit_log.validate()?;
        
        // Log the audit event for tracing
        info!(
            event = "audit_log",
            user_id = ?audit_log.user_id,
            data_type = %audit_log.data_type,
            access_type = %audit_log.access_type,
            purpose = %audit_log.purpose
        );
        
        // Insert into database
        let result = sqlx::query!(
            r#"
            INSERT INTO audit_logs (
                id, user_id, accessed_at, data_type, data_id,
                access_type, purpose, source_ip, device_info, created_at,
                attempt_type, attempt_correlation_id, risk_score, failure_reason
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id
            "#,
            audit_log.id,
            audit_log.user_id,
            audit_log.accessed_at,
            audit_log.data_type,
            audit_log.data_id,
            audit_log.access_type,
            audit_log.purpose,
            audit_log.source_ip.map(|ip| ip.to_string()),
            audit_log.device_info,
            audit_log.created_at,
            audit_log.attempt_type,
            audit_log.attempt_correlation_id,
            audit_log.risk_score,
            audit_log.failure_reason
        )
        .fetch_one(&self.pool)
        .await;
        
        match result {
            Ok(_) => {
                Ok(audit_log)
            }
            Err(e) => {
                error!("Failed to create audit log: {}", e);
                Err(AuditLogRepositoryError::DatabaseError(e))
            }
        }
    }
    
    async fn log_authentication_attempt(
        &self,
        user_id: Option<Uuid>,
        attempt_type: String,
        correlation_id: Uuid,
        risk_score: u8,
        failure_reason: Option<String>,
        source_ip: Option<String>,
        device_info: Option<String>,
    ) -> Result<Uuid, AuditLogRepositoryError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // Log the authentication attempt for tracing
        info!(
            event = "auth_attempt",
            user_id = ?user_id,
            attempt_type = %attempt_type,
            correlation_id = %correlation_id,
            risk_score = %risk_score
        );
        
        // Insert into database
        let result = sqlx::query!(
            r#"
            INSERT INTO audit_logs (
                id, user_id, accessed_at, data_type, data_id,
                access_type, purpose, source_ip, device_info, created_at,
                attempt_type, attempt_correlation_id, risk_score, failure_reason
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id
            "#,
            id,
            user_id,
            now,
            "AuthenticationAttempt", // data_type
            Uuid::nil(), // data_id
            "Authentication", // access_type
            "SecurityMonitoring", // purpose
            source_ip,
            device_info,
            now, // created_at
            attempt_type,
            correlation_id,
            risk_score,
            failure_reason
        )
        .fetch_one(&self.pool)
        .await;
        
        match result {
            Ok(row) => {
                Ok(row.id)
            }
            Err(e) => {
                error!("Failed to log authentication attempt: {}", e);
                Err(AuditLogRepositoryError::DatabaseError(e))
            }
        }
    }
    
    /// Get recent authentication attempts for pattern detection
    async fn get_recent_authentication_attempts(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        window_seconds: u64,
    ) -> Result<Vec<AuthenticationAttemptRecord>, AuditLogRepositoryError> {
        let since = Utc::now() - chrono::Duration::seconds(window_seconds as i64);
        
        let query = if let Some(uid) = user_id {
            sqlx::query!(
                r#"
                SELECT user_id, source_ip, device_info, attempt_type, accessed_at, attempt_correlation_id
                FROM audit_logs
                WHERE user_id = $1 
                AND attempt_type IS NOT NULL
                AND accessed_at >= $2
                ORDER BY accessed_at DESC
                "#,
                uid,
                since
            )
        } else if let Some(ip) = ip_address {
            sqlx::query!(
                r#"
                SELECT user_id, source_ip, device_info, attempt_type, accessed_at, attempt_correlation_id
                FROM audit_logs
                WHERE source_ip = $1::TEXT
                AND attempt_type IS NOT NULL
                AND accessed_at >= $2
                AND source_ip IS NOT NULL
                ORDER BY accessed_at DESC
                "#,
                ip.to_string(),
                since
            )
        } else {
            return Ok(Vec::new());
        };
        
        let rows = query.fetch_all(&self.pool).await?;
        
        let mut attempts = Vec::new();
        for row in rows {
            // Convert attempt_type string to AccessAttemptType enum
            let attempt_type = match row.attempt_type.as_deref() {
                Some("Success") => AccessAttemptType::Success,
                Some("FailedDualAuth") => AccessAttemptType::FailedDualAuth,
                Some("InvalidCredentials") => AccessAttemptType::InvalidCredentials,
                Some("AccountLocked") => AccessAttemptType::AccountLocked,
                _ => AccessAttemptType::Success, // Default to Success for unknown types
            };
            
            attempts.push(AuthenticationAttemptRecord {
                user_id: row.user_id,
                ip_address: row.source_ip.and_then(|ip| ip.parse().ok()),
                device_fingerprint: row.device_info,
                attempt_type,
                timestamp: row.accessed_at,
                correlation_id: row.attempt_correlation_id.unwrap_or_else(Uuid::new_v4),
            });
        }
        
        Ok(attempts)
    }
    
    async fn find_by_user_id(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, AuditLogRepositoryError> {
        // Check admin access
        self.check_admin_access(requester_id, is_admin)?;
        // Check dual authentication
        self.check_dual_auth(has_dual_auth, Some(requester_id), None, None, Uuid::new_v4()).await?;
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, accessed_at, data_type, data_id,
                   access_type, purpose, source_ip, device_info, created_at,
                   attempt_type, attempt_correlation_id, risk_score, failure_reason
            FROM audit_logs
            WHERE user_id = $1
            ORDER BY accessed_at DESC
            LIMIT 1000
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let audit_logs = rows.into_iter().map(|row| {
            AuditLog {
                id: row.id,
                user_id: row.user_id,
                accessed_at: row.accessed_at,
                data_type: row.data_type,
                data_id: row.data_id,
                access_type: row.access_type,
                purpose: row.purpose,
                source_ip: row.source_ip.and_then(|ip| ip.parse().ok()),
                device_info: row.device_info,
                created_at: row.created_at,
                attempt_type: row.attempt_type.unwrap_or_else(|| "Success".to_string()),
                attempt_correlation_id: row.attempt_correlation_id.unwrap_or_else(Uuid::new_v4),
                risk_score: row.risk_score.unwrap_or(0),
                failure_reason: row.failure_reason,
            }
        }).collect();
        
        Ok(audit_logs)
    }
    
    async fn find_by_data_type(&self, data_type: &str, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, AuditLogRepositoryError> {
        // Check admin access
        self.check_admin_access(requester_id, is_admin)?;
        // Check dual authentication
        self.check_dual_auth(has_dual_auth, Some(requester_id), None, None, Uuid::new_v4()).await?;
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, accessed_at, data_type, data_id,
                   access_type, purpose, source_ip, device_info, created_at,
                   attempt_type, attempt_correlation_id, risk_score, failure_reason
            FROM audit_logs
            WHERE data_type = $1
            ORDER BY accessed_at DESC
            LIMIT 1000
            "#,
            data_type
        )
        .fetch_all(&self.pool)
        .await?;
        
        let audit_logs = rows.into_iter().map(|row| {
            AuditLog {
                id: row.id,
                user_id: row.user_id,
                accessed_at: row.accessed_at,
                data_type: row.data_type,
                data_id: row.data_id,
                access_type: row.access_type,
                purpose: row.purpose,
                source_ip: row.source_ip.and_then(|ip| ip.parse().ok()),
                device_info: row.device_info,
                created_at: row.created_at,
                attempt_type: row.attempt_type.unwrap_or_else(|| "Success".to_string()),
                attempt_correlation_id: row.attempt_correlation_id.unwrap_or_else(Uuid::new_v4),
                risk_score: row.risk_score.unwrap_or(0),
                failure_reason: row.failure_reason,
            }
        }).collect();
        
        Ok(audit_logs)
    }
    
    async fn find_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        requester_id: Uuid,
        is_admin: bool,
        has_dual_auth: bool
    ) -> Result<Vec<AuditLog>, AuditLogRepositoryError> {
        // Check admin access
        self.check_admin_access(requester_id, is_admin)?;
        // Check dual authentication
        self.check_dual_auth(has_dual_auth, Some(requester_id), None, None, Uuid::new_v4()).await?;
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, accessed_at, data_type, data_id,
                   access_type, purpose, source_ip, device_info, created_at,
                   attempt_type, attempt_correlation_id, risk_score, failure_reason
            FROM audit_logs
            WHERE accessed_at >= $1 AND accessed_at <= $2
            ORDER BY accessed_at DESC
            LIMIT 1000
            "#,
            start,
            end
        )
        .fetch_all(&self.pool)
        .await?;
        
        let audit_logs = rows.into_iter().map(|row| {
            AuditLog {
                id: row.id,
                user_id: row.user_id,
                accessed_at: row.accessed_at,
                data_type: row.data_type,
                data_id: row.data_id,
                access_type: row.access_type,
                purpose: row.purpose,
                source_ip: row.source_ip.and_then(|ip| ip.parse().ok()),
                device_info: row.device_info,
                created_at: row.created_at,
                attempt_type: row.attempt_type.unwrap_or_else(|| "Success".to_string()),
                attempt_correlation_id: row.attempt_correlation_id.unwrap_or_else(Uuid::new_v4),
                risk_score: row.risk_score.unwrap_or(0),
                failure_reason: row.failure_reason,
            }
        }).collect();
        
        Ok(audit_logs)
    }
    
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<AuditLog, AuditLogRepositoryError> {
        // Check admin access
        self.check_admin_access(requester_id, is_admin)?;
        // Check dual authentication
        self.check_dual_auth(has_dual_auth, Some(requester_id), None, None, Uuid::new_v4()).await?;
        
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, accessed_at, data_type, data_id,
                   access_type, purpose, source_ip, device_info, created_at,
                   attempt_type, attempt_correlation_id, risk_score, failure_reason
            FROM audit_logs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let row = match row {
            Some(row) => row,
            None => return Err(AuditLogRepositoryError::NotFound),
        };
        
        let audit_log = AuditLog {
            id: row.id,
            user_id: row.user_id,
            accessed_at: row.accessed_at,
            data_type: row.data_type,
            data_id: row.data_id,
            access_type: row.access_type,
            purpose: row.purpose,
            source_ip: row.source_ip.and_then(|ip| ip.parse().ok()),
            device_info: row.device_info,
            created_at: row.created_at,
            attempt_type: row.attempt_type.unwrap_or_else(|| "Success".to_string()),
            attempt_correlation_id: row.attempt_correlation_id.unwrap_or_else(Uuid::new_v4),
            risk_score: row.risk_score.unwrap_or(0),
            failure_reason: row.failure_reason,
        };
        
        Ok(audit_log)
    }
}
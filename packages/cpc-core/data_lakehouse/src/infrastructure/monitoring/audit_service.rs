//! Extended audit logging service for the data lakehouse
//!
//! Building on the HIPAA implementation from the health module, we generalize audit trails for all data access.

use crate::domain::models::{
    AuditLog, AccessPurpose, DataAction, DataError, AuditError, DataAsset
};
use uuid::Uuid;
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::Row;

/// Repository trait for audit logs
#[async_trait::async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn save(&self, log: &AuditLog) -> Result<(), AuditError>;
    async fn get_by_id(&self, id: Uuid) -> Result<AuditLog, AuditError>;
    async fn get_by_asset(&self, asset_id: Uuid) -> Result<Vec<AuditLog>, AuditError>;
    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLog>, AuditError>;
}

/// Privacy configuration for audit logging
#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    pub research_anonymization: bool,
    pub sync_redaction: bool,
    pub analytics_aggregation: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            research_anonymization: true,
            sync_redaction: true,
            analytics_aggregation: true,
        }
    }
}

/// Extended audit service for data lakehouse
pub struct DataAuditService {
    repository: Arc<dyn AuditLogRepository>,
    privacy_config: PrivacyConfig,
}

impl DataAuditService {
    pub fn new(repository: Arc<dyn AuditLogRepository>, privacy_config: PrivacyConfig) -> Self {
        Self {
            repository,
            privacy_config,
        }
    }

    /// Log data access with appropriate anonymization based on purpose
    pub async fn log_access(
        &self,
        user_id: Option<Uuid>,
        asset_id: Uuid,
        purpose: AccessPurpose,
        action: DataAction,
    ) -> Result<(), DataError> {
        let timestamp = chrono::Utc::now();
        let mut log = AuditLog::new(
            user_id,
            asset_id,
            purpose.clone(),
            action,
            timestamp,
        );
        
        // Add client information
        log.source_ip = self.get_client_ip();
        log.device_info = self.get_device_info();
        
        // Apply anonymization based on purpose
        let anonymized = self.apply_anonymization(&log, &purpose);
        
        self.repository.save(&anonymized)
            .await
            .map_err(|e| DataError::AuditError(e.to_string()))?;
        
        Ok(())
    }

    /// Apply anonymization rules based on access purpose
    fn apply_anonymization(&self, log: &AuditLog, purpose: &AccessPurpose) -> AuditLog {
        match purpose {
            AccessPurpose::Research => {
                if self.privacy_config.research_anonymization {
                    AuditLog {
                        user_id: None,  // Anonymize for research
                        source_ip: self.redact_ip(&log.source_ip),
                        device_info: None,  // Omit for research
                        ..log.clone()
                    }
                } else {
                    log.clone()
                }
            },
            AccessPurpose::UserView | 
            AccessPurpose::ProviderAccess | 
            AccessPurpose::Admin => log.clone(),
            AccessPurpose::DataSync => {
                if self.privacy_config.sync_redaction {
                    AuditLog {
                        // Minimal redaction for sync operations
                        source_ip: self.partial_redact_ip(&log.source_ip),
                        ..log.clone()
                    }
                } else {
                    log.clone()
                }
            },
            AccessPurpose::Analytics | AccessPurpose::MachineLearning => {
                if self.privacy_config.analytics_aggregation {
                    AuditLog {
                        user_id: None,  // Anonymize for analytics
                        source_ip: self.redact_ip(&log.source_ip),
                        device_info: None,  // Omit for analytics
                        ..log.clone()
                    }
                } else {
                    log.clone()
                }
            },
        }
    }

    /// Get client IP address (placeholder implementation)
    fn get_client_ip(&self) -> Option<String> {
        // In a real implementation, this would extract the IP from the request context
        Some("192.168.1.1".to_string())
    }

    /// Get device information (placeholder implementation)
    fn get_device_info(&self) -> Option<String> {
        // In a real implementation, this would extract device info from the request context
        Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string())
    }

    /// Fully redact an IP address
    fn redact_ip(&self, ip: &Option<String>) -> Option<String> {
        ip.as_ref().map(|_| "***.***.***.***".to_string())
    }

    /// Partially redact an IP address (keep first octet)
    fn partial_redact_ip(&self, ip: &Option<String>) -> Option<String> {
        ip.as_ref().and_then(|ip_str| {
            ip_str.split('.').next()
                .map(|first_octet| format!("{}.***.***.***", first_octet))
        })
    }

    /// Check if access is permitted based on purpose and asset sensitivity
    pub async fn check_access_permission(
        &self,
        user_id: Option<Uuid>,
        asset: &DataAsset,
        purpose: &AccessPurpose,
    ) -> Result<bool, DataError> {
        // In a real implementation, this would check:
        // 1. User permissions for the asset
        // 2. Asset sensitivity levels
        // 3. Purpose appropriateness
        
        // For now, we'll allow all access
        Ok(true)
    }
}

/// PostgreSQL implementation of audit log repository
pub struct PostgresAuditLogRepository {
    connection: PgPool,
}

impl PostgresAuditLogRepository {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl AuditLogRepository for PostgresAuditLogRepository {
    async fn save(&self, log: &AuditLog) -> Result<(), AuditError> {
        sqlx::query(
            "INSERT INTO audit_logs (id, user_id, asset_id, purpose, action, timestamp, source_ip, device_info, data_content)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(log.id)
        .bind(log.user_id)
        .bind(log.asset_id)
        .bind(serde_json::to_string(&log.purpose).map_err(|e| AuditError::ValidationError(e.to_string()))?)
        .bind(serde_json::to_string(&log.action).map_err(|e| AuditError::ValidationError(e.to_string()))?)
        .bind(log.timestamp)
        .bind(&log.source_ip)
        .bind(&log.device_info)
        .bind(&log.data_content)
        .execute(&self.connection)
        .await?;
        
        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<AuditLog, AuditError> {
        let row = sqlx::query(
            "SELECT id, user_id, asset_id, purpose, action, timestamp, source_ip, device_info, data_content
             FROM audit_logs WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.connection)
        .await?;

        // Parse JSON fields back to enums
        let purpose_str: String = row.get("purpose");
        let purpose = serde_json::from_str(&purpose_str)
            .map_err(|e| AuditError::ValidationError(format!("Failed to parse purpose: {}", e)))?;
        
        let action_str: String = row.get("action");
        let action = serde_json::from_str(&action_str)
            .map_err(|e| AuditError::ValidationError(format!("Failed to parse action: {}", e)))?;
        
        Ok(AuditLog {
            id: row.get("id"),
            user_id: row.get("user_id"),
            asset_id: row.get("asset_id"),
            purpose,
            action,
            timestamp: row.get("timestamp"),
            source_ip: row.get("source_ip"),
            device_info: row.get("device_info"),
            data_content: row.get("data_content"),
        })
    }

    async fn get_by_asset(&self, asset_id: Uuid) -> Result<Vec<AuditLog>, AuditError> {
        let rows = sqlx::query(
            "SELECT id, user_id, asset_id, purpose, action, timestamp, source_ip, device_info, data_content
             FROM audit_logs WHERE asset_id = $1 ORDER BY timestamp DESC LIMIT 100"
        )
        .bind(asset_id)
        .fetch_all(&self.connection)
        .await?;

        // Parse JSON fields back to enums
        let logs = rows.into_iter().map(|row| {
            let purpose_str: String = row.get("purpose");
            let purpose = serde_json::from_str(&purpose_str)
                .map_err(|e| AuditError::ValidationError(format!("Failed to parse purpose: {}", e)))?;
            
            let action_str: String = row.get("action");
            let action = serde_json::from_str(&action_str)
                .map_err(|e| AuditError::ValidationError(format!("Failed to parse action: {}", e)))?;
            
            Ok(AuditLog {
                id: row.get("id"),
                user_id: row.get("user_id"),
                asset_id: row.get("asset_id"),
                purpose,
                action,
                timestamp: row.get("timestamp"),
                source_ip: row.get("source_ip"),
                device_info: row.get("device_info"),
                data_content: row.get("data_content"),
            })
        }).collect::<Result<Vec<AuditLog>, AuditError>>()?;

        Ok(logs)
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLog>, AuditError> {
        let rows = sqlx::query(
            "SELECT id, user_id, asset_id, purpose, action, timestamp, source_ip, device_info, data_content
             FROM audit_logs WHERE user_id = $1 ORDER BY timestamp DESC LIMIT 100"
        )
        .bind(user_id)
        .fetch_all(&self.connection)
        .await?;

        // Parse JSON fields back to enums
        let logs = rows.into_iter().map(|row| {
            let purpose_str: String = row.get("purpose");
            let purpose = serde_json::from_str(&purpose_str)
                .map_err(|e| AuditError::ValidationError(format!("Failed to parse purpose: {}", e)))?;
            
            let action_str: String = row.get("action");
            let action = serde_json::from_str(&action_str)
                .map_err(|e| AuditError::ValidationError(format!("Failed to parse action: {}", e)))?;
            
            Ok(AuditLog {
                id: row.get("id"),
                user_id: row.get("user_id"),
                asset_id: row.get("asset_id"),
                purpose,
                action,
                timestamp: row.get("timestamp"),
                source_ip: row.get("source_ip"),
                device_info: row.get("device_info"),
                data_content: row.get("data_content"),
            })
        }).collect::<Result<Vec<AuditLog>, AuditError>>()?;

        Ok(logs)
    }
}
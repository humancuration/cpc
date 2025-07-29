//! PostgreSQL database adapter for centralized storage.

use sqlx::PgPool;
use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::AuditEvent,
        errors::ConsentError,
    },
    application::service::ConsentStorage,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// PostgreSQL storage adapter
pub struct PostgresAdapter {
    pool: PgPool,
}

impl PostgresAdapter {
    /// Create a new PostgreSQL adapter
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConsentStorage for PostgresAdapter {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let domain_str = format!("{:?}", domain);
        
        let row = sqlx::query!(
            r#"SELECT level as "level: DataSharingLevel", created_at, updated_at 
               FROM consent_profiles 
               WHERE user_id = $1 AND domain = $2"#,
            user_id,
            domain_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        
        match row {
            Some(record) => {
                let profile = ConsentProfile {
                    user_id: user_id.to_string(),
                    domain: domain.clone(),
                    level: record.level,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                };
                Ok(Some(profile))
            },
            None => Ok(None),
        }
    }
    
    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
        let domain_str = format!("{:?}", profile.domain);
        let level_str = format!("{:?}", profile.level);
        
        sqlx::query!(
            r#"INSERT INTO consent_profiles (user_id, domain, level, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (user_id, domain) 
               DO UPDATE SET level = $3, updated_at = $5"#,
            profile.user_id,
            domain_str,
            level_str,
            profile.created_at,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
        let domain_str = format!("{:?}", domain);
        
        sqlx::query!(
            "DELETE FROM consent_profiles WHERE user_id = $1 AND domain = $2",
            user_id,
            domain_str
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        let rows = sqlx::query!(
            r#"SELECT id, domain, action, previous_level, new_level, actor_type, actor_id, timestamp
               FROM consent_audit_log 
               WHERE user_id = $1
               ORDER BY timestamp ASC"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        
        let mut events = Vec::new();
        for row in rows {
            // Reconstruct the domain enum
            let domain = match row.domain.as_str() {
                "FinancialData" => Domain::FinancialData,
                "HealthData" => Domain::HealthData,
                "CalendarData" => Domain::CalendarData,
                "CrmData" => Domain::CrmData,
                "ScmData" => Domain::ScmData,
                "DocumentData" => Domain::DocumentData,
                "WebsiteData" => Domain::WebsiteData,
                "RecruitmentData" => Domain::RecruitmentData,
                "DataLakehouse" => Domain::DataLakehouse,
                "ForecastingData" => Domain::ForecastingData,
                _ => return Err(ConsentError::StorageError("Invalid domain value".to_string())),
            };
            
            // Reconstruct the audit event
            let event = AuditEvent {
                id: row.id,
                user_id: user_id.to_string(),
                domain,
                action: // This would need to be reconstructed from row.action
                    match row.action.as_str() {
                        "Granted" => crate::domain::audit::ConsentAction::Granted,
                        "Revoked" => crate::domain::audit::ConsentAction::Revoked,
                        "Modified" => crate::domain::audit::ConsentAction::Modified,
                        _ => return Err(ConsentError::StorageError("Invalid action value".to_string())),
                    },
                previous_level: row.previous_level.map(|level| 
                    match level.as_str() {
                        "None" => DataSharingLevel::None,
                        "Minimal" => DataSharingLevel::Minimal,
                        "Standard" => DataSharingLevel::Standard,
                        "Full" => DataSharingLevel::Full,
                        _ => return Err(ConsentError::StorageError("Invalid previous_level value".to_string())),
                    }
                ),
                new_level: 
                    match row.new_level.as_str() {
                        "None" => DataSharingLevel::None,
                        "Minimal" => DataSharingLevel::Minimal,
                        "Standard" => DataSharingLevel::Standard,
                        "Full" => DataSharingLevel::Full,
                        _ => return Err(ConsentError::StorageError("Invalid new_level value".to_string())),
                    },
                actor: // This would need to be reconstructed from actor_type and actor_id
                    match row.actor_type.as_str() {
                        "User" => crate::domain::audit::Actor::User(row.actor_id),
                        "Service" => crate::domain::audit::Actor::Service(row.actor_id),
                        "Admin" => crate::domain::audit::Actor::Admin(row.actor_id),
                        _ => return Err(ConsentError::StorageError("Invalid actor_type value".to_string())),
                    },
                timestamp: row.timestamp,
            };
            
            events.push(event);
        }
        
        Ok(events)
    }
    
    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
        let domain_str = format!("{:?}", event.domain);
        let action_str = format!("{:?}", event.action);
        let previous_level_str = event.previous_level.as_ref().map(|level| format!("{:?}", level));
        let new_level_str = format!("{:?}", event.new_level);
        
        let (actor_type, actor_id) = match &event.actor {
            crate::domain::audit::Actor::User(id) => ("User".to_string(), id.clone()),
            crate::domain::audit::Actor::Service(name) => ("Service".to_string(), name.clone()),
            crate::domain::audit::Actor::Admin(id) => ("Admin".to_string(), id.clone()),
        };
        
        sqlx::query!(
            r#"INSERT INTO consent_audit_log 
               (id, user_id, domain, action, previous_level, new_level, actor_type, actor_id, timestamp)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            event.id,
            event.user_id,
            domain_str,
            action_str,
            previous_level_str,
            new_level_str,
            actor_type,
            actor_id,
            event.timestamp
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        
        Ok(())
    }
}
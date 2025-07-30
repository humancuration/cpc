//! Database repositories for the health module
//!
//! This module contains implementations of repositories for health data,
//! including audit logging for HIPAA compliance.

use crate::domain::health_condition::{HealthCondition, HealthConditionType, Severity, Status};
use crate::domain::vital_signs::{VitalSign, MeasurementType, DataSource};
use crate::domain::audit_log::{AuditLog, AuditPurpose, AccessType};
use crate::infrastructure::database::audit_log_repository::AuditLogRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;
use tracing::error;

/// Error types for health data operations
#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid measurement type: {0}")]
    InvalidMeasurementType(String),
    #[error("Invalid data source: {0}")]
    InvalidDataSource(String),
    #[error("Invalid condition type: {0}")]
    InvalidConditionType(String),
    #[error("Invalid severity: {0}")]
    InvalidSeverity(String),
    #[error("Invalid status: {0}")]
    InvalidStatus(String),
    #[error("Health record not found")]
    NotFound,
    #[error("Audit log error: {0}")]
    AuditLogError(String),
}

/// Repository trait for vital sign operations
#[async_trait]
pub trait VitalSignRepository: Send + Sync {
    async fn save(&self, vital_sign: &VitalSign, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError>;
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<VitalSign, HealthError>;
    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<VitalSign>, HealthError>;
}

/// Repository trait for health condition operations
#[async_trait]
pub trait HealthConditionRepository: Send + Sync {
    async fn save(&self, condition: &HealthCondition, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError>;
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<HealthCondition, HealthError>;
    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<HealthCondition>, HealthError>;
}

/// Implementation of VitalSignRepository for PostgreSQL
pub struct VitalSignRepositoryImpl {
    pool: PgPool,
    audit_log_repository: Box<dyn AuditLogRepository>,
}

impl VitalSignRepositoryImpl {
    pub fn new(pool: PgPool, audit_log_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { pool, audit_log_repository }
    }
    
    /// Log an audit entry for vital sign access
    async fn log_audit(&self, user_id: Option<Uuid>, data_id: Uuid, access_type: AccessType, purpose: AuditPurpose) {
        let audit_log = AuditLog::new(
            user_id,
            "VitalSign",
            data_id,
            access_type.as_str(),
            purpose.as_str(),
            None, // source_ip would be provided by request context
            None, // device_info would be provided by request context
        );
        
        // Attempt to log the audit entry, but don't fail the operation if it fails
        if let Err(e) = self.audit_log_repository.create(audit_log).await {
            error!("Failed to create audit log: {}", e);
        }
    }
}

#[async_trait]
impl VitalSignRepository for VitalSignRepositoryImpl {
    async fn save(&self, vital_sign: &VitalSign, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError> {
        // Log audit for write operation
        self.log_audit(
            Some(vital_sign.user_id),
            vital_sign.id,
            AccessType::Write,
            AuditPurpose::UserView,
        ).await;
        
        let measurement_type_str = match &vital_sign.measurement_type {
            MeasurementType::BloodPressure => "BloodPressure",
            MeasurementType::HeartRate => "HeartRate",
            MeasurementType::Temperature => "Temperature",
            MeasurementType::Weight => "Weight",
            MeasurementType::Height => "Height",
            MeasurementType::BloodSugar => "BloodSugar",
            MeasurementType::Cholesterol => "Cholesterol",
            MeasurementType::Other(_) => "Other",
        };

        let (source_type_str, source_details_str) = match &vital_sign.source {
            DataSource::Manual => ("Manual", None),
            DataSource::Device(device_name) => ("Device", Some(device_name.as_str())),
            DataSource::Api(api_name) => ("Api", Some(api_name.as_str())),
        };

        sqlx::query!(
            r#"
            INSERT INTO vital_signs (
                id, user_id, measurement_type, value, unit, 
                recorded_at, source_type, source_details, notes, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                measurement_type = EXCLUDED.measurement_type,
                value = EXCLUDED.value,
                unit = EXCLUDED.unit,
                recorded_at = EXCLUDED.recorded_at,
                source_type = EXCLUDED.source_type,
                source_details = EXCLUDED.source_details,
                notes = EXCLUDED.notes,
                updated_at = NOW()
            "#,
            vital_sign.id,
            vital_sign.user_id,
            measurement_type_str,
            vital_sign.value,
            vital_sign.unit,
            vital_sign.recorded_at,
            source_type_str,
            source_details_str,
            vital_sign.notes,
            vital_sign.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<VitalSign, HealthError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, measurement_type, value, unit, 
                   recorded_at, source_type, source_details, notes, created_at
            FROM vital_signs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        let row = match row {
            Some(row) => row,
            None => return Err(HealthError::NotFound),
        };
        
        // Log audit for read operation
        self.log_audit(
            Some(row.user_id),
            row.id,
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;

        let measurement_type = match row.measurement_type.as_str() {
            "BloodPressure" => MeasurementType::BloodPressure,
            "HeartRate" => MeasurementType::HeartRate,
            "Temperature" => MeasurementType::Temperature,
            "Weight" => MeasurementType::Weight,
            "Height" => MeasurementType::Height,
            "BloodSugar" => MeasurementType::BloodSugar,
            "Cholesterol" => MeasurementType::Cholesterol,
            _ => MeasurementType::Other(row.measurement_type),
        };

        let source = match row.source_type.as_str() {
            "Manual" => DataSource::Manual,
            "Device" => DataSource::Device(row.source_details.unwrap_or_default()),
            "Api" => DataSource::Api(row.source_details.unwrap_or_default()),
            _ => DataSource::Manual,
        };

        Ok(VitalSign {
            id: row.id,
            user_id: row.user_id,
            measurement_type,
            value: row.value,
            unit: row.unit,
            recorded_at: row.recorded_at,
            source,
            notes: row.notes,
            created_at: row.created_at,
        })
    }

    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<VitalSign>, HealthError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, measurement_type, value, unit, 
                   recorded_at, source_type, source_details, notes, created_at
            FROM vital_signs
            WHERE user_id = $1
            ORDER BY recorded_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Log audit for bulk read operation
        // Note: We're logging one audit entry for the entire query rather than per record
        // to avoid excessive logging
        self.log_audit(
            Some(user_id),
            Uuid::nil(), // Using nil UUID as placeholder for bulk operation
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;

        let vital_signs = rows
            .into_iter()
            .map(|row| {
                let measurement_type = match row.measurement_type.as_str() {
                    "BloodPressure" => MeasurementType::BloodPressure,
                    "HeartRate" => MeasurementType::HeartRate,
                    "Temperature" => MeasurementType::Temperature,
                    "Weight" => MeasurementType::Weight,
                    "Height" => MeasurementType::Height,
                    "BloodSugar" => MeasurementType::BloodSugar,
                    "Cholesterol" => MeasurementType::Cholesterol,
                    _ => MeasurementType::Other(row.measurement_type),
                };

                let source = match row.source_type.as_str() {
                    "Manual" => DataSource::Manual,
                    "Device" => DataSource::Device(row.source_details.unwrap_or_default()),
                    "Api" => DataSource::Api(row.source_details.unwrap_or_default()),
                    _ => DataSource::Manual,
                };

                VitalSign {
                    id: row.id,
                    user_id: row.user_id,
                    measurement_type,
                    value: row.value,
                    unit: row.unit,
                    recorded_at: row.recorded_at,
                    source,
                    notes: row.notes,
                    created_at: row.created_at,
                }
            })
            .collect();

        Ok(vital_signs)
    }
}

/// Implementation of HealthConditionRepository for PostgreSQL
pub struct HealthConditionRepositoryImpl {
    pool: PgPool,
    audit_log_repository: Box<dyn AuditLogRepository>,
}

impl HealthConditionRepositoryImpl {
    pub fn new(pool: PgPool, audit_log_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { pool, audit_log_repository }
    }
    
    /// Log an audit entry for health condition access
    async fn log_audit(&self, user_id: Option<Uuid>, data_id: Uuid, access_type: AccessType, purpose: AuditPurpose) {
        let audit_log = AuditLog::new(
            user_id,
            "HealthCondition",
            data_id,
            access_type.as_str(),
            purpose.as_str(),
            None, // source_ip would be provided by request context
            None, // device_info would be provided by request context
        );
        
        // Attempt to log the audit entry, but don't fail the operation if it fails
        if let Err(e) = self.audit_log_repository.create(audit_log).await {
            error!("Failed to create audit log: {}", e);
        }
    }
}

#[async_trait]
impl HealthConditionRepository for HealthConditionRepositoryImpl {
    async fn save(&self, condition: &HealthCondition, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError> {
        // Log audit for write operation
        self.log_audit(
            Some(condition.user_id),
            condition.id,
            AccessType::Write,
            AuditPurpose::UserView,
        ).await;
        
        let condition_category_str = match &condition.condition_type {
            HealthConditionType::Chronic(_) => "Chronic",
            HealthConditionType::Acute(_) => "Acute",
            HealthConditionType::Injury(_) => "Injury",
            HealthConditionType::Allergy(_) => "Allergy",
            HealthConditionType::Other(_) => "Other",
        };

        let condition_description_str = match &condition.condition_type {
            HealthConditionType::Chronic(desc) => desc.as_str(),
            HealthConditionType::Acute(desc) => desc.as_str(),
            HealthConditionType::Injury(desc) => desc.as_str(),
            HealthConditionType::Allergy(desc) => desc.as_str(),
            HealthConditionType::Other(desc) => desc.as_str(),
        };

        let severity_str = match &condition.severity {
            Severity::Mild => "Mild",
            Severity::Moderate => "Moderate",
            Severity::Severe => "Severe",
            Severity::Critical => "Critical",
            Severity::Unknown => "Unknown",
        };

        let status_str = match &condition.status {
            Status::Active => "Active",
            Status::Inactive => "Inactive",
            Status::Resolved => "Resolved",
            Status::Recurring => "Recurring",
        };

        sqlx::query!(
            r#"
            INSERT INTO health_conditions (
                id, user_id, condition_category, condition_description, 
                severity, status, diagnosed_at, resolved_at, notes, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                condition_category = EXCLUDED.condition_category,
                condition_description = EXCLUDED.condition_description,
                severity = EXCLUDED.severity,
                status = EXCLUDED.status,
                diagnosed_at = EXCLUDED.diagnosed_at,
                resolved_at = EXCLUDED.resolved_at,
                notes = EXCLUDED.notes,
                updated_at = NOW()
            "#,
            condition.id,
            condition.user_id,
            condition_category_str,
            condition_description_str,
            severity_str,
            status_str,
            condition.diagnosed_at,
            condition.resolved_at,
            condition.notes,
            condition.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<HealthCondition, HealthError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, condition_category, condition_description, 
                   severity, status, diagnosed_at, resolved_at, notes, created_at
            FROM health_conditions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        let row = match row {
            Some(row) => row,
            None => return Err(HealthError::NotFound),
        };
        
        // Log audit for read operation
        self.log_audit(
            Some(row.user_id),
            row.id,
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;

        let condition_type = match row.condition_category.as_str() {
            "Chronic" => HealthConditionType::Chronic(row.condition_description),
            "Acute" => HealthConditionType::Acute(row.condition_description),
            "Injury" => HealthConditionType::Injury(row.condition_description),
            "Allergy" => HealthConditionType::Allergy(row.condition_description),
            _ => HealthConditionType::Other(row.condition_description),
        };

        let severity = match row.severity.as_str() {
            "Mild" => Severity::Mild,
            "Moderate" => Severity::Moderate,
            "Severe" => Severity::Severe,
            "Critical" => Severity::Critical,
            _ => Severity::Unknown,
        };

        let status = match row.status.as_str() {
            "Active" => Status::Active,
            "Inactive" => Status::Inactive,
            "Resolved" => Status::Resolved,
            "Recurring" => Status::Recurring,
            _ => Status::Active,
        };

        Ok(HealthCondition {
            id: row.id,
            user_id: row.user_id,
            condition_type,
            severity,
            status,
            diagnosed_at: row.diagnosed_at,
            resolved_at: row.resolved_at,
            notes: row.notes,
            created_at: row.created_at,
        })
    }

    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<HealthCondition>, HealthError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, condition_category, condition_description, 
                   severity, status, diagnosed_at, resolved_at, notes, created_at
            FROM health_conditions
            WHERE user_id = $1
            ORDER BY diagnosed_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Log audit for bulk read operation
        // Note: We're logging one audit entry for the entire query rather than per record
        // to avoid excessive logging
        self.log_audit(
            Some(user_id),
            Uuid::nil(), // Using nil UUID as placeholder for bulk operation
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;

        let conditions = rows
            .into_iter()
            .map(|row| {
                let condition_type = match row.condition_category.as_str() {
                    "Chronic" => HealthConditionType::Chronic(row.condition_description),
                    "Acute" => HealthConditionType::Acute(row.condition_description),
                    "Injury" => HealthConditionType::Injury(row.condition_description),
                    "Allergy" => HealthConditionType::Allergy(row.condition_description),
                    _ => HealthConditionType::Other(row.condition_description),
                };

                let severity = match row.severity.as_str() {
                    "Mild" => Severity::Mild,
                    "Moderate" => Severity::Moderate,
                    "Severe" => Severity::Severe,
                    "Critical" => Severity::Critical,
                    _ => Severity::Unknown,
                };

                let status = match row.status.as_str() {
                    "Active" => Status::Active,
                    "Inactive" => Status::Inactive,
                    "Resolved" => Status::Resolved,
                    "Recurring" => Status::Recurring,
                    _ => Status::Active,
                };

                HealthCondition {
                    id: row.id,
                    user_id: row.user_id,
                    condition_type,
                    severity,
                    status,
                    diagnosed_at: row.diagnosed_at,
                    resolved_at: row.resolved_at,
                    notes: row.notes,
                    created_at: row.created_at,
                }
            })
            .collect();

        Ok(conditions)
    }
}
//! Health condition tracking service
//!
//! This service handles health condition management.

use crate::domain::health_condition::{HealthCondition, ConditionType, ConditionSeverity, ConditionStatus};
use crate::application::HealthError;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;

/// Service for tracking health conditions
pub struct ConditionTrackingService {
    repository: Arc<dyn HealthConditionRepository>,
}

impl ConditionTrackingService {
    /// Create a new condition tracking service
    pub fn new(repository: Arc<dyn HealthConditionRepository>) -> Self {
        Self {
            repository,
        }
    }

    /// Create a new health condition
    pub async fn create_condition(
        &self,
        user_id: Uuid,
        condition_type: ConditionType,
        diagnosis_date: DateTime<Utc>,
        severity: ConditionSeverity,
        status: ConditionStatus,
        notes: Option<String>,
    ) -> Result<HealthCondition, HealthError> {
        let condition = HealthCondition::new(
            user_id,
            condition_type,
            diagnosis_date,
            severity,
            status,
            notes,
            Utc::now(),
            Utc::now(),
        );
        self.repository.save(&condition).await?;
        Ok(condition)
    }

    /// Update an existing health condition
    pub async fn update_condition(
        &self,
        id: Uuid,
        severity: ConditionSeverity,
        status: ConditionStatus,
        notes: Option<String>,
    ) -> Result<HealthCondition, HealthError> {
        let mut condition = self.repository.find_by_id(id).await?;
        condition.severity = severity;
        condition.status = status;
        condition.notes = notes;
        condition.updated_at = Utc::now();
        self.repository.save(&condition).await?;
        Ok(condition)
    }

    /// Get a specific health condition by ID
    pub async fn get_condition(&self, id: Uuid) -> Result<HealthCondition, HealthError> {
        self.repository.find_by_id(id).await
    }

    /// List health conditions with optional filtering
    pub async fn list_conditions(
        &self,
        user_id: Uuid,
        filter: Option<ConditionFilter>,
    ) -> Result<Vec<HealthCondition>, HealthError> {
        self.repository.find_by_user(user_id, filter).await
    }
}

/// Repository trait for health condition persistence
#[async_trait::async_trait]
pub trait HealthConditionRepository: Send + Sync {
    async fn save(&self, condition: &HealthCondition) -> Result<(), HealthError>;
    async fn find_by_id(&self, id: Uuid) -> Result<HealthCondition, HealthError>;
    async fn find_by_user(
        &self,
        user_id: Uuid,
        filter: Option<ConditionFilter>,
    ) -> Result<Vec<HealthCondition>, HealthError>;
}

/// Filter for querying health conditions
pub struct ConditionFilter {
    pub status: Option<ConditionStatus>,
    pub condition_type: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
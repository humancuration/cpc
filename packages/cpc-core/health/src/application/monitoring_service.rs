//! Health monitoring service
//!
//! This service handles vital sign recording and monitoring.

use crate::domain::vital_signs::{VitalSign, VitalSignType, MeasurementSource, AnonymizedVitalSign};
use crate::application::HealthError;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;

/// Service for monitoring health metrics
pub struct HealthMonitoringService {
    repository: Arc<dyn VitalSignRepository>,
    p2p_manager: Arc<dyn P2PManager>,
}

impl HealthMonitoringService {
    /// Create a new health monitoring service
    pub fn new(
        repository: Arc<dyn VitalSignRepository>,
        p2p_manager: Arc<dyn P2PManager>,
    ) -> Self {
        Self {
            repository,
            p2p_manager,
        }
    }

    /// Record a new vital sign measurement
    pub async fn record_vital_sign(
        &self,
        user_id: Uuid,
        measurement_type: VitalSignType,
        value: f32,
        unit: String,
        source: MeasurementSource,
        notes: Option<String>,
    ) -> Result<VitalSign, HealthError> {
        let vital_sign = VitalSign::new(
            user_id, 
            measurement_type, 
            value, 
            unit, 
            source, 
            notes,
            Utc::now()
        );
        
        // Anonymize data for research sharing if user has consented
        if let Some(anonymized) = vital_sign.anonymize_for_research() {
            self.p2p_manager.share_health_data(anonymized).await?;
        }
        
        self.repository.save(&vital_sign).await?;
        Ok(vital_sign)
    }

    /// Get a specific vital sign by ID
    pub async fn get_vital_sign(&self, id: Uuid) -> Result<VitalSign, HealthError> {
        self.repository.find_by_id(id).await
    }

    /// List vital signs with optional filtering
    pub async fn list_vital_signs(
        &self,
        user_id: Uuid,
        filter: Option<VitalSignFilter>,
    ) -> Result<Vec<VitalSign>, HealthError> {
        self.repository.find_by_user(user_id, filter).await
    }
}

/// Repository trait for vital sign persistence
#[async_trait::async_trait]
pub trait VitalSignRepository: Send + Sync {
    async fn save(&self, vital_sign: &VitalSign) -> Result<(), HealthError>;
    async fn find_by_id(&self, id: Uuid) -> Result<VitalSign, HealthError>;
    async fn find_by_user(
        &self,
        user_id: Uuid,
        filter: Option<VitalSignFilter>,
    ) -> Result<Vec<VitalSign>, HealthError>;
}

/// Filter for querying vital signs
pub struct VitalSignFilter {
    pub measurement_type: Option<VitalSignType>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// P2P manager trait for health data sharing
#[async_trait::async_trait]
pub trait P2PManager: Send + Sync {
    async fn share_health_data(&self, data: AnonymizedVitalSign) -> Result<(), HealthError>;
}

/// Health trend result
pub struct HealthTrendResult {
    pub measurement_type: String,
    pub average: f32,
    pub min: f32,
    pub max: f32,
    pub trend: f32, // Change over time
}
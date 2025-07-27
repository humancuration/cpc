//! Database models for the health module
//!
//! These structs map directly to database tables.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for vital signs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSignModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub measurement_type: String,
    pub value: f32,
    pub unit: String,
    pub source_type: String,
    pub source_details: Option<String>,
    pub notes: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for health conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConditionModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub condition_category: String,
    pub condition_description: String,
    pub diagnosis_date: DateTime<Utc>,
    pub severity: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for health data sharing preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthDataSharingPreferenceModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub health_data_sharing_enabled: bool,
    pub research_sharing_level: String,
    pub emergency_access_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for health alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAlertModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub related_data: Option<serde_json::Value>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
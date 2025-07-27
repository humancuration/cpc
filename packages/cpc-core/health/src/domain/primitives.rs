//! Primitive types for the health module
//!
//! This module contains basic types and structures used throughout the health module.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a health alert
#[derive(Debug, Clone)]
pub struct HealthAlert {
    pub id: Uuid,
    pub user_id: Uuid,
    pub alert_type: HealthAlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub related_data: Option<serde_json::Value>,
}

/// Types of health alerts
#[derive(Debug, Clone)]
pub enum HealthAlertType {
    AbnormalReading,
    MedicationReminder,
    AppointmentReminder,
    TreatmentMilestone,
}

/// Severity levels for alerts
#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Represents health trends over time
#[derive(Debug, Clone)]
pub struct HealthTrend {
    pub measurement_type: String,
    pub values: Vec<(DateTime<Utc>, f32)>,
    pub trend_direction: TrendDirection,
}

/// Direction of a health trend
#[derive(Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Deteriorating,
    Stable,
    InsufficientData,
}
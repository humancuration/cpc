//! Health condition domain model
//!
//! This module contains the core entities and logic for managing health conditions.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a diagnosed health condition
#[derive(Debug, Clone)]
pub struct HealthCondition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub condition_type: ConditionType,
    pub diagnosis_date: DateTime<Utc>,
    pub severity: ConditionSeverity,
    pub status: ConditionStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl HealthCondition {
    /// Create a new health condition
    pub fn new(
        user_id: Uuid,
        condition_type: ConditionType,
        diagnosis_date: DateTime<Utc>,
        severity: ConditionSeverity,
        status: ConditionStatus,
        notes: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            condition_type,
            diagnosis_date,
            severity,
            status,
            notes,
            created_at,
            updated_at,
        }
    }
}

/// Types of health conditions
#[derive(Debug, Clone)]
pub enum ConditionType {
    Chronic(String),  // e.g., "Diabetes Type 2"
    Acute(String),    // e.g., "Influenza"
    Genetic(String),
    MentalHealth(String),
}

/// Severity levels for health conditions
#[derive(Debug, Clone)]
pub enum ConditionSeverity {
    Mild,
    Moderate,
    Severe,
    Critical,
}

/// Status of a health condition
#[derive(Debug, Clone)]
pub enum ConditionStatus {
    Active,
    Remission,
    Resolved,
    Chronic,
}
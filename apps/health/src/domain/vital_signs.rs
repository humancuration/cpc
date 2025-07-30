//! Vital signs domain model
//!
//! This module contains the core entities and logic for managing vital signs.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a vital sign measurement
#[derive(Debug, Clone)]
pub struct VitalSign {
    pub id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub measurement_type: VitalSignType,
    pub value: f32,
    pub unit: String,
    pub source: MeasurementSource,
    pub notes: Option<String>,
}

impl VitalSign {
    /// Create a new vital sign measurement
    pub fn new(
        user_id: Uuid,
        measurement_type: VitalSignType,
        value: f32,
        unit: String,
        source: MeasurementSource,
        notes: Option<String>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            timestamp,
            measurement_type,
            value,
            unit,
            source,
            notes,
        }
    }

    /// Anonymize data for research sharing
    pub fn anonymize_for_research(&self) -> Option<AnonymizedVitalSign> {
        // Implementation would remove personally identifiable information
        // while preserving useful statistical data
        Some(AnonymizedVitalSign {
            timestamp: self.timestamp,
            measurement_type: self.measurement_type.clone(),
            value: self.value,
            unit: self.unit.clone(),
            source_type: self.source.source_type(),
        })
    }
}

/// Types of vital sign measurements
#[derive(Debug, Clone)]
pub enum VitalSignType {
    HeartRate,
    BloodPressure,
    BloodGlucose,
    BodyTemperature,
    OxygenSaturation,
    RespiratoryRate,
    BodyWeight,
    BodyMassIndex,
}

/// Source of a measurement
#[derive(Debug, Clone)]
pub enum MeasurementSource {
    Wearable(String),  // Device model
    Manual,
    MedicalDevice(String),
}

impl MeasurementSource {
    /// Get the source type for anonymization
    pub fn source_type(&self) -> String {
        match self {
            MeasurementSource::Wearable(_) => "wearable".to_string(),
            MeasurementSource::Manual => "manual".to_string(),
            MeasurementSource::MedicalDevice(_) => "medical_device".to_string(),
        }
    }
}

/// Anonymized vital sign for research sharing
#[derive(Debug, Clone)]
pub struct AnonymizedVitalSign {
    pub timestamp: DateTime<Utc>,
    pub measurement_type: VitalSignType,
    pub value: f32,
    pub unit: String,
    pub source_type: String,
}
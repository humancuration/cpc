//! Warehouse entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::primitives::{GeoLocation, OperatingHours, DomainError, Result};
use super::consent::WarehouseConsentSettings;

/// Warehouse entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: GeoLocation,
    pub capacity: i32,
    pub current_utilization: i32,
    pub operating_hours: OperatingHours,
    pub contact_info: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: WarehouseConsentSettings,
}

impl Warehouse {
    /// Create a new warehouse
    pub fn new(
        name: String,
        location: GeoLocation,
        capacity: i32,
        operating_hours: OperatingHours,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            location,
            capacity,
            current_utilization: 0,
            operating_hours,
            contact_info: None,
            created_at: now,
            updated_at: now,
            consent_settings: WarehouseConsentSettings::default(),
        }
    }

    /// Set description for the warehouse
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set contact information for the warehouse
    pub fn with_contact_info(mut self, contact_info: String) -> Self {
        self.contact_info = Some(contact_info);
        self
    }

    /// Set consent settings for the warehouse
    pub fn set_consent_settings(&mut self, settings: WarehouseConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Calculate available capacity
    pub fn calculate_available_capacity(&self) -> i32 {
        self.capacity - self.current_utilization
    }

    /// Validate capacity for additional items
    pub fn validate_capacity(&self, additional_items: i32) -> Result<()> {
        let new_utilization = self.current_utilization + additional_items;
        
        if new_utilization > self.capacity {
            Err(DomainError::CapacityExceeded)
        } else {
            Ok(())
        }
    }

    /// Update utilization
    pub fn update_utilization(&mut self, delta: i32) -> Result<()> {
        let new_utilization = self.current_utilization + delta;
        
        if new_utilization > self.capacity {
            return Err(DomainError::CapacityExceeded);
        }
        
        if new_utilization < 0 {
            return Err(DomainError::ValidationError {
                message: "Utilization cannot be negative".to_string(),
            });
        }
        
        self.current_utilization = new_utilization;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if the warehouse is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.current_utilization >= self.capacity
    }

    /// Check if the warehouse is over capacity
    pub fn is_over_capacity(&self) -> bool {
        self.current_utilization > self.capacity
    }

    /// Calculate utilization percentage
    pub fn utilization_percentage(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            (self.current_utilization as f64 / self.capacity as f64) * 100.0
        }
    }

    /// Validate the warehouse
    pub fn validate(&self) -> Result<()> {
        // Utilization cannot exceed capacity
        if self.current_utilization > self.capacity {
            return Err(DomainError::ValidationError {
                message: "Current utilization cannot exceed capacity".to_string(),
            });
        }
        
        // Capacity must be a positive value
        if self.capacity <= 0 {
            return Err(DomainError::ValidationError {
                message: "Capacity must be a positive value".to_string(),
            });
        }
        
        // Utilization cannot be negative
        if self.current_utilization < 0 {
            return Err(DomainError::ValidationError {
                message: "Utilization cannot be negative".to_string(),
            });
        }
        
        // Location must have valid geocoordinates
        if self.location.latitude < -90.0 || self.location.latitude > 90.0 {
            return Err(DomainError::ValidationError {
                message: "Latitude must be between -90 and 90 degrees".to_string(),
            });
        }
        
        if self.location.longitude < -180.0 || self.location.longitude > 180.0 {
            return Err(DomainError::ValidationError {
                message: "Longitude must be between -180 and 180 degrees".to_string(),
            });
        }
        
        Ok(())
    }

    /// Check if capacity data can be shared
    pub fn can_share_capacity_data(&self) -> bool {
        matches!(
            self.consent_settings.share_capacity_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if utilization data can be shared
    pub fn can_share_utilization_data(&self) -> bool {
        matches!(
            self.consent_settings.share_utilization_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if location data can be shared
    pub fn can_share_location_data(&self) -> bool {
        matches!(
            self.consent_settings.share_location_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if operating hours can be shared
    pub fn can_share_operating_hours(&self) -> bool {
        matches!(
            self.consent_settings.share_operating_hours,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }
}
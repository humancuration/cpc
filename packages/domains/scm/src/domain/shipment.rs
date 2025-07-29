//! Shipment entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

use super::primitives::{NodeId, DomainError, Result};
use super::consent::ShipmentConsentSettings;

/// Status of a shipment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShipmentStatus {
    Created,
    InTransit,
    Delayed,
    Delivered,
    Cancelled,
}

/// Line item in a shipment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipmentLineItem {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub quantity: i32,
    pub description: Option<String>,
}

impl ShipmentLineItem {
    pub fn new(
        inventory_item_id: Uuid,
        quantity: i32,
        description: Option<String>,
    ) -> Result<Self> {
        // Validate quantity
        if quantity <= 0 {
            return Err(DomainError::ValidationError {
                message: "Quantity must be positive".to_string(),
            });
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            inventory_item_id,
            quantity,
            description,
        })
    }
}

/// Shipment entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shipment {
    pub id: Uuid,
    pub tracking_number: String,
    pub carrier: String,
    pub status: ShipmentStatus,
    pub origin_id: NodeId,
    pub destination_id: NodeId,
    pub expected_transit_days: i32,
    pub actual_transit_days: Option<i32>,
    pub estimated_delivery: DateTime<Utc>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub line_items: Vec<ShipmentLineItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: ShipmentConsentSettings,
}

impl Shipment {
    /// Create a new shipment
    pub fn new(
        tracking_number: String,
        carrier: String,
        origin_id: NodeId,
        destination_id: NodeId,
        expected_transit_days: i32,
    ) -> Self {
        let now = Utc::now();
        let estimated_delivery = now + Duration::days(expected_transit_days as i64);
        
        Self {
            id: Uuid::new_v4(),
            tracking_number,
            carrier,
            status: ShipmentStatus::Created,
            origin_id,
            destination_id,
            expected_transit_days,
            actual_transit_days: None,
            estimated_delivery,
            actual_delivery: None,
            line_items: Vec::new(),
            created_at: now,
            updated_at: now,
            consent_settings: ShipmentConsentSettings::default(),
        }
    }

    /// Add a line item to the shipment
    pub fn add_line_item(&mut self, line_item: ShipmentLineItem) -> Result<()> {
        // Validate that we're not adding line items to a delivered or cancelled shipment
        match self.status {
            ShipmentStatus::Delivered | ShipmentStatus::Cancelled => {
                return Err(DomainError::InvalidOperation {
                    message: "Cannot add line items to delivered or cancelled shipments".to_string(),
                });
            }
            _ => {}
        }
        
        self.line_items.push(line_item);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a line item from the shipment
    pub fn remove_line_item(&mut self, line_item_id: Uuid) -> Result<()> {
        // Validate that we're not removing line items from a delivered or cancelled shipment
        match self.status {
            ShipmentStatus::Delivered | ShipmentStatus::Cancelled => {
                return Err(DomainError::InvalidOperation {
                    message: "Cannot remove line items from delivered or cancelled shipments".to_string(),
                });
            }
            _ => {}
        }
        
        let initial_count = self.line_items.len();
        self.line_items.retain(|item| item.id != line_item_id);
        
        if self.line_items.len() == initial_count {
            return Err(DomainError::NotFound);
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update the status of the shipment
    pub fn update_status(&mut self, new_status: ShipmentStatus) -> Result<()> {
        // Validate status transition rules
        let valid_transition = match (&self.status, &new_status) {
            (ShipmentStatus::Created, ShipmentStatus::InTransit) => true,
            (ShipmentStatus::Created, ShipmentStatus::Cancelled) => true,
            (ShipmentStatus::InTransit, ShipmentStatus::Delivered) => true,
            (ShipmentStatus::InTransit, ShipmentStatus::Delayed) => true,
            (ShipmentStatus::Delayed, ShipmentStatus::InTransit) => true,
            (ShipmentStatus::Delayed, ShipmentStatus::Delivered) => true,
            _ => false,
        };
        
        if !valid_transition {
            return Err(DomainError::InvalidStatusTransition);
        }
        
        // Update timestamps accordingly
        match new_status {
            ShipmentStatus::InTransit => {
                // If we're transitioning to InTransit, we might want to update the estimated delivery
                // based on current time and expected transit days
                self.estimated_delivery = Utc::now() + Duration::days(self.expected_transit_days as i64);
            }
            ShipmentStatus::Delivered => {
                let now = Utc::now();
                self.actual_delivery = Some(now);
                
                // Calculate actual transit days
                let duration = now - self.created_at;
                self.actual_transit_days = Some(duration.num_days() as i32);
            }
            _ => {}
        }
        
        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark shipment as delayed
    pub fn mark_delayed(&mut self) -> Result<()> {
        self.update_status(ShipmentStatus::Delayed)
    }

    /// Mark shipment as in transit
    pub fn mark_in_transit(&mut self) -> Result<()> {
        self.update_status(ShipmentStatus::InTransit)
    }

    /// Mark shipment as delivered
    pub fn mark_delivered(&mut self) -> Result<()> {
        self.update_status(ShipmentStatus::Delivered)
    }

    /// Cancel the shipment
    pub fn cancel(&mut self) -> Result<()> {
        self.update_status(ShipmentStatus::Cancelled)
    }

    /// Set consent settings for the shipment
    pub fn set_consent_settings(&mut self, settings: ShipmentConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Validate the shipment
    pub fn validate(&self) -> Result<()> {
        // Cannot change origin/destination after creation (this is checked at the application level)
        // For domain validation, we just ensure they're not the same
        if self.origin_id == self.destination_id {
            return Err(DomainError::ValidationError {
                message: "Origin and destination cannot be the same".to_string(),
            });
        }
        
        // Tracking numbers must be unique (uniqueness is enforced at the repository level)
        if self.tracking_number.is_empty() {
            return Err(DomainError::ValidationError {
                message: "Tracking number cannot be empty".to_string(),
            });
        }
        
        // Expected transit days must be positive
        if self.expected_transit_days <= 0 {
            return Err(DomainError::ValidationError {
                message: "Expected transit days must be positive".to_string(),
            });
        }
        
        // If actual delivery is set, it must be after created_at
        if let Some(actual_delivery) = self.actual_delivery {
            if actual_delivery <= self.created_at {
                return Err(DomainError::ValidationError {
                    message: "Actual delivery date must be after shipment creation".to_string(),
                });
            }
        }
        
        // Validate line items
        for item in &self.line_items {
            // Quantity must be positive
            if item.quantity <= 0 {
                return Err(DomainError::ValidationError {
                    message: "Line item quantity must be positive".to_string(),
                });
            }
        }
        
        Ok(())
    }

    /// Check if tracking data can be shared
    pub fn can_share_tracking_data(&self) -> bool {
        matches!(
            self.consent_settings.share_tracking_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if location history can be shared
    pub fn can_share_location_history(&self) -> bool {
        matches!(
            self.consent_settings.share_location_history,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if status updates can be shared
    pub fn can_share_status_updates(&self) -> bool {
        matches!(
            self.consent_settings.share_status_updates,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if carrier info can be shared
    pub fn can_share_carrier_info(&self) -> bool {
        matches!(
            self.consent_settings.share_carrier_info,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Calculate if the shipment is delayed
    pub fn is_delayed(&self) -> bool {
        match (self.status, self.actual_delivery) {
            (ShipmentStatus::Delivered, Some(actual_delivery)) => {
                actual_delivery > self.estimated_delivery
            }
            (_, None) => {
                // If not yet delivered, check if estimated delivery has passed
                Utc::now() > self.estimated_delivery
            }
            _ => false,
        }
    }

    /// Get the current transit duration in days
    pub fn current_transit_days(&self) -> i32 {
        let end_time = self.actual_delivery.unwrap_or_else(Utc::now);
        let duration = end_time - self.created_at;
        duration.num_days() as i32
    }
}
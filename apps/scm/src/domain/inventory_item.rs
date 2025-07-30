//! Inventory item entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::primitives::{Money, DomainError, Result};
use super::consent::InventoryConsentSettings;

/// Categories of inventory items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InventoryCategory {
    RawMaterials,
    WorkInProcess,
    FinishedGoods,
    Packaging,
    Supplies,
    Equipment,
}

/// Inventory item entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventoryItem {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: InventoryCategory,
    pub unit_of_measure: String,
    pub safety_stock_level: i32,
    pub reorder_point: i32,
    pub current_quantity: i32,
    pub warehouse_id: Uuid,
    pub unit_cost: Option<Money>, // Optional cost information
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: InventoryConsentSettings,
}

impl InventoryItem {
    /// Create a new inventory item
    pub fn new(
        sku: String,
        name: String,
        category: InventoryCategory,
        unit_of_measure: String,
        safety_stock_level: i32,
        reorder_point: i32,
        warehouse_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            sku,
            name,
            description: None,
            category,
            unit_of_measure,
            safety_stock_level,
            reorder_point,
            current_quantity: 0,
            warehouse_id,
            unit_cost: None,
            created_at: now,
            updated_at: now,
            consent_settings: InventoryConsentSettings::default(),
        }
    }

    /// Set description for the inventory item
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set unit cost for the inventory item
    pub fn with_unit_cost(mut self, unit_cost: Money) -> Self {
        self.unit_cost = Some(unit_cost);
        self
    }

    /// Set consent settings for the inventory item
    pub fn set_consent_settings(&mut self, settings: InventoryConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Update the quantity of the inventory item
    pub fn update_quantity(&mut self, delta: i32) -> Result<()> {
        let new_quantity = self.current_quantity + delta;
        
        // Prevent negative inventory (except for backorders with explicit configuration)
        if new_quantity < 0 {
            return Err(DomainError::InsufficientInventory);
        }
        
        self.current_quantity = new_quantity;
        self.updated_at = Utc::now();
        
        // Trigger reorder if below safety stock
        if self.current_quantity <= self.reorder_point {
            // In a real implementation, this would trigger a business workflow
            // For now, we just return a specific error that can be handled by the application layer
            return Err(DomainError::InvalidOperation {
                message: "Reorder point reached".to_string(),
            });
        }
        
        Ok(())
    }

    /// Check if reorder is needed
    pub fn is_reorder_needed(&self) -> bool {
        self.current_quantity <= self.reorder_point
    }

    /// Check if safety stock level is breached
    pub fn is_safety_stock_breached(&self) -> bool {
        self.current_quantity < self.safety_stock_level
    }

    /// Calculate total value of current inventory
    pub fn calculate_total_value(&self) -> Option<Money> {
        self.unit_cost.as_ref().map(|cost| {
            cost.multiply(rust_decimal::Decimal::from(self.current_quantity))
        })
    }

    /// Validate the inventory item
    pub fn validate(&self) -> Result<()> {
        // Safety stock level must be less than or equal to reorder point
        if self.safety_stock_level > self.reorder_point {
            return Err(DomainError::ValidationError {
                message: "Safety stock level must be less than or equal to reorder point".to_string(),
            });
        }
        
        // Quantity cannot be negative
        if self.current_quantity < 0 {
            return Err(DomainError::ValidationError {
                message: "Current quantity cannot be negative".to_string(),
            });
        }
        
        // Reorder point must be non-negative
        if self.reorder_point < 0 {
            return Err(DomainError::ValidationError {
                message: "Reorder point must be non-negative".to_string(),
            });
        }
        
        // Safety stock level must be non-negative
        if self.safety_stock_level < 0 {
            return Err(DomainError::ValidationError {
                message: "Safety stock level must be non-negative".to_string(),
            });
        }
        
        Ok(())
    }

    /// Check if quantities can be shared
    pub fn can_share_quantities(&self) -> bool {
        matches!(
            self.consent_settings.share_quantities,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if cost data can be shared
    pub fn can_share_cost_data(&self) -> bool {
        matches!(
            self.consent_settings.share_cost_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if movement history can be shared
    pub fn can_share_movement_history(&self) -> bool {
        matches!(
            self.consent_settings.share_movement_history,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }
}
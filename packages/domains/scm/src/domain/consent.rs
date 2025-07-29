//! Consent framework for the SCM module
//!
//! This module provides integration with the centralized Consent Manager
//! for managing data sharing permissions in the SCM module.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::primitives::DataSharingLevel;
use uuid::Uuid;
use consent_manager::domain::consent::{DataSharingLevel as NewDataSharingLevel, Domain};
use consent_manager::application::service::ConsentService;
use std::sync::Arc;

/// Consent settings for supply chain networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkConsentSettings {
    pub share_topology: DataSharingLevel,
    pub share_lead_times: DataSharingLevel,
    pub share_node_details: DataSharingLevel,
    pub share_performance_data: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl NetworkConsentSettings {
    /// Create NetworkConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_topology: legacy_level.clone(),
            share_lead_times: legacy_level.clone(),
            share_node_details: legacy_level.clone(),
            share_performance_data: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for NetworkConsentSettings {
    fn default() -> Self {
        Self {
            share_topology: DataSharingLevel::None,
            share_lead_times: DataSharingLevel::None,
            share_node_details: DataSharingLevel::None,
            share_performance_data: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Consent settings for inventory items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryConsentSettings {
    pub share_quantities: DataSharingLevel,
    pub share_cost_data: DataSharingLevel,
    pub share_movement_history: DataSharingLevel,
    pub share_reorder_points: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl InventoryConsentSettings {
    /// Create InventoryConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_quantities: legacy_level.clone(),
            share_cost_data: legacy_level.clone(),
            share_movement_history: legacy_level.clone(),
            share_reorder_points: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for InventoryConsentSettings {
    fn default() -> Self {
        Self {
            share_quantities: DataSharingLevel::None,
            share_cost_data: DataSharingLevel::None,
            share_movement_history: DataSharingLevel::None,
            share_reorder_points: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Consent settings for procurement orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcurementConsentSettings {
    pub share_with_supplier: DataSharingLevel,
    pub share_price_data: DataSharingLevel,
    pub share_delivery_schedule: DataSharingLevel,
    pub share_line_items: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl ProcurementConsentSettings {
    /// Create ProcurementConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_with_supplier: legacy_level.clone(),
            share_price_data: legacy_level.clone(),
            share_delivery_schedule: legacy_level.clone(),
            share_line_items: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for ProcurementConsentSettings {
    fn default() -> Self {
        Self {
            share_with_supplier: DataSharingLevel::None,
            share_price_data: DataSharingLevel::None,
            share_delivery_schedule: DataSharingLevel::None,
            share_line_items: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Consent settings for shipments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShipmentConsentSettings {
    pub share_tracking_data: DataSharingLevel,
    pub share_location_history: DataSharingLevel,
    pub share_status_updates: DataSharingLevel,
    pub share_carrier_info: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl ShipmentConsentSettings {
    /// Create ShipmentConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_tracking_data: legacy_level.clone(),
            share_location_history: legacy_level.clone(),
            share_status_updates: legacy_level.clone(),
            share_carrier_info: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for ShipmentConsentSettings {
    fn default() -> Self {
        Self {
            share_tracking_data: DataSharingLevel::None,
            share_location_history: DataSharingLevel::None,
            share_status_updates: DataSharingLevel::None,
            share_carrier_info: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Consent settings for warehouses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WarehouseConsentSettings {
    pub share_capacity_data: DataSharingLevel,
    pub share_utilization_data: DataSharingLevel,
    pub share_location_data: DataSharingLevel,
    pub share_operating_hours: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl WarehouseConsentSettings {
    /// Create WarehouseConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_capacity_data: legacy_level.clone(),
            share_utilization_data: legacy_level.clone(),
            share_location_data: legacy_level.clone(),
            share_operating_hours: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for WarehouseConsentSettings {
    fn default() -> Self {
        Self {
            share_capacity_data: DataSharingLevel::None,
            share_utilization_data: DataSharingLevel::None,
            share_location_data: DataSharingLevel::None,
            share_operating_hours: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Consent settings for suppliers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupplierConsentSettings {
    pub share_contact_info: DataSharingLevel,
    pub share_performance_metrics: DataSharingLevel,
    pub share_contract_details: DataSharingLevel,
    pub share_certification_data: DataSharingLevel,
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl SupplierConsentSettings {
    /// Create SupplierConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &NewDataSharingLevel) -> Self {
        let legacy_level = map_new_to_legacy_level(level);
        Self {
            share_contact_info: legacy_level.clone(),
            share_performance_metrics: legacy_level.clone(),
            share_contract_details: legacy_level.clone(),
            share_certification_data: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for SupplierConsentSettings {
    fn default() -> Self {
        Self {
            share_contact_info: DataSharingLevel::None,
            share_performance_metrics: DataSharingLevel::None,
            share_contract_details: DataSharingLevel::None,
            share_certification_data: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
}

/// Map the new DataSharingLevel to the legacy SCM DataSharingLevel
fn map_new_to_legacy_level(level: &NewDataSharingLevel) -> DataSharingLevel {
    match level {
        NewDataSharingLevel::None => DataSharingLevel::None,
        NewDataSharingLevel::Minimal => DataSharingLevel::ViewOnly,
        NewDataSharingLevel::Standard => DataSharingLevel::Editable,
        NewDataSharingLevel::Full => DataSharingLevel::FullAccess,
    }
}
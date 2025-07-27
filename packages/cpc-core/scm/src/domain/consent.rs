//! Consent framework for the SCM module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::primitives::DataSharingLevel;
use uuid::Uuid;

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
    pub fn new() -> Self {
        Self {
            share_topology: DataSharingLevel::None,
            share_lead_times: DataSharingLevel::None,
            share_node_details: DataSharingLevel::None,
            share_performance_data: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_topology: DataSharingLevel::FullAccess,
            share_lead_times: DataSharingLevel::FullAccess,
            share_node_details: DataSharingLevel::FullAccess,
            share_performance_data: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_topology: DataSharingLevel::ViewOnly,
            share_lead_times: DataSharingLevel::ViewOnly,
            share_node_details: DataSharingLevel::ViewOnly,
            share_performance_data: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for NetworkConsentSettings {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            share_quantities: DataSharingLevel::None,
            share_cost_data: DataSharingLevel::None,
            share_movement_history: DataSharingLevel::None,
            share_reorder_points: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_quantities: DataSharingLevel::FullAccess,
            share_cost_data: DataSharingLevel::FullAccess,
            share_movement_history: DataSharingLevel::FullAccess,
            share_reorder_points: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_quantities: DataSharingLevel::ViewOnly,
            share_cost_data: DataSharingLevel::ViewOnly,
            share_movement_history: DataSharingLevel::ViewOnly,
            share_reorder_points: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for InventoryConsentSettings {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            share_with_supplier: DataSharingLevel::None,
            share_price_data: DataSharingLevel::None,
            share_delivery_schedule: DataSharingLevel::None,
            share_line_items: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_with_supplier: DataSharingLevel::FullAccess,
            share_price_data: DataSharingLevel::FullAccess,
            share_delivery_schedule: DataSharingLevel::FullAccess,
            share_line_items: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_with_supplier: DataSharingLevel::ViewOnly,
            share_price_data: DataSharingLevel::ViewOnly,
            share_delivery_schedule: DataSharingLevel::ViewOnly,
            share_line_items: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for ProcurementConsentSettings {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            share_tracking_data: DataSharingLevel::None,
            share_location_history: DataSharingLevel::None,
            share_status_updates: DataSharingLevel::None,
            share_carrier_info: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_tracking_data: DataSharingLevel::FullAccess,
            share_location_history: DataSharingLevel::FullAccess,
            share_status_updates: DataSharingLevel::FullAccess,
            share_carrier_info: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_tracking_data: DataSharingLevel::ViewOnly,
            share_location_history: DataSharingLevel::ViewOnly,
            share_status_updates: DataSharingLevel::ViewOnly,
            share_carrier_info: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for ShipmentConsentSettings {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            share_capacity_data: DataSharingLevel::None,
            share_utilization_data: DataSharingLevel::None,
            share_location_data: DataSharingLevel::None,
            share_operating_hours: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_capacity_data: DataSharingLevel::FullAccess,
            share_utilization_data: DataSharingLevel::FullAccess,
            share_location_data: DataSharingLevel::FullAccess,
            share_operating_hours: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_capacity_data: DataSharingLevel::ViewOnly,
            share_utilization_data: DataSharingLevel::ViewOnly,
            share_location_data: DataSharingLevel::ViewOnly,
            share_operating_hours: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for WarehouseConsentSettings {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            share_contact_info: DataSharingLevel::None,
            share_performance_metrics: DataSharingLevel::None,
            share_contract_details: DataSharingLevel::None,
            share_certification_data: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_full_access() -> Self {
        Self {
            share_contact_info: DataSharingLevel::FullAccess,
            share_performance_metrics: DataSharingLevel::FullAccess,
            share_contract_details: DataSharingLevel::FullAccess,
            share_certification_data: DataSharingLevel::FullAccess,
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_view_only() -> Self {
        Self {
            share_contact_info: DataSharingLevel::ViewOnly,
            share_performance_metrics: DataSharingLevel::ViewOnly,
            share_contract_details: DataSharingLevel::ViewOnly,
            share_certification_data: DataSharingLevel::ViewOnly,
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for SupplierConsentSettings {
    fn default() -> Self {
        Self::new()
    }
}
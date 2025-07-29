//! Consent service integration for the SCM module
//!
//! This module provides integration with the centralized Consent Manager
//! for managing data sharing permissions in the SCM module.

use crate::domain::consent::{
    NetworkConsentSettings, InventoryConsentSettings, ProcurementConsentSettings,
    ShipmentConsentSettings, WarehouseConsentSettings, SupplierConsentSettings
};
use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
    },
    application::service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

/// Service for checking and managing SCM consent
pub struct ScmConsentService {
    consent_service: Arc<ConsentService>,
}

impl ScmConsentService {
    /// Create a new SCM consent service
    pub fn new(consent_service: Arc<ConsentService>) -> Self {
        Self { consent_service }
    }

    /// Get network consent settings for a user
    pub async fn get_network_consent(&self, user_id: Uuid) -> Result<NetworkConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent levels for each network data category
        let share_topology = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all network settings
        // In a more complex implementation, we might have separate categories
        let settings = NetworkConsentSettings {
            share_topology: map_data_sharing_level(&share_topology),
            share_lead_times: map_data_sharing_level(&share_topology),
            share_node_details: map_data_sharing_level(&share_topology),
            share_performance_data: map_data_sharing_level(&share_topology),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Get inventory consent settings for a user
    pub async fn get_inventory_consent(&self, user_id: Uuid) -> Result<InventoryConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent level for inventory data
        let share_inventory = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all inventory settings
        let settings = InventoryConsentSettings {
            share_quantities: map_data_sharing_level(&share_inventory),
            share_cost_data: map_data_sharing_level(&share_inventory),
            share_movement_history: map_data_sharing_level(&share_inventory),
            share_reorder_points: map_data_sharing_level(&share_inventory),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Get procurement consent settings for a user
    pub async fn get_procurement_consent(&self, user_id: Uuid) -> Result<ProcurementConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent level for procurement data
        let share_procurement = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all procurement settings
        let settings = ProcurementConsentSettings {
            share_with_supplier: map_data_sharing_level(&share_procurement),
            share_price_data: map_data_sharing_level(&share_procurement),
            share_delivery_schedule: map_data_sharing_level(&share_procurement),
            share_line_items: map_data_sharing_level(&share_procurement),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Get shipment consent settings for a user
    pub async fn get_shipment_consent(&self, user_id: Uuid) -> Result<ShipmentConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent level for shipment data
        let share_shipment = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all shipment settings
        let settings = ShipmentConsentSettings {
            share_tracking_data: map_data_sharing_level(&share_shipment),
            share_location_history: map_data_sharing_level(&share_shipment),
            share_status_updates: map_data_sharing_level(&share_shipment),
            share_carrier_info: map_data_sharing_level(&share_shipment),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Get warehouse consent settings for a user
    pub async fn get_warehouse_consent(&self, user_id: Uuid) -> Result<WarehouseConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent level for warehouse data
        let share_warehouse = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all warehouse settings
        let settings = WarehouseConsentSettings {
            share_capacity_data: map_data_sharing_level(&share_warehouse),
            share_utilization_data: map_data_sharing_level(&share_warehouse),
            share_location_data: map_data_sharing_level(&share_warehouse),
            share_operating_hours: map_data_sharing_level(&share_warehouse),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Get supplier consent settings for a user
    pub async fn get_supplier_consent(&self, user_id: Uuid) -> Result<SupplierConsentSettings, Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        
        // Get consent level for supplier data
        let share_supplier = self.consent_service
            .get_consent_level(&user_id_str, Domain::ScmData)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        // For now, we're using the same level for all supplier settings
        let settings = SupplierConsentSettings {
            share_contact_info: map_data_sharing_level(&share_supplier),
            share_performance_metrics: map_data_sharing_level(&share_supplier),
            share_contract_details: map_data_sharing_level(&share_supplier),
            share_certification_data: map_data_sharing_level(&share_supplier),
            custom_fields: std::collections::HashMap::new(),
        };
        
        Ok(settings)
    }

    /// Update consent level for SCM data
    pub async fn update_consent_level(
        &self,
        user_id: Uuid,
        level: DataSharingLevel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_id_str = user_id.to_string();
        let actor = consent_manager::domain::audit::Actor::User(user_id_str.clone());
        
        // Update the consent level for SCM data
        self.consent_service
            .update_consent_level(&user_id_str, Domain::ScmData, level, actor)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            
        Ok(())
    }
}

/// Map the new DataSharingLevel to the legacy SCM DataSharingLevel
fn map_data_sharing_level(level: &DataSharingLevel) -> crate::domain::primitives::DataSharingLevel {
    match level {
        DataSharingLevel::None => crate::domain::primitives::DataSharingLevel::None,
        DataSharingLevel::Minimal => crate::domain::primitives::DataSharingLevel::ViewOnly,
        DataSharingLevel::Standard => crate::domain::primitives::DataSharingLevel::Editable,
        DataSharingLevel::Full => crate::domain::primitives::DataSharingLevel::FullAccess,
    }
}
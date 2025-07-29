//! Database models for the SCM module
//!
//! This module contains the SQLx models that map to database tables
//! for SCM entities.

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use rust_decimal::Decimal;

/// Database model for supply chain networks
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplyChainNetworkModel {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub nodes: Option<sqlx::types::Json<Vec<NetworkNodeModel>>>,
    pub connections: Option<sqlx::types::Json<Vec<NetworkConnectionModel>>>,
    pub consent_settings: Option<sqlx::types::Json<NetworkConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for network nodes
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NetworkNodeModel {
    pub id: Uuid,
    pub name: String,
    pub node_type: String, // "supplier", "warehouse", "distribution_center", "retail_location", "manufacturing_plant", "customer"
    pub latitude: f64,
    pub longitude: f64,
    pub capacity: Option<i32>,
    pub operating_hours: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for network connections
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NetworkConnectionModel {
    pub id: Uuid,
    pub start_node_id: Uuid,
    pub end_node_id: Uuid,
    pub lead_time_days: i32,
    pub cost_per_unit: Option<f64>,
    pub transport_mode: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for network consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NetworkConsentSettingsModel {
    pub share_topology: String, // "none", "view_only", "editable", "full_access"
    pub share_lead_times: String,
    pub share_node_details: String,
    pub share_performance_data: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for inventory items
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryItemModel {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String, // "raw_materials", "work_in_process", "finished_goods", "packaging", "supplies", "equipment"
    pub unit_of_measure: String,
    pub safety_stock_level: i32,
    pub reorder_point: i32,
    pub current_quantity: i32,
    pub warehouse_id: Uuid,
    pub unit_cost_amount: Option<Decimal>,
    pub unit_cost_currency: Option<String>,
    pub consent_settings: Option<sqlx::types::Json<InventoryConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for inventory consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryConsentSettingsModel {
    pub share_quantities: String, // "none", "view_only", "editable", "full_access"
    pub share_cost_data: String,
    pub share_movement_history: String,
    pub share_reorder_points: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for procurement orders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcurementOrderModel {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub order_number: String,
    pub status: String, // "draft", "submitted", "approved", "rejected", "shipped", "received", "cancelled"
    pub expected_delivery: DateTime<Utc>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub total_amount: Decimal,
    pub total_amount_currency: String,
    pub consent_settings: Option<sqlx::types::Json<ProcurementConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for procurement order line items
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderLineItemModel {
    pub id: Uuid,
    pub order_id: Uuid,
    pub inventory_item_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: Decimal,
    pub unit_price_currency: String,
    pub extended_price_amount: Decimal,
    pub extended_price_currency: String,
    pub description: Option<String>,
}

/// Database model for procurement consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcurementConsentSettingsModel {
    pub share_with_supplier: String, // "none", "view_only", "editable", "full_access"
    pub share_price_data: String,
    pub share_delivery_schedule: String,
    pub share_line_items: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for shipments
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentModel {
    pub id: Uuid,
    pub tracking_number: String,
    pub carrier: String,
    pub status: String, // "created", "in_transit", "delayed", "delivered", "cancelled"
    pub origin_id: Uuid,
    pub destination_id: Uuid,
    pub expected_transit_days: i32,
    pub actual_transit_days: Option<i32>,
    pub estimated_delivery: DateTime<Utc>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub consent_settings: Option<sqlx::types::Json<ShipmentConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for shipment line items
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentLineItemModel {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub inventory_item_id: Uuid,
    pub quantity: i32,
    pub description: Option<String>,
}

/// Database model for shipment consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentConsentSettingsModel {
    pub share_tracking_data: String, // "none", "view_only", "editable", "full_access"
    pub share_location_history: String,
    pub share_status_updates: String,
    pub share_carrier_info: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for warehouses
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WarehouseModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub capacity: i32,
    pub current_utilization: i32,
    pub open_time: String, // Format: "HH:MM:SS"
    pub close_time: String, // Format: "HH:MM:SS"
    pub contact_info: Option<String>,
    pub consent_settings: Option<sqlx::types::Json<WarehouseConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for warehouse consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WarehouseConsentSettingsModel {
    pub share_capacity_data: String, // "none", "view_only", "editable", "full_access"
    pub share_utilization_data: String,
    pub share_location_data: String,
    pub share_operating_hours: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for suppliers
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub website: Option<String>,
    pub delivery_time_score: f64,
    pub quality_score: f64,
    pub responsiveness_score: f64,
    pub last_evaluation_date: DateTime<Utc>,
    pub is_critical: bool,
    pub consent_settings: Option<sqlx::types::Json<SupplierConsentSettingsModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for supplier contracts
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContractModel {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub terms: String,
    pub renewal_required: bool,
    pub special_certifications: Option<sqlx::types::Json<Vec<String>>>,
}

/// Database model for supplier consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierConsentSettingsModel {
    pub share_contact_info: String, // "none", "view_only", "editable", "full_access"
    pub share_performance_metrics: String,
    pub share_contract_details: String,
    pub share_certification_data: String,
    pub custom_fields: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_supply_chain_network_model_serialization() {
        let model = SupplyChainNetworkModel {
            id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            name: "Test Network".to_string(),
            nodes: Some(sqlx::types::Json(vec![NetworkNodeModel {
                id: Uuid::new_v4(),
                name: "Test Node".to_string(),
                node_type: "warehouse".to_string(),
                latitude: 40.7128,
                longitude: -74.0060,
                capacity: Some(1000),
                operating_hours: Some("08:00-17:00".to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }])),
            connections: Some(sqlx::types::Json(vec![NetworkConnectionModel {
                id: Uuid::new_v4(),
                start_node_id: Uuid::new_v4(),
                end_node_id: Uuid::new_v4(),
                lead_time_days: 5,
                cost_per_unit: Some(10.50),
                transport_mode: "truck".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }])),
            consent_settings: Some(sqlx::types::Json(NetworkConsentSettingsModel {
                share_topology: "view_only".to_string(),
                share_lead_times: "view_only".to_string(),
                share_node_details: "none".to_string(),
                share_performance_data: "none".to_string(),
                custom_fields: HashMap::new(),
            })),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: SupplyChainNetworkModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.name, deserialized.name);
    }

    #[test]
    fn test_inventory_item_model_serialization() {
        let model = InventoryItemModel {
            id: Uuid::new_v4(),
            sku: "TEST-001".to_string(),
            name: "Test Item".to_string(),
            description: Some("A test inventory item".to_string()),
            category: "raw_materials".to_string(),
            unit_of_measure: "units".to_string(),
            safety_stock_level: 100,
            reorder_point: 50,
            current_quantity: 75,
            warehouse_id: Uuid::new_v4(),
            unit_cost_amount: Some(Decimal::new(2500, 2)), // $25.00
            unit_cost_currency: Some("USD".to_string()),
            consent_settings: Some(sqlx::types::Json(InventoryConsentSettingsModel {
                share_quantities: "view_only".to_string(),
                share_cost_data: "none".to_string(),
                share_movement_history: "none".to_string(),
                share_reorder_points: "view_only".to_string(),
                custom_fields: HashMap::new(),
            })),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: InventoryItemModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.sku, deserialized.sku);
        assert_eq!(model.name, deserialized.name);
    }

    #[test]
    fn test_warehouse_model_serialization() {
        let model = WarehouseModel {
            id: Uuid::new_v4(),
            name: "Main Warehouse".to_string(),
            description: Some("Primary distribution center".to_string()),
            latitude: 40.7128,
            longitude: -74.0060,
            capacity: 10000,
            current_utilization: 7500,
            open_time: "08:00:00".to_string(),
            close_time: "17:00:00".to_string(),
            contact_info: Some("warehouse@example.com".to_string()),
            consent_settings: Some(sqlx::types::Json(WarehouseConsentSettingsModel {
                share_capacity_data: "view_only".to_string(),
                share_utilization_data: "view_only".to_string(),
                share_location_data: "view_only".to_string(),
                share_operating_hours: "view_only".to_string(),
                custom_fields: HashMap::new(),
            })),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: WarehouseModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.name, deserialized.name);
        assert_eq!(model.capacity, deserialized.capacity);
    }

    #[test]
    fn test_supplier_model_serialization() {
        let model = SupplierModel {
            id: Uuid::new_v4(),
            name: "Test Supplier".to_string(),
            description: Some("A reliable supplier".to_string()),
            email: "supplier@example.com".to_string(),
            phone: "+1-555-123-4567".to_string(),
            address: "123 Supplier St, City, Country".to_string(),
            website: Some("https://supplier.example.com".to_string()),
            delivery_time_score: 0.95,
            quality_score: 0.98,
            responsiveness_score: 0.90,
            last_evaluation_date: Utc::now(),
            is_critical: true,
            consent_settings: Some(sqlx::types::Json(SupplierConsentSettingsModel {
                share_contact_info: "view_only".to_string(),
                share_performance_metrics: "view_only".to_string(),
                share_contract_details: "none".to_string(),
                share_certification_data: "none".to_string(),
                custom_fields: HashMap::new(),
            })),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: SupplierModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.name, deserialized.name);
        assert_eq!(model.email, deserialized.email);
    }
}
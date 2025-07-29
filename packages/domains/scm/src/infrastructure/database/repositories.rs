//! Database repositories for the SCM module
//!
//! This module contains the SQLx repository implementations for SCM entities.

use sqlx::PgPool;
use uuid::Uuid;
use crate::infrastructure::database::models::*;
use crate::domain::{
    supply_chain_network::{SupplyChainNetwork, NetworkNode, NetworkConnection, NodeType},
    inventory_item::{InventoryItem, InventoryCategory},
    procurement_order::{ProcurementOrder, OrderLineItem, OrderStatus},
    shipment::{Shipment, ShipmentLineItem, ShipmentStatus},
    warehouse::Warehouse,
    supplier::{Supplier, ContactInformation, SupplierMetrics, Contract},
    primitives::{Money, Currency, GeoLocation, OperatingHours, DataSharingLevel},
    consent::{
        NetworkConsentSettings, InventoryConsentSettings, ProcurementConsentSettings,
        ShipmentConsentSettings, WarehouseConsentSettings, SupplierConsentSettings
    },
};
use chrono::{Duration, DateTime, Utc, NaiveTime};
use std::collections::HashMap;
use thiserror::Error;
use rust_decimal::Decimal;

/// Error types for repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Network not found: {0}")]
    NetworkNotFound(Uuid),
    
    #[error("Inventory item not found: {0}")]
    InventoryItemNotFound(Uuid),
    
    #[error("Procurement order not found: {0}")]
    ProcurementOrderNotFound(Uuid),
    
    #[error("Shipment not found: {0}")]
    ShipmentNotFound(Uuid),
    
    #[error("Warehouse not found: {0}")]
    WarehouseNotFound(Uuid),
    
    #[error("Supplier not found: {0}")]
    SupplierNotFound(Uuid),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

// Supply Chain Network Repository
#[async_trait::async_trait]
pub trait SupplyChainNetworkRepository: Send + Sync {
    async fn save(&self, network: &SupplyChainNetwork) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<SupplyChainNetwork, RepositoryError>;
    async fn find_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<SupplyChainNetwork>, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct PgSupplyChainNetworkRepository {
    pool: PgPool,
}

impl PgSupplyChainNetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    fn convert_network_to_model(&self, network: &SupplyChainNetwork) -> Result<SupplyChainNetworkModel, RepositoryError> {
        let nodes: Vec<NetworkNodeModel> = network.nodes.iter().map(|node| {
            NetworkNodeModel {
                id: node.id,
                name: node.name.clone(),
                node_type: match node.node_type {
                    NodeType::Supplier => "supplier".to_string(),
                    NodeType::Warehouse => "warehouse".to_string(),
                    NodeType::DistributionCenter => "distribution_center".to_string(),
                    NodeType::RetailLocation => "retail_location".to_string(),
                    NodeType::ManufacturingPlant => "manufacturing_plant".to_string(),
                    NodeType::Customer => "customer".to_string(),
                },
                latitude: node.location.latitude,
                longitude: node.location.longitude,
                capacity: node.capacity,
                operating_hours: node.operating_hours.clone(),
                created_at: node.created_at,
                updated_at: node.updated_at,
            }
        }).collect();
        
        let connections: Vec<NetworkConnectionModel> = network.connections.iter().map(|conn| {
            NetworkConnectionModel {
                id: conn.id,
                start_node_id: conn.start_node_id,
                end_node_id: conn.end_node_id,
                lead_time_days: conn.lead_time_days,
                cost_per_unit: conn.cost_per_unit,
                transport_mode: conn.transport_mode.clone(),
                created_at: conn.created_at,
                updated_at: conn.updated_at,
            }
        }).collect();
        
        let consent_settings = NetworkConsentSettingsModel {
            share_topology: match network.consent_settings.share_topology {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_lead_times: match network.consent_settings.share_lead_times {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_node_details: match network.consent_settings.share_node_details {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_performance_data: match network.consent_settings.share_performance_data {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            custom_fields: network.consent_settings.custom_fields.clone(),
        };
        
        Ok(SupplyChainNetworkModel {
            id: network.id,
            owner_id: network.owner_id,
            name: network.name.clone(),
            nodes: Some(sqlx::types::Json(nodes)),
            connections: Some(sqlx::types::Json(connections)),
            consent_settings: Some(sqlx::types::Json(consent_settings)),
            created_at: network.created_at,
            updated_at: network.updated_at,
        })
    }
    
    fn convert_model_to_network(&self, model: SupplyChainNetworkModel) -> Result<SupplyChainNetwork, RepositoryError> {
        let nodes: Vec<NetworkNode> = model.nodes
            .map(|nodes_json| nodes_json.0)
            .unwrap_or_default()
            .into_iter()
            .map(|node_model| {
                let node_type = match node_model.node_type.as_str() {
                    "supplier" => NodeType::Supplier,
                    "warehouse" => NodeType::Warehouse,
                    "distribution_center" => NodeType::DistributionCenter,
                    "retail_location" => NodeType::RetailLocation,
                    "manufacturing_plant" => NodeType::ManufacturingPlant,
                    "customer" => NodeType::Customer,
                    _ => NodeType::Warehouse,
                };
                
                NetworkNode {
                    id: node_model.id,
                    name: node_model.name,
                    node_type,
                    location: GeoLocation {
                        latitude: node_model.latitude,
                        longitude: node_model.longitude,
                    },
                    capacity: node_model.capacity,
                    operating_hours: node_model.operating_hours,
                    created_at: node_model.created_at,
                    updated_at: node_model.updated_at,
                }
            })
            .collect();
        
        let connections: Vec<NetworkConnection> = model.connections
            .map(|connections_json| connections_json.0)
            .unwrap_or_default()
            .into_iter()
            .map(|conn_model| {
                NetworkConnection {
                    id: conn_model.id,
                    start_node_id: conn_model.start_node_id,
                    end_node_id: conn_model.end_node_id,
                    lead_time_days: conn_model.lead_time_days,
                    cost_per_unit: conn_model.cost_per_unit,
                    transport_mode: conn_model.transport_mode,
                    created_at: conn_model.created_at,
                    updated_at: conn_model.updated_at,
                }
            })
            .collect();
        
        let consent_settings_model = model.consent_settings
            .ok_or_else(|| RepositoryError::ConversionError("Missing consent_settings".to_string()))?
            .0;
        
        let consent_settings = NetworkConsentSettings {
            share_topology: match consent_settings_model.share_topology.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_lead_times: match consent_settings_model.share_lead_times.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_node_details: match consent_settings_model.share_node_details.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_performance_data: match consent_settings_model.share_performance_data.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            custom_fields: consent_settings_model.custom_fields,
        };
        
        Ok(SupplyChainNetwork {
            id: model.id,
            owner_id: model.owner_id,
            name: model.name,
            nodes,
            connections,
            created_at: model.created_at,
            updated_at: model.updated_at,
            consent_settings,
        })
    }
}

#[async_trait::async_trait]
impl SupplyChainNetworkRepository for PgSupplyChainNetworkRepository {
    async fn save(&self, network: &SupplyChainNetwork) -> Result<(), RepositoryError> {
        let model = self.convert_network_to_model(network)?;
        
        sqlx::query!(
            r#"
            INSERT INTO supply_chain_networks (
                id, owner_id, name, nodes, connections, consent_settings,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            ON CONFLICT (id) DO UPDATE SET
                owner_id = $2, name = $3, nodes = $4, connections = $5,
                consent_settings = $6, updated_at = $8
            "#,
            model.id,
            model.owner_id,
            model.name,
            model.nodes as Option<sqlx::types::Json<Vec<NetworkNodeModel>>>,
            model.connections as Option<sqlx::types::Json<Vec<NetworkConnectionModel>>>,
            model.consent_settings as Option<sqlx::types::Json<NetworkConsentSettingsModel>>,
            model.created_at,
            model.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<SupplyChainNetwork, RepositoryError> {
        let row = sqlx::query_as!(
            SupplyChainNetworkModel,
            r#"
            SELECT id, owner_id, name, nodes, connections, consent_settings,
                   created_at, updated_at
            FROM supply_chain_networks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let model = row.ok_or_else(|| RepositoryError::NetworkNotFound(id))?;
        let network = self.convert_model_to_network(model)?;
        
        Ok(network)
    }
    
    async fn find_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<SupplyChainNetwork>, RepositoryError> {
        let rows = sqlx::query_as!(
            SupplyChainNetworkModel,
            r#"
            SELECT id, owner_id, name, nodes, connections, consent_settings,
                   created_at, updated_at
            FROM supply_chain_networks
            WHERE owner_id = $1
            ORDER BY name
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut networks = Vec::new();
        for model in rows {
            let network = self.convert_model_to_network(model)?;
            networks.push(network);
        }
        
        Ok(networks)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM supply_chain_networks WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

// Inventory Item Repository
#[async_trait::async_trait]
pub trait InventoryItemRepository: Send + Sync {
    async fn save(&self, item: &InventoryItem) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<InventoryItem, RepositoryError>;
    async fn find_by_warehouse_id(&self, warehouse_id: Uuid) -> Result<Vec<InventoryItem>, RepositoryError>;
    async fn find_by_sku(&self, sku: &str) -> Result<InventoryItem, RepositoryError>;
    async fn find_items_needing_reorder(&self) -> Result<Vec<InventoryItem>, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct PgInventoryItemRepository {
    pool: PgPool,
}

impl PgInventoryItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    fn convert_item_to_model(&self, item: &InventoryItem) -> Result<InventoryItemModel, RepositoryError> {
        let category = match item.category {
            InventoryCategory::RawMaterials => "raw_materials".to_string(),
            InventoryCategory::WorkInProcess => "work_in_process".to_string(),
            InventoryCategory::FinishedGoods => "finished_goods".to_string(),
            InventoryCategory::Packaging => "packaging".to_string(),
            InventoryCategory::Supplies => "supplies".to_string(),
            InventoryCategory::Equipment => "equipment".to_string(),
        };
        
        let consent_settings = InventoryConsentSettingsModel {
            share_quantities: match item.consent_settings.share_quantities {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_cost_data: match item.consent_settings.share_cost_data {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_movement_history: match item.consent_settings.share_movement_history {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_reorder_points: match item.consent_settings.share_reorder_points {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            custom_fields: item.consent_settings.custom_fields.clone(),
        };
        
        Ok(InventoryItemModel {
            id: item.id,
            sku: item.sku.clone(),
            name: item.name.clone(),
            description: item.description.clone(),
            category,
            unit_of_measure: item.unit_of_measure.clone(),
            safety_stock_level: item.safety_stock_level,
            reorder_point: item.reorder_point,
            current_quantity: item.current_quantity,
            warehouse_id: item.warehouse_id,
            unit_cost_amount: item.unit_cost.as_ref().map(|m| m.amount),
            unit_cost_currency: item.unit_cost.as_ref().map(|m| match m.currency {
                Currency::USD => "USD".to_string(),
                Currency::EUR => "EUR".to_string(),
                Currency::GBP => "GBP".to_string(),
                Currency::JPY => "JPY".to_string(),
                Currency::CAD => "CAD".to_string(),
                Currency::AUD => "AUD".to_string(),
                Currency::CHF => "CHF".to_string(),
                Currency::CNY => "CNY".to_string(),
                Currency::SEK => "SEK".to_string(),
                Currency::NZD => "NZD".to_string(),
                Currency::MXN => "MXN".to_string(),
                Currency::SGD => "SGD".to_string(),
                Currency::HKD => "HKD".to_string(),
                Currency::NOK => "NOK".to_string(),
                Currency::KRW => "KRW".to_string(),
                Currency::TRY => "TRY".to_string(),
                Currency::RUB => "RUB".to_string(),
                Currency::INR => "INR".to_string(),
                Currency::BRL => "BRL".to_string(),
                Currency::ZAR => "ZAR".to_string(),
            }),
            consent_settings: Some(sqlx::types::Json(consent_settings)),
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
    }
    
    fn convert_model_to_item(&self, model: InventoryItemModel) -> Result<InventoryItem, RepositoryError> {
        let category = match model.category.as_str() {
            "raw_materials" => InventoryCategory::RawMaterials,
            "work_in_process" => InventoryCategory::WorkInProcess,
            "finished_goods" => InventoryCategory::FinishedGoods,
            "packaging" => InventoryCategory::Packaging,
            "supplies" => InventoryCategory::Supplies,
            "equipment" => InventoryCategory::Equipment,
            _ => InventoryCategory::Supplies,
        };
        
        let unit_cost = match (model.unit_cost_amount, model.unit_cost_currency) {
            (Some(amount), Some(currency_str)) => {
                let currency = match currency_str.as_str() {
                    "USD" => Currency::USD,
                    "EUR" => Currency::EUR,
                    "GBP" => Currency::GBP,
                    "JPY" => Currency::JPY,
                    "CAD" => Currency::CAD,
                    "AUD" => Currency::AUD,
                    "CHF" => Currency::CHF,
                    "CNY" => Currency::CNY,
                    "SEK" => Currency::SEK,
                    "NZD" => Currency::NZD,
                    "MXN" => Currency::MXN,
                    "SGD" => Currency::SGD,
                    "HKD" => Currency::HKD,
                    "NOK" => Currency::NOK,
                    "KRW" => Currency::KRW,
                    "TRY" => Currency::TRY,
                    "RUB" => Currency::RUB,
                    "INR" => Currency::INR,
                    "BRL" => Currency::BRL,
                    "ZAR" => Currency::ZAR,
                    _ => Currency::USD,
                };
                Some(Money { amount, currency })
            }
            _ => None,
        };
        
        let consent_settings_model = model.consent_settings
            .ok_or_else(|| RepositoryError::ConversionError("Missing consent_settings".to_string()))?
            .0;
        
        let consent_settings = InventoryConsentSettings {
            share_quantities: match consent_settings_model.share_quantities.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_cost_data: match consent_settings_model.share_cost_data.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_movement_history: match consent_settings_model.share_movement_history.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_reorder_points: match consent_settings_model.share_reorder_points.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            custom_fields: consent_settings_model.custom_fields,
        };
        
        Ok(InventoryItem {
            id: model.id,
            sku: model.sku,
            name: model.name,
            description: model.description,
            category,
            unit_of_measure: model.unit_of_measure,
            safety_stock_level: model.safety_stock_level,
            reorder_point: model.reorder_point,
            current_quantity: model.current_quantity,
            warehouse_id: model.warehouse_id,
            unit_cost,
            created_at: model.created_at,
            updated_at: model.updated_at,
            consent_settings,
        })
    }
}

#[async_trait::async_trait]
impl InventoryItemRepository for PgInventoryItemRepository {
    async fn save(&self, item: &InventoryItem) -> Result<(), RepositoryError> {
        let model = self.convert_item_to_model(item)?;
        
        sqlx::query!(
            r#"
            INSERT INTO inventory_items (
                id, sku, name, description, category, unit_of_measure,
                safety_stock_level, reorder_point, current_quantity, warehouse_id,
                unit_cost_amount, unit_cost_currency, consent_settings,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            )
            ON CONFLICT (id) DO UPDATE SET
                sku = $2, name = $3, description = $4, category = $5,
                unit_of_measure = $6, safety_stock_level = $7, reorder_point = $8,
                current_quantity = $9, warehouse_id = $10,
                unit_cost_amount = $11, unit_cost_currency = $12,
                consent_settings = $13, updated_at = $15
            "#,
            model.id,
            model.sku,
            model.name,
            model.description,
            model.category,
            model.unit_of_measure,
            model.safety_stock_level,
            model.reorder_point,
            model.current_quantity,
            model.warehouse_id,
            model.unit_cost_amount,
            model.unit_cost_currency,
            model.consent_settings as Option<sqlx::types::Json<InventoryConsentSettingsModel>>,
            model.created_at,
            model.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<InventoryItem, RepositoryError> {
        let row = sqlx::query_as!(
            InventoryItemModel,
            r#"
            SELECT id, sku, name, description, category, unit_of_measure,
                   safety_stock_level, reorder_point, current_quantity, warehouse_id,
                   unit_cost_amount, unit_cost_currency, consent_settings,
                   created_at, updated_at
            FROM inventory_items
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let model = row.ok_or_else(|| RepositoryError::InventoryItemNotFound(id))?;
        let item = self.convert_model_to_item(model)?;
        
        Ok(item)
    }
    
    async fn find_by_warehouse_id(&self, warehouse_id: Uuid) -> Result<Vec<InventoryItem>, RepositoryError> {
        let rows = sqlx::query_as!(
            InventoryItemModel,
            r#"
            SELECT id, sku, name, description, category, unit_of_measure,
                   safety_stock_level, reorder_point, current_quantity, warehouse_id,
                   unit_cost_amount, unit_cost_currency, consent_settings,
                   created_at, updated_at
            FROM inventory_items
            WHERE warehouse_id = $1
            ORDER BY name
            "#,
            warehouse_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for model in rows {
            let item = self.convert_model_to_item(model)?;
            items.push(item);
        }
        
        Ok(items)
    }
    
    async fn find_by_sku(&self, sku: &str) -> Result<InventoryItem, RepositoryError> {
        let row = sqlx::query_as!(
            InventoryItemModel,
            r#"
            SELECT id, sku, name, description, category, unit_of_measure,
                   safety_stock_level, reorder_point, current_quantity, warehouse_id,
                   unit_cost_amount, unit_cost_currency, consent_settings,
                   created_at, updated_at
            FROM inventory_items
            WHERE sku = $1
            "#,
            sku
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let model = row.ok_or_else(|| RepositoryError::InventoryItemNotFound(Uuid::nil()))?;
        let item = self.convert_model_to_item(model)?;
        
        Ok(item)
    }
    
    async fn find_items_needing_reorder(&self) -> Result<Vec<InventoryItem>, RepositoryError> {
        let rows = sqlx::query_as!(
            InventoryItemModel,
            r#"
            SELECT id, sku, name, description, category, unit_of_measure,
                   safety_stock_level, reorder_point, current_quantity, warehouse_id,
                   unit_cost_amount, unit_cost_currency, consent_settings,
                   created_at, updated_at
            FROM inventory_items
            WHERE current_quantity <= reorder_point
            ORDER BY current_quantity
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for model in rows {
            let item = self.convert_model_to_item(model)?;
            items.push(item);
        }
        
        Ok(items)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM inventory_items WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

// Procurement Order Repository
#[async_trait::async_trait]
pub trait ProcurementOrderRepository: Send + Sync {
    async fn save(&self, order: &ProcurementOrder) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<ProcurementOrder, RepositoryError>;
    async fn find_by_supplier_id(&self, supplier_id: Uuid) -> Result<Vec<ProcurementOrder>, RepositoryError>;
    async fn find_by_status(&self, status: OrderStatus) -> Result<Vec<ProcurementOrder>, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct PgProcurementOrderRepository {
    pool: PgPool,
}

impl PgProcurementOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    fn convert_order_to_model(&self, order: &ProcurementOrder) -> Result<ProcurementOrderModel, RepositoryError> {
        let status = match order.status {
            OrderStatus::Draft => "draft".to_string(),
            OrderStatus::Submitted => "submitted".to_string(),
            OrderStatus::Approved => "approved".to_string(),
            OrderStatus::Rejected => "rejected".to_string(),
            OrderStatus::Shipped => "shipped".to_string(),
            OrderStatus::Received => "received".to_string(),
            OrderStatus::Cancelled => "cancelled".to_string(),
        };
        
        let consent_settings = ProcurementConsentSettingsModel {
            share_with_supplier: match order.consent_settings.share_with_supplier {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_price_data: match order.consent_settings.share_price_data {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_delivery_schedule: match order.consent_settings.share_delivery_schedule {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            share_line_items: match order.consent_settings.share_line_items {
                DataSharingLevel::None => "none".to_string(),
                DataSharingLevel::ViewOnly => "view_only".to_string(),
                DataSharingLevel::Editable => "editable".to_string(),
                DataSharingLevel::FullAccess => "full_access".to_string(),
            },
            custom_fields: order.consent_settings.custom_fields.clone(),
        };
        
        Ok(ProcurementOrderModel {
            id: order.id,
            supplier_id: order.supplier_id,
            order_number: order.order_number.clone(),
            status,
            expected_delivery: order.expected_delivery,
            actual_delivery: order.actual_delivery,
            total_amount: order.total_amount.amount,
            total_amount_currency: match order.total_amount.currency {
                Currency::USD => "USD".to_string(),
                Currency::EUR => "EUR".to_string(),
                Currency::GBP => "GBP".to_string(),
                Currency::JPY => "JPY".to_string(),
                Currency::CAD => "CAD".to_string(),
                Currency::AUD => "AUD".to_string(),
                Currency::CHF => "CHF".to_string(),
                Currency::CNY => "CNY".to_string(),
                Currency::SEK => "SEK".to_string(),
                Currency::NZD => "NZD".to_string(),
                Currency::MXN => "MXN".to_string(),
                Currency::SGD => "SGD".to_string(),
                Currency::HKD => "HKD".to_string(),
                Currency::NOK => "NOK".to_string(),
                Currency::KRW => "KRW".to_string(),
                Currency::TRY => "TRY".to_string(),
                Currency::RUB => "RUB".to_string(),
                Currency::INR => "INR".to_string(),
                Currency::BRL => "BRL".to_string(),
                Currency::ZAR => "ZAR".to_string(),
            },
            consent_settings: Some(sqlx::types::Json(consent_settings)),
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
    }
    
    fn convert_line_item_to_model(&self, line_item: &OrderLineItem, order_id: Uuid) -> Result<OrderLineItemModel, RepositoryError> {
        Ok(OrderLineItemModel {
            id: line_item.id,
            order_id,
            inventory_item_id: line_item.inventory_item_id,
            quantity: line_item.quantity,
            unit_price_amount: line_item.unit_price.amount,
            unit_price_currency: match line_item.unit_price.currency {
                Currency::USD => "USD".to_string(),
                Currency::EUR => "EUR".to_string(),
                Currency::GBP => "GBP".to_string(),
                Currency::JPY => "JPY".to_string(),
                Currency::CAD => "CAD".to_string(),
                Currency::AUD => "AUD".to_string(),
                Currency::CHF => "CHF".to_string(),
                Currency::CNY => "CNY".to_string(),
                Currency::SEK => "SEK".to_string(),
                Currency::NZD => "NZD".to_string(),
                Currency::MXN => "MXN".to_string(),
                Currency::SGD => "SGD".to_string(),
                Currency::HKD => "HKD".to_string(),
                Currency::NOK => "NOK".to_string(),
                Currency::KRW => "KRW".to_string(),
                Currency::TRY => "TRY".to_string(),
                Currency::RUB => "RUB".to_string(),
                Currency::INR => "INR".to_string(),
                Currency::BRL => "BRL".to_string(),
                Currency::ZAR => "ZAR".to_string(),
            },
            extended_price_amount: line_item.extended_price.amount,
            extended_price_currency: match line_item.extended_price.currency {
                Currency::USD => "USD".to_string(),
                Currency::EUR => "EUR".to_string(),
                Currency::GBP => "GBP".to_string(),
                Currency::JPY => "JPY".to_string(),
                Currency::CAD => "CAD".to_string(),
                Currency::AUD => "AUD".to_string(),
                Currency::CHF => "CHF".to_string(),
                Currency::CNY => "CNY".to_string(),
                Currency::SEK => "SEK".to_string(),
                Currency::NZD => "NZD".to_string(),
                Currency::MXN => "MXN".to_string(),
                Currency::SGD => "SGD".to_string(),
                Currency::HKD => "HKD".to_string(),
                Currency::NOK => "NOK".to_string(),
                Currency::KRW => "KRW".to_string(),
                Currency::TRY => "TRY".to_string(),
                Currency::RUB => "RUB".to_string(),
                Currency::INR => "INR".to_string(),
                Currency::BRL => "BRL".to_string(),
                Currency::ZAR => "ZAR".to_string(),
            },
            description: line_item.description.clone(),
        })
    }
    
    fn convert_model_to_order(&self, order_model: ProcurementOrderModel, line_item_models: Vec<OrderLineItemModel>) -> Result<ProcurementOrder, RepositoryError> {
        let status = match order_model.status.as_str() {
            "draft" => OrderStatus::Draft,
            "submitted" => OrderStatus::Submitted,
            "approved" => OrderStatus::Approved,
            "rejected" => OrderStatus::Rejected,
            "shipped" => OrderStatus::Shipped,
            "received" => OrderStatus::Received,
            "cancelled" => OrderStatus::Cancelled,
            _ => OrderStatus::Draft,
        };
        
        let total_amount_currency = match order_model.total_amount_currency.as_str() {
            "USD" => Currency::USD,
            "EUR" => Currency::EUR,
            "GBP" => Currency::GBP,
            "JPY" => Currency::JPY,
            "CAD" => Currency::CAD,
            "AUD" => Currency::AUD,
            "CHF" => Currency::CHF,
            "CNY" => Currency::CNY,
            "SEK" => Currency::SEK,
            "NZD" => Currency::NZD,
            "MXN" => Currency::MXN,
            "SGD" => Currency::SGD,
            "HKD" => Currency::HKD,
            "NOK" => Currency::NOK,
            "KRW" => Currency::KRW,
            "TRY" => Currency::TRY,
            "RUB" => Currency::RUB,
            "INR" => Currency::INR,
            "BRL" => Currency::BRL,
            "ZAR" => Currency::ZAR,
            _ => Currency::USD,
        };
        
        let total_amount = Money {
            amount: order_model.total_amount,
            currency: total_amount_currency,
        };
        
        let consent_settings_model = order_model.consent_settings
            .ok_or_else(|| RepositoryError::ConversionError("Missing consent_settings".to_string()))?
            .0;
        
        let consent_settings = ProcurementConsentSettings {
            share_with_supplier: match consent_settings_model.share_with_supplier.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_price_data: match consent_settings_model.share_price_data.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_delivery_schedule: match consent_settings_model.share_delivery_schedule.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            share_line_items: match consent_settings_model.share_line_items.as_str() {
                "none" => DataSharingLevel::None,
                "view_only" => DataSharingLevel::ViewOnly,
                "editable" => DataSharingLevel::Editable,
                "full_access" => DataSharingLevel::FullAccess,
                _ => DataSharingLevel::None,
            },
            custom_fields: consent_settings_model.custom_fields,
        };
        
        let line_items: Vec<OrderLineItem> = line_item_models.into_iter().map(|line_item_model| {
            let unit_price_currency = match line_item_model.unit_price_currency.as_str() {
                "USD" => Currency::USD,
                "EUR" => Currency::EUR,
                "GBP" => Currency::GBP,
                "JPY" => Currency::JPY,
                "CAD" => Currency::CAD,
                "AUD" => Currency::AUD,
                "CHF" => Currency::CHF,
                "CNY" => Currency::CNY,
                "SEK" => Currency::SEK,
                "NZD" => Currency::NZD,
                "MXN" => Currency::MXN,
                "SGD" => Currency::SGD,
                "HKD" => Currency::HKD,
                "NOK" => Currency::NOK,
                "KRW" => Currency::KRW,
                "TRY" => Currency::TRY,
                "RUB" => Currency::RUB,
                "INR" => Currency::INR,
                "BRL" => Currency::BRL,
                "ZAR" => Currency::ZAR,
                _ => Currency::USD,
            };
            
            let extended_price_currency = match line_item_model.extended_price_currency.as_str() {
                "USD" => Currency::USD,
                "EUR" => Currency
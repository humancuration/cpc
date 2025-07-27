//! # CPC Supply Chain Management Module
//!
//! Supply Chain Management module for the CPC platform.
//!
//! This module provides comprehensive supply chain functionality including:
//! - Supply chain network management
//! - Inventory tracking and management
//! - Procurement order processing
//! - Shipment tracking
//! - Warehouse management
//! - Supplier relationship management
//! - Privacy-preserving data sharing
//!
//! # Architecture
//!
//! This module follows hexagonal (clean) architecture with vertical slices:
//!
//! - **Supply Chain Network Slice**: Network graph management, lead time calculations
//! - **Inventory Slice**: Stock level management, reorder logic
//! - **Procurement Slice**: Order creation, approval workflows
//! - **Shipment Slice**: Shipment tracking and management
//! - **Warehouse Slice**: Capacity management, location services
//! - **Supplier Slice**: Performance metrics, contract management
//! - **Domain Module**: Cross-cutting supply chain primitives and shared models
//!
//! # Key Features
//!
//! - Supply chain network visualization
//! - Inventory management with automatic reorder points
//! - Procurement order processing with approval workflows
//! - Shipment tracking with status updates
//! - Warehouse capacity management
//! - Supplier performance tracking
//! - Privacy-preserving data sharing with explicit user consent
//! - p2p data sharing using p2panda with Double Ratchet encryption
//! - Bevy visualization components
//! - Yew web components

// Vertical slices
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

/// Re-export commonly used types
pub use domain::{
    supply_chain_network::SupplyChainNetwork,
    inventory_item::InventoryItem,
    procurement_order::ProcurementOrder,
    shipment::Shipment,
    warehouse::Warehouse,
    supplier::Supplier,
    primitives::{Money, GeoLocation, OperatingHours},
};

#[cfg(test)]
mod tests {
    use super::*;
    use domain::primitives::{Money, Currency, GeoLocation, OperatingHours};
    use domain::supply_chain_network::{SupplyChainNetwork, NetworkNode, NodeType};
    use domain::inventory_item::{InventoryItem, InventoryCategory};
    use domain::procurement_order::{ProcurementOrder, OrderStatus, OrderLineItem};
    use domain::shipment::{Shipment, ShipmentStatus, ShipmentLineItem};
    use domain::warehouse::Warehouse;
    use domain::supplier::{Supplier, ContactInformation, SupplierMetrics, Contract};
    use rust_decimal::Decimal;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_supply_chain_network_creation() {
        let owner_id = Uuid::new_v4();
        let network = SupplyChainNetwork::new(
            owner_id,
            "Test Network".to_string(),
        );

        assert_eq!(network.owner_id, owner_id);
        assert_eq!(network.name, "Test Network");
    }

    #[test]
    fn test_inventory_item_creation() {
        let warehouse_id = Uuid::new_v4();
        let item = InventoryItem::new(
            "SKU-001".to_string(),
            "Test Item".to_string(),
            InventoryCategory::RawMaterials,
            "units".to_string(),
            100,
            50,
            warehouse_id,
        );

        assert_eq!(item.sku, "SKU-001");
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category, InventoryCategory::RawMaterials);
        assert_eq!(item.safety_stock_level, 100);
        assert_eq!(item.reorder_point, 50);
        assert_eq!(item.warehouse_id, warehouse_id);
    }

    #[test]
    fn test_procurement_order_creation() {
        let supplier_id = Uuid::new_v4();
        let order = ProcurementOrder::new(
            supplier_id,
            "PO-001".to_string(),
            Utc::now() + chrono::Duration::days(7),
        );

        assert_eq!(order.supplier_id, supplier_id);
        assert_eq!(order.order_number, "PO-001");
        assert_eq!(order.status, OrderStatus::Draft);
    }

    #[test]
    fn test_shipment_creation() {
        let origin_id = Uuid::new_v4();
        let destination_id = Uuid::new_v4();
        let shipment = Shipment::new(
            "TRK-001".to_string(),
            "Carrier Inc".to_string(),
            origin_id,
            destination_id,
            5,
        );

        assert_eq!(shipment.tracking_number, "TRK-001");
        assert_eq!(shipment.carrier, "Carrier Inc");
        assert_eq!(shipment.origin_id, origin_id);
        assert_eq!(shipment.destination_id, destination_id);
        assert_eq!(shipment.expected_transit_days, 5);
        assert_eq!(shipment.status, ShipmentStatus::Created);
    }

    #[test]
    fn test_warehouse_creation() {
        let location = GeoLocation {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        
        let operating_hours = OperatingHours {
            open_time: chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            close_time: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        };

        let warehouse = Warehouse::new(
            "Main Warehouse".to_string(),
            location,
            10000,
            operating_hours,
        );

        assert_eq!(warehouse.name, "Main Warehouse");
        assert_eq!(warehouse.location, location);
        assert_eq!(warehouse.capacity, 10000);
        assert_eq!(warehouse.operating_hours, operating_hours);
    }

    #[test]
    fn test_supplier_creation() {
        let contact_info = ContactInformation {
            email: "supplier@example.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Supplier St, City, Country".to_string(),
        };

        let metrics = SupplierMetrics {
            delivery_time_score: 0.95,
            quality_score: 0.98,
            responsiveness_score: 0.90,
        };

        let contract = Contract {
            id: Uuid::new_v4(),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(365),
            terms: "Standard terms".to_string(),
        };

        let supplier = Supplier::new(
            "Test Supplier".to_string(),
            contact_info,
            metrics,
            vec![contract],
        );

        assert_eq!(supplier.name, "Test Supplier");
        assert_eq!(supplier.contact_info.email, "supplier@example.com");
        assert_eq!(supplier.performance_metrics.delivery_time_score, 0.95);
    }
}
//! Inventory service for orchestrating inventory operations

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    inventory_item::{InventoryItem, InventoryCategory},
    warehouse::Warehouse,
    primitives::{DomainError, Result as DomainResult},
    consent::InventoryConsentSettings,
};
use crate::infrastructure::database::repositories::{
    InventoryItemRepository, WarehouseRepository,
};

/// Service error types
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },
    
    #[error("Item not found")]
    ItemNotFound,
    
    #[error("Warehouse not found")]
    WarehouseNotFound,
    
    #[error("Insufficient inventory")]
    InsufficientInventory,
    
    #[error("Capacity exceeded")]
    CapacityExceeded,
}

/// Result type for service operations
pub type Result<T> = std::result::Result<T, ServiceError>;

/// Privacy consent service trait
#[async_trait]
pub trait PrivacyConsentService: Send + Sync {
    async fn verify_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType,
    ) -> Result<()>;
}

/// Consent types for inventory operations
#[derive(Debug, Clone)]
pub enum ConsentType {
    InventoryView,
    InventoryModification,
    InventoryCreation,
    InventoryAdjustment,
}

/// Inventory service for managing inventory operations
pub struct InventoryService {
    inventory_repo: Arc<dyn InventoryItemRepository>,
    warehouse_repo: Arc<dyn WarehouseRepository>,
    privacy_service: Arc<dyn PrivacyConsentService>,
}

impl InventoryService {
    pub fn new(
        inventory_repo: Arc<dyn InventoryItemRepository>,
        warehouse_repo: Arc<dyn WarehouseRepository>,
        privacy_service: Arc<dyn PrivacyConsentService>,
    ) -> Self {
        Self {
            inventory_repo,
            warehouse_repo,
            privacy_service,
        }
    }

    /// Create a new inventory item
    pub async fn create_inventory_item(
        &self,
        user_id: Uuid,
        sku: String,
        name: String,
        category: InventoryCategory,
        unit_of_measure: String,
        safety_stock_level: i32,
        reorder_point: i32,
        warehouse_id: Uuid,
        unit_cost: Option<crate::domain::primitives::Money>,
        description: Option<String>,
        consent_settings: InventoryConsentSettings,
    ) -> Result<InventoryItem> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryCreation)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to create inventory item: {}", message),
                    }
                }
                _ => e,
            })?;

        // Verify warehouse exists
        let warehouse = self.warehouse_repo
            .find_by_id(warehouse_id)
            .await
            .map_err(|_| ServiceError::WarehouseNotFound)?;

        // Create item
        let mut item = InventoryItem::new(
            sku,
            name,
            category,
            unit_of_measure,
            safety_stock_level,
            reorder_point,
            warehouse_id,
        );
        
        // Set optional fields
        if let Some(desc) = description {
            item = item.with_description(desc);
        }
        
        if let Some(cost) = unit_cost {
            item = item.with_unit_cost(cost);
        }
        
        // Set consent settings
        item.set_consent_settings(consent_settings);
        
        // Validate item
        item.validate().map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.inventory_repo
            .save(&item)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(item)
    }

    /// Update inventory quantity
    pub async fn update_inventory_quantity(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        delta: i32,
    ) -> Result<InventoryItem> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryAdjustment)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to adjust inventory: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find item
        let mut item = self.inventory_repo
            .find_by_id(item_id)
            .await
            .map_err(|_| ServiceError::ItemNotFound)?;

        // Update quantity
        item.update_quantity(delta).map_err(|e| match e {
            DomainError::InvalidOperation { message } => {
                // This is a special case for reorder point reached
                if message == "Reorder point reached" {
                    // In a real implementation, this would trigger a reorder workflow
                    // For now, we'll just continue with the update
                    item.update_quantity(delta).map_err(ServiceError::Domain)?;
                    item
                } else {
                    return Err(ServiceError::Domain(DomainError::InvalidOperation { message }));
                }
            }
            _ => return Err(ServiceError::Domain(e)),
        })?;

        // Save to repository
        self.inventory_repo
            .save(&item)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(item)
    }

    /// Get an inventory item by ID
    pub async fn get_inventory_item(
        &self,
        user_id: Uuid,
        item_id: Uuid,
    ) -> Result<InventoryItem> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to view inventory item: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find item
        let item = self.inventory_repo
            .find_by_id(item_id)
            .await
            .map_err(|_| ServiceError::ItemNotFound)?;

        Ok(item)
    }

    /// List inventory items for a warehouse
    pub async fn list_inventory_for_warehouse(
        &self,
        user_id: Uuid,
        warehouse_id: Uuid,
    ) -> Result<Vec<InventoryItem>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to list inventory: {}", message),
                    }
                }
                _ => e,
            })?;

        // List items
        let items = self.inventory_repo
            .find_by_warehouse_id(warehouse_id)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(items)
    }

    /// Get low stock items
    pub async fn get_low_stock_items(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<InventoryItem>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to view low stock items: {}", message),
                    }
                }
                _ => e,
            })?;

        // Get items that need reordering
        let items = self.inventory_repo
            .find_items_needing_reorder()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(items)
    }

    /// Transfer inventory between warehouses
    pub async fn transfer_inventory(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        from_warehouse_id: Uuid,
        to_warehouse_id: Uuid,
        quantity: i32,
    ) -> Result<()> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::InventoryModification)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to transfer inventory: {}", message),
                    }
                }
                _ => e,
            })?;

        // Verify warehouses exist
        let from_warehouse = self.warehouse_repo
            .find_by_id(from_warehouse_id)
            .await
            .map_err(|_| ServiceError::WarehouseNotFound)?;

        let to_warehouse = self.warehouse_repo
            .find_by_id(to_warehouse_id)
            .await
            .map_err(|_| ServiceError::WarehouseNotFound)?;

        // Find item
        let mut item = self.inventory_repo
            .find_by_id(item_id)
            .await
            .map_err(|_| ServiceError::ItemNotFound)?;

        // Check if item is in the from warehouse
        if item.warehouse_id != from_warehouse_id {
            return Err(ServiceError::InvalidOperation {
                message: "Item is not in the specified from warehouse".to_string(),
            });
        }

        // Check if there's enough quantity
        if item.current_quantity < quantity {
            return Err(ServiceError::InsufficientInventory);
        }

        // Check if destination warehouse has capacity
        to_warehouse.validate_capacity(quantity)
            .map_err(|_| ServiceError::CapacityExceeded)?;

        // Update source warehouse utilization
        let mut from_warehouse = from_warehouse;
        from_warehouse.update_utilization(-quantity)
            .map_err(ServiceError::Domain)?;

        // Update destination warehouse utilization
        let mut to_warehouse = to_warehouse;
        to_warehouse.update_utilization(quantity)
            .map_err(ServiceError::Domain)?;

        // Update item warehouse
        item.warehouse_id = to_warehouse_id;

        // Update quantities
        item.update_quantity(-quantity).map_err(ServiceError::Domain)?;

        // Save changes
        self.inventory_repo
            .save(&item)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        self.warehouse_repo
            .save(&from_warehouse)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        self.warehouse_repo
            .save(&to_warehouse)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(())
    }
}
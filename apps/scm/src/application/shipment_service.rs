//! Shipment service for orchestrating shipment operations

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    shipment::{Shipment, ShipmentLineItem, ShipmentStatus},
    supply_chain_network::{SupplyChainNetwork, NodeType},
    primitives::{NodeId, DomainError, Result as DomainResult},
    consent::ShipmentConsentSettings,
};
use crate::infrastructure::database::repositories::{
    ShipmentRepository, SupplyChainNetworkRepository,
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
    
    #[error("Shipment not found")]
    ShipmentNotFound,
    
    #[error("Network not found")]
    NetworkNotFound,
    
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
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

/// Consent types for shipment operations
#[derive(Debug, Clone)]
pub enum ConsentType {
    ShipmentView,
    ShipmentCreation,
    ShipmentTracking,
    ShipmentModification,
}

/// Shipment service for managing shipment operations
pub struct ShipmentService {
    shipment_repo: Arc<dyn ShipmentRepository>,
    network_repo: Arc<dyn SupplyChainNetworkRepository>,
    privacy_service: Arc<dyn PrivacyConsentService>,
}

impl ShipmentService {
    pub fn new(
        shipment_repo: Arc<dyn ShipmentRepository>,
        network_repo: Arc<dyn SupplyChainNetworkRepository>,
        privacy_service: Arc<dyn PrivacyConsentService>,
    ) -> Self {
        Self {
            shipment_repo,
            network_repo,
            privacy_service,
        }
    }

    /// Create a new shipment
    pub async fn create_shipment(
        &self,
        user_id: Uuid,
        tracking_number: String,
        carrier: String,
        origin_id: NodeId,
        destination_id: NodeId,
        expected_transit_days: i32,
        line_items: Vec<ShipmentLineItem>,
        consent_settings: ShipmentConsentSettings,
    ) -> Result<Shipment> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentCreation)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to create shipment: {}", message),
                    }
                }
                _ => e,
            })?;

        // Validate that origin and destination exist in the network
        // This would typically be done by checking against the network repository
        // For now, we'll assume they exist

        // Create shipment
        let mut shipment = Shipment::new(
            tracking_number,
            carrier,
            origin_id,
            destination_id,
            expected_transit_days,
        );
        
        // Add line items
        for item in line_items {
            shipment.add_line_item(item)
                .map_err(ServiceError::Domain)?;
        }
        
        // Set consent settings
        shipment.set_consent_settings(consent_settings);
        
        // Validate shipment
        shipment.validate().map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.shipment_repo
            .save(&shipment)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(shipment)
    }

    /// Update shipment status
    pub async fn update_shipment_status(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
        new_status: ShipmentStatus,
    ) -> Result<Shipment> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentModification)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to update shipment status: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find shipment
        let mut shipment = self.shipment_repo
            .find_by_id(shipment_id)
            .await
            .map_err(|_| ServiceError::ShipmentNotFound)?;

        // Update status
        shipment.update_status(new_status).map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.shipment_repo
            .save(&shipment)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(shipment)
    }

    /// Mark shipment as in transit
    pub async fn mark_shipment_in_transit(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
    ) -> Result<Shipment> {
        self.update_shipment_status(user_id, shipment_id, ShipmentStatus::InTransit).await
    }

    /// Mark shipment as delivered
    pub async fn mark_shipment_delivered(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
    ) -> Result<Shipment> {
        self.update_shipment_status(user_id, shipment_id, ShipmentStatus::Delivered).await
    }

    /// Mark shipment as delayed
    pub async fn mark_shipment_delayed(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
    ) -> Result<Shipment> {
        self.update_shipment_status(user_id, shipment_id, ShipmentStatus::Delayed).await
    }

    /// Cancel shipment
    pub async fn cancel_shipment(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
    ) -> Result<Shipment> {
        self.update_shipment_status(user_id, shipment_id, ShipmentStatus::Cancelled).await
    }

    /// Get a shipment by ID
    pub async fn get_shipment(
        &self,
        user_id: Uuid,
        shipment_id: Uuid,
    ) -> Result<Shipment> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to view shipment: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find shipment
        let shipment = self.shipment_repo
            .find_by_id(shipment_id)
            .await
            .map_err(|_| ServiceError::ShipmentNotFound)?;

        Ok(shipment)
    }

    /// List shipments by status
    pub async fn list_shipments_by_status(
        &self,
        user_id: Uuid,
        status: ShipmentStatus,
    ) -> Result<Vec<Shipment>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to list shipments: {}", message),
                    }
                }
                _ => e,
            })?;

        // List shipments
        let shipments = self.shipment_repo
            .find_by_status(status)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(shipments)
    }

    /// List shipments by carrier
    pub async fn list_shipments_by_carrier(
        &self,
        user_id: Uuid,
        carrier: String,
    ) -> Result<Vec<Shipment>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to list shipments: {}", message),
                    }
                }
                _ => e,
            })?;

        // List shipments
        let shipments = self.shipment_repo
            .find_by_carrier(&carrier)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(shipments)
    }

    /// Track shipment by tracking number
    pub async fn track_shipment(
        &self,
        user_id: Uuid,
        tracking_number: String,
    ) -> Result<Shipment> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentTracking)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to track shipment: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find shipment
        let shipment = self.shipment_repo
            .find_by_tracking_number(&tracking_number)
            .await
            .map_err(|_| ServiceError::ShipmentNotFound)?;

        Ok(shipment)
    }

    /// Get delayed shipments
    pub async fn get_delayed_shipments(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Shipment>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ShipmentView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to view delayed shipments: {}", message),
                    }
                }
                _ => e,
            })?;

        // Get all shipments and filter for delayed ones
        let all_shipments = self.shipment_repo
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        let delayed_shipments: Vec<Shipment> = all_shipments
            .into_iter()
            .filter(|s| s.is_delayed())
            .collect();

        Ok(delayed_shipments)
    }
}
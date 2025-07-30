//! Procurement service for orchestrating procurement order operations

use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::{
    procurement_order::{ProcurementOrder, OrderLineItem, OrderStatus},
    supplier::Supplier,
    primitives::{Money, DomainError, Result as DomainResult},
    consent::ProcurementConsentSettings,
};
use crate::infrastructure::database::repositories::{
    ProcurementOrderRepository, SupplierRepository,
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
    
    #[error("Budget insufficient: available {0}, required {1}")]
    BudgetInsufficient(Money, Money),
    
    #[error("Supplier not found")]
    SupplierNotFound,
    
    #[error("Order not found")]
    OrderNotFound,
}

/// Result type for service operations
pub type Result<T> = std::result::Result<T, ServiceError>;

/// Privacy consent service trait
#[async_trait]
pub trait PrivacyConsentService: Send + Sync {
    async fn verify_consent(
        &self,
        user_id: uuid::Uuid,
        consent_type: ConsentType,
    ) -> Result<()>;
}

/// Consent types for SCM operations
#[derive(Debug, Clone)]
pub enum ConsentType {
    ProcurementCreation,
    ProcurementApproval,
    ProcurementView,
    ProcurementModification,
}

/// Finance service trait for budget verification
#[async_trait]
pub trait FinanceService: Send + Sync {
    async fn verify_budget_availability(
        &self,
        user_id: uuid::Uuid,
        category: BudgetCategory,
        amount: Money,
    ) -> Result<BudgetCheckResult>;
}

/// Budget category for procurement
#[derive(Debug, Clone)]
pub enum BudgetCategory {
    Procurement,
    RawMaterials,
    Equipment,
    Services,
}

/// Result of budget check
#[derive(Debug, Clone)]
pub struct BudgetCheckResult {
    pub is_approved: bool,
    pub available_amount: Money,
    pub requested_amount: Money,
}

/// Procurement service for managing procurement orders
pub struct ProcurementService {
    order_repo: Arc<dyn ProcurementOrderRepository>,
    supplier_repo: Arc<dyn SupplierRepository>,
    privacy_service: Arc<dyn PrivacyConsentService>,
    finance_service: Arc<dyn FinanceService>,
}

impl ProcurementService {
    pub fn new(
        order_repo: Arc<dyn ProcurementOrderRepository>,
        supplier_repo: Arc<dyn SupplierRepository>,
        privacy_service: Arc<dyn PrivacyConsentService>,
        finance_service: Arc<dyn FinanceService>,
    ) -> Self {
        Self {
            order_repo,
            supplier_repo,
            privacy_service,
            finance_service,
        }
    }

    /// Create a new procurement order
    pub async fn create_procurement_order(
        &self,
        user_id: uuid::Uuid,
        supplier_id: uuid::Uuid,
        order_number: String,
        expected_delivery: chrono::DateTime<chrono::Utc>,
        line_items: Vec<OrderLineItem>,
        consent_settings: ProcurementConsentSettings,
    ) -> Result<ProcurementOrder> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementCreation)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to create procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Verify supplier exists
        let supplier = self.supplier_repo
            .find_by_id(supplier_id)
            .await
            .map_err(|_| ServiceError::SupplierNotFound)?;

        // Create order
        let mut order = ProcurementOrder::new(
            supplier_id,
            order_number,
            expected_delivery,
        );
        
        // Add line items
        for item in line_items {
            order.add_line_item(item)
                .map_err(ServiceError::Domain)?;
        }
        
        // Set consent settings
        order.set_consent_settings(consent_settings);
        
        // Validate order
        order.validate().map_err(ServiceError::Domain)?;
        
        // Verify budget availability through Finance module
        let total_amount = order.calculate_total();
        let budget_check = self.finance_service
            .verify_budget_availability(
                user_id,
                BudgetCategory::Procurement,
                total_amount,
            )
            .await?;
        
        if !budget_check.is_approved {
            return Err(ServiceError::BudgetInsufficient(
                budget_check.available_amount,
                total_amount,
            ));
        }
        
        // Save to repository
        self.order_repo
            .save(&order)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(order)
    }

    /// Submit a procurement order
    pub async fn submit_procurement_order(
        &self,
        user_id: uuid::Uuid,
        order_id: uuid::Uuid,
    ) -> Result<ProcurementOrder> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementModification)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to submit procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find order
        let mut order = self.order_repo
            .find_by_id(order_id)
            .await
            .map_err(|_| ServiceError::OrderNotFound)?;

        // Submit order
        order.submit().map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.order_repo
            .save(&order)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(order)
    }

    /// Approve a procurement order
    pub async fn approve_procurement_order(
        &self,
        user_id: uuid::Uuid,
        order_id: uuid::Uuid,
    ) -> Result<ProcurementOrder> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementApproval)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to approve procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find order
        let mut order = self.order_repo
            .find_by_id(order_id)
            .await
            .map_err(|_| ServiceError::OrderNotFound)?;

        // Verify budget availability through Finance module
        let total_amount = order.calculate_total();
        let budget_check = self.finance_service
            .verify_budget_availability(
                user_id,
                BudgetCategory::Procurement,
                total_amount,
            )
            .await?;
        
        if !budget_check.is_approved {
            return Err(ServiceError::BudgetInsufficient(
                budget_check.available_amount,
                total_amount,
            ));
        }

        // Approve order
        order.approve().map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.order_repo
            .save(&order)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(order)
    }

    /// Reject a procurement order
    pub async fn reject_procurement_order(
        &self,
        user_id: uuid::Uuid,
        order_id: uuid::Uuid,
    ) -> Result<ProcurementOrder> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementApproval)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to reject procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find order
        let mut order = self.order_repo
            .find_by_id(order_id)
            .await
            .map_err(|_| ServiceError::OrderNotFound)?;

        // Reject order
        order.reject().map_err(ServiceError::Domain)?;
        
        // Save to repository
        self.order_repo
            .save(&order)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;
        
        Ok(order)
    }

    /// Get a procurement order by ID
    pub async fn get_procurement_order(
        &self,
        user_id: uuid::Uuid,
        order_id: uuid::Uuid,
    ) -> Result<ProcurementOrder> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to view procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find order
        let order = self.order_repo
            .find_by_id(order_id)
            .await
            .map_err(|_| ServiceError::OrderNotFound)?;

        Ok(order)
    }

    /// List procurement orders for a supplier
    pub async fn list_procurement_orders_for_supplier(
        &self,
        user_id: uuid::Uuid,
        supplier_id: uuid::Uuid,
    ) -> Result<Vec<ProcurementOrder>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to list procurement orders: {}", message),
                    }
                }
                _ => e,
            })?;

        // List orders
        let orders = self.order_repo
            .find_by_supplier_id(supplier_id)
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        Ok(orders)
    }

    /// Share order with supplier based on consent settings
    pub async fn share_order_with_supplier(
        &self,
        user_id: uuid::Uuid,
        order_id: uuid::Uuid,
        supplier_id: uuid::Uuid,
    ) -> Result<()> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::ProcurementView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to share procurement order: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find order
        let order = self.order_repo
            .find_by_id(order_id)
            .await
            .map_err(|_| ServiceError::OrderNotFound)?;

        // Verify sharing consent
        if !order.can_share_with_supplier() {
            return Err(ServiceError::PermissionDenied {
                message: "Sharing not permitted by current consent settings".to_string(),
            });
        }

        // In a real implementation, this would share via p2p
        // For now, we'll just return Ok to indicate the check passed
        Ok(())
    }
}
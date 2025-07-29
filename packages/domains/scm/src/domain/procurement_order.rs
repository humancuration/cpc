//! Procurement order entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::primitives::{Money, DomainError, Result};
use super::consent::ProcurementConsentSettings;

/// Status of a procurement order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Shipped,
    Received,
    Cancelled,
}

/// Line item in a procurement order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderLineItem {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub quantity: i32,
    pub unit_price: Money,
    pub extended_price: Money,
    pub description: Option<String>,
}

impl OrderLineItem {
    pub fn new(
        inventory_item_id: Uuid,
        quantity: i32,
        unit_price: Money,
        description: Option<String>,
    ) -> Result<Self> {
        // Validate quantity
        if quantity <= 0 {
            return Err(DomainError::ValidationError {
                message: "Quantity must be positive".to_string(),
            });
        }
        
        // Validate that unit price and extended price use the same currency
        let extended_price = unit_price.multiply(rust_decimal::Decimal::from(quantity));
        
        Ok(Self {
            id: Uuid::new_v4(),
            inventory_item_id,
            quantity,
            unit_price,
            extended_price,
            description,
        })
    }
}

/// Procurement order entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcurementOrder {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub expected_delivery: DateTime<Utc>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub line_items: Vec<OrderLineItem>,
    pub total_amount: Money,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: ProcurementConsentSettings,
}

impl ProcurementOrder {
    /// Create a new procurement order
    pub fn new(
        supplier_id: Uuid,
        order_number: String,
        expected_delivery: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            supplier_id,
            order_number,
            status: OrderStatus::Draft,
            expected_delivery,
            actual_delivery: None,
            line_items: Vec::new(),
            total_amount: Money::zero(super::primitives::Currency::USD), // Default currency
            created_at: now,
            updated_at: now,
            consent_settings: ProcurementConsentSettings::default(),
        }
    }

    /// Add a line item to the order
    pub fn add_line_item(&mut self, line_item: OrderLineItem) -> Result<()> {
        // Validate that we're not adding line items to a submitted or later status order
        match self.status {
            OrderStatus::Submitted | OrderStatus::Approved | OrderStatus::Shipped | 
            OrderStatus::Received | OrderStatus::Cancelled => {
                return Err(DomainError::InvalidOperation {
                    message: "Cannot add line items to orders that are submitted or later in the workflow".to_string(),
                });
            }
            _ => {}
        }
        
        self.line_items.push(line_item);
        self.recalculate_total();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a line item from the order
    pub fn remove_line_item(&mut self, line_item_id: Uuid) -> Result<()> {
        // Validate that we're not removing line items from a submitted or later status order
        match self.status {
            OrderStatus::Submitted | OrderStatus::Approved | OrderStatus::Shipped | 
            OrderStatus::Received | OrderStatus::Cancelled => {
                return Err(DomainError::InvalidOperation {
                    message: "Cannot remove line items from orders that are submitted or later in the workflow".to_string(),
                });
            }
            _ => {}
        }
        
        let initial_count = self.line_items.len();
        self.line_items.retain(|item| item.id != line_item_id);
        
        if self.line_items.len() == initial_count {
            return Err(DomainError::NotFound);
        }
        
        self.recalculate_total();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Recalculate the total amount of the order
    fn recalculate_total(&mut self) {
        let mut total = Money::zero(self.total_amount.currency);
        for item in &self.line_items {
            // Try to add the extended price to the total
            if let Ok(new_total) = total.add(&item.extended_price) {
                total = new_total;
            }
            // If currencies don't match, we keep the original total currency
            // In a real implementation, we might want to handle currency conversion
        }
        self.total_amount = total;
    }

    /// Submit the order
    pub fn submit(&mut self) -> Result<()> {
        // Validate that the order has line items
        if self.line_items.is_empty() {
            return Err(DomainError::ValidationError {
                message: "Cannot submit an order with no line items".to_string(),
            });
        }
        
        // Validate that all required fields are set
        if self.supplier_id.is_nil() {
            return Err(DomainError::ValidationError {
                message: "Supplier ID must be set before submitting".to_string(),
            });
        }
        
        // Validate status transition
        match self.status {
            OrderStatus::Draft => {
                self.status = OrderStatus::Submitted;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Approve the order
    pub fn approve(&mut self) -> Result<()> {
        match self.status {
            OrderStatus::Submitted => {
                self.status = OrderStatus::Approved;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Reject the order
    pub fn reject(&mut self) -> Result<()> {
        match self.status {
            OrderStatus::Submitted => {
                self.status = OrderStatus::Rejected;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Mark the order as shipped
    pub fn mark_shipped(&mut self) -> Result<()> {
        match self.status {
            OrderStatus::Approved => {
                self.status = OrderStatus::Shipped;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Mark the order as received
    pub fn mark_received(&mut self, actual_delivery: DateTime<Utc>) -> Result<()> {
        match self.status {
            OrderStatus::Shipped => {
                self.status = OrderStatus::Received;
                self.actual_delivery = Some(actual_delivery);
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Cancel the order
    pub fn cancel(&mut self) -> Result<()> {
        match self.status {
            OrderStatus::Draft | OrderStatus::Submitted => {
                self.status = OrderStatus::Cancelled;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }

    /// Set consent settings for the procurement order
    pub fn set_consent_settings(&mut self, settings: ProcurementConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Validate the procurement order
    pub fn validate(&self) -> Result<()> {
        // Delivery dates must be in the future when created
        if self.expected_delivery <= self.created_at {
            return Err(DomainError::ValidationError {
                message: "Expected delivery date must be in the future".to_string(),
            });
        }
        
        // If actual delivery is set, it must be after created_at
        if let Some(actual_delivery) = self.actual_delivery {
            if actual_delivery <= self.created_at {
                return Err(DomainError::ValidationError {
                    message: "Actual delivery date must be after order creation".to_string(),
                });
            }
            
            // Actual delivery must be before or equal to expected delivery
            if actual_delivery > self.expected_delivery {
                // This is a warning condition, not an error, as late deliveries are possible
                // We'll allow it but note it in a real implementation
            }
        }
        
        // Validate line items
        for item in &self.line_items {
            // Quantity must be positive
            if item.quantity <= 0 {
                return Err(DomainError::ValidationError {
                    message: "Line item quantity must be positive".to_string(),
                });
            }
            
            // Unit price and extended price must use the same currency
            if item.unit_price.currency != item.extended_price.currency {
                return Err(DomainError::CurrencyMismatch);
            }
            
            // Extended price should equal unit price * quantity
            let calculated_extended = item.unit_price.multiply(rust_decimal::Decimal::from(item.quantity));
            if calculated_extended.amount != item.extended_price.amount {
                return Err(DomainError::ValidationError {
                    message: "Extended price does not match unit price * quantity".to_string(),
                });
            }
        }
        
        Ok(())
    }

    /// Check if the order can be shared with supplier
    pub fn can_share_with_supplier(&self) -> bool {
        matches!(
            self.consent_settings.share_with_supplier,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if price data can be shared
    pub fn can_share_price_data(&self) -> bool {
        matches!(
            self.consent_settings.share_price_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if delivery schedule can be shared
    pub fn can_share_delivery_schedule(&self) -> bool {
        matches!(
            self.consent_settings.share_delivery_schedule,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Calculate total amount of the order
    pub fn calculate_total(&self) -> Money {
        let mut total = Money::zero(self.total_amount.currency);
        for item in &self.line_items {
            if let Ok(new_total) = total.add(&item.extended_price) {
                total = new_total;
            }
        }
        total
    }
}
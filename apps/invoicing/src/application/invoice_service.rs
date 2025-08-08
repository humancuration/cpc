//! Invoice application service
//!
//! This module contains the application service for managing invoices, including payment processing.

use crate::domain::{Invoice, PaymentStatus, InvoiceItem};
use crate::domain::payment::{PaymentProcessor, PaymentData, PaymentResult, PaymentProvider};
use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use thiserror::Error;
use std::sync::Arc;
use common_utils::financial::MonetaryValue;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(Uuid),
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Invoice not found: {0}")]
    NotFound(Uuid),
}

#[async_trait]
pub trait InvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError>;
    async fn update_status(&self, id: Uuid, status: PaymentStatus) -> Result<Invoice, RepositoryError>;
    async fn update(&self, invoice: &Invoice) -> Result<Invoice, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, RepositoryError>;
}

// Placeholder for P2P manager
pub struct P2PManager;

impl P2PManager {
    pub async fn share_invoice(&self, _invoice: &Invoice) -> Result<(), ServiceError> {
        // Implementation would go here
        Ok(())
    }
    
    pub async fn notify_client(&self, _invoice: &Invoice) -> Result<(), ServiceError> {
        // Implementation would go here
        Ok(())
    }
}

pub struct CreateInvoiceInput {
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<InvoiceItem>,
    pub total_amount: Decimal,
    pub due_date: DateTime<Utc>,
}

impl CreateInvoiceInput {
    /// Calculate the total amount for the invoice items using high-precision fixed-point arithmetic
    pub fn calculate_total(&self) -> Decimal {
        let mut total_fixed = fixed::types::I64F64::from_num(0.0);
        
        for item in &self.items {
            let quantity_fixed = fixed::types::I64F64::from_num(item.quantity as f64);
            let unit_price_fixed = fixed::types::I64F64::from_num(item.unit_price.to_f64().unwrap_or(0.0));
            let item_total_fixed = quantity_fixed * unit_price_fixed;
            total_fixed = total_fixed + item_total_fixed;
        }
        
        // Round to 2 decimal places using banker's rounding for currency
        let monetary_value = MonetaryValue::new(total_fixed, "USD");
        let rounded_value = monetary_value.round(2, common_utils::financial::RoundingStrategy::Bankers);
        Decimal::from_f64(rounded_value.value().to_num::<f64>()).unwrap_or(Decimal::ZERO)
    }
}

impl Invoice {
    pub fn new(input: CreateInvoiceInput) -> Result<Self, ServiceError> {
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            client_id: input.client_id,
            client_name: input.client_name,
            client_email: input.client_email,
            items: input.items,
            total_amount: input.total_amount,
            due_date: input.due_date,
            status: PaymentStatus::Draft,
            created_at: now,
            updated_at: now,
        })
    }
}
}

pub struct InvoiceService {
    repo: Arc<dyn InvoiceRepository>,
    p2p_manager: Arc<P2PManager>,
    payment_processor: Arc<dyn PaymentProcessor>,
    calendar_integration: Arc<dyn CalendarEventRegistrar>,
}

impl InvoiceService {
    pub fn new(
        repo: Arc<dyn InvoiceRepository>,
        p2p_manager: Arc<P2PManager>,
        payment_processor: Arc<dyn PaymentProcessor>,
    ) -> Self {
        Self { repo, p2p_manager, payment_processor }
    }

    pub async fn create_invoice(&self, mut input: CreateInvoiceInput) -> Result<Invoice, ServiceError> {
        // If total_amount is zero, calculate it from the items
        if input.total_amount.is_zero() {
            input.total_amount = input.calculate_total();
        }
        
        // Domain validation occurs here
        let invoice = Invoice::new(input)?;
        let invoice = self.repo.create(invoice).await
            .map_err(|e| ServiceError::RepositoryError(e.to_string()))?;
        self.p2p_manager.share_invoice(&invoice).await?;
        Ok(invoice)
    }

    pub async fn send_invoice(&self, id: Uuid) -> Result<Invoice, ServiceError> {
        let mut invoice = self.repo.find_by_id(id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(_) => ServiceError::InvoiceNotFound(id),
                _ => ServiceError::RepositoryError(e.to_string()),
            })?;
        invoice.status = PaymentStatus::Sent;
        invoice.updated_at = Utc::now();
        let updated = self.repo.update_status(invoice.id, invoice.status).await
            .map_err(|e| ServiceError::RepositoryError(e.to_string()))?;
        self.p2p_manager.notify_client(&updated).await?;
        Ok(updated)
    }

    /// Process payment for an invoice
    pub async fn process_payment(&self, id: Uuid, payment_data: PaymentData) -> Result<Invoice, ServiceError> {
        let mut invoice = self.repo.find_by_id(id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(_) => ServiceError::InvoiceNotFound(id),
                _ => ServiceError::RepositoryError(e.to_string()),
            })?;

        // Process payment through selected provider
        let payment_result = self.payment_processor.process_payment(&invoice, payment_data).await
            .map_err(|e| ServiceError::RepositoryError(e.to_string()))?;

        // Update invoice status based on payment result
        match payment_result {
            PaymentResult::Success(provider, intent_id) => {
                invoice.update_payment_info(provider, intent_id);
                invoice.update_status(PaymentStatus::Paid);
            }
            PaymentResult::Pending => {
                invoice.update_status(PaymentStatus::Pending);
            }
            PaymentResult::Failed => {
                invoice.update_status(PaymentStatus::PaymentFailed);
            }
        }

        let updated = self.repo.update(&invoice).await
            .map_err(|e| ServiceError::RepositoryError(e.to_string()))?;
        self.p2p_manager.notify_client(&updated).await?;
        Ok(updated)
    }

    /// Get payment status from provider
    pub async fn get_payment_status(&self, id: Uuid) -> Result<PaymentStatus, ServiceError> {
        let invoice = self.repo.find_by_id(id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(_) => ServiceError::InvoiceNotFound(id),
                _ => ServiceError::RepositoryError(e.to_string()),
            })?;

        if let (Some(provider), Some(intent_id)) = (&invoice.payment_provider, &invoice.payment_intent_id) {
            let status = self.payment_processor.get_payment_status(*provider, intent_id).await
                .map_err(|e| ServiceError::RepositoryError(e.to_string()))?;
            Ok(status)
        } else {
            Err(ServiceError::InvalidInput("Invoice has no payment information".to_string()))
        }
    }
}
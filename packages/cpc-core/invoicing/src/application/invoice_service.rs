//! Invoice application service

use crate::domain::{Invoice, PaymentStatus, InvoiceItem};
use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use thiserror::Error;
use std::sync::Arc;

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

pub struct InvoiceService {
    repo: Arc<dyn InvoiceRepository>,
    p2p_manager: Arc<P2PManager>,
}

impl InvoiceService {
    pub fn new(repo: Arc<dyn InvoiceRepository>, p2p_manager: Arc<P2PManager>) -> Self {
        Self { repo, p2p_manager }
    }

    pub async fn create_invoice(&self, input: CreateInvoiceInput) -> Result<Invoice, ServiceError> {
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
}
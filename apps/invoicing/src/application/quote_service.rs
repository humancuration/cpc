//! Quote application service

use crate::domain::{Quote, QuoteStatus, QuoteItem, Invoice, InvoiceItem, PaymentStatus};
use crate::application::InvoiceService;
use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use thiserror::Error;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum QuoteServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Quote not found: {0}")]
    QuoteNotFound(Uuid),
    #[error("Service error: {0}")]
    ServiceError(String),
}

#[async_trait]
pub trait QuoteRepository {
    async fn create(&self, quote: Quote) -> Result<Quote, QuoteServiceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Quote, QuoteServiceError>;
    async fn accept(&self, id: Uuid) -> Result<Invoice, QuoteServiceError>;
}

pub struct CreateQuoteInput {
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<QuoteItem>,
    pub total_amount: Decimal,
    pub validity_period: Duration,
}

impl Quote {
    pub fn new(input: CreateQuoteInput) -> Result<Self, QuoteServiceError> {
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            client_id: input.client_id,
            client_name: input.client_name,
            client_email: input.client_email,
            items: input.items,
            total_amount: input.total_amount,
            validity_period: input.validity_period,
            status: QuoteStatus::Draft,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub fn convert_to_invoice(self) -> CreateInvoiceInput {
        CreateInvoiceInput {
            client_id: self.client_id,
            client_name: self.client_name,
            client_email: self.client_email,
            items: self.items.into_iter().map(|item| InvoiceItem {
                description: item.description,
                quantity: item.quantity,
                unit_price: item.unit_price,
            }).collect(),
            total_amount: self.total_amount,
            due_date: Utc::now() + chrono::Duration::days(30), // Default 30 days
        }
    }
}

pub struct QuoteService {
    repo: Arc<dyn QuoteRepository>,
    invoice_service: Arc<InvoiceService>,
}

impl QuoteService {
    pub fn new(repo: Arc<dyn QuoteRepository>, invoice_service: Arc<InvoiceService>) -> Self {
        Self { repo, invoice_service }
    }

    pub async fn create_quote(&self, input: CreateQuoteInput) -> Result<Quote, QuoteServiceError> {
        let quote = Quote::new(input)?;
        let quote = self.repo.create(quote).await?;
        Ok(quote)
    }

    pub async fn accept_quote(&self, id: Uuid) -> Result<Invoice, QuoteServiceError> {
        let quote = self.repo.find_by_id(id).await?;
        let invoice_input = quote.convert_to_invoice();
        self.invoice_service.create_invoice(invoice_input).await
            .map_err(|e| QuoteServiceError::ServiceError(e.to_string()))
    }
}
//! Payment domain models for the invoicing module
//!
//! This module contains the core business entities for payment processing.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

/// Payment provider types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentProvider {
    Stripe,
    PayPal,
    Manual,
}

/// Payment result from processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentResult {
    Success(PaymentProvider, String), // (provider, intent_id)
    Pending,
    Failed,
}

/// Payment data for processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentData {
    pub provider: PaymentProvider,
    pub token: String,
    // Additional provider-specific data can be added here
}

/// Payment processor trait
#[async_trait::async_trait]
pub trait PaymentProcessor: Send + Sync {
    async fn process_payment(&self, invoice: &Invoice, payment_data: PaymentData) -> Result<PaymentResult, PaymentError>;
    async fn get_payment_status(&self, provider: PaymentProvider, intent_id: &str) -> Result<PaymentStatus, PaymentError>;
}

/// Error types for payment operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum PaymentError {
    #[error("Invalid payment data: {0}")]
    InvalidPaymentData(String),
    #[error("Payment provider error: {0}")]
    ProviderError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Payment declined: {0}")]
    PaymentDeclined(String),
}

/// Invoice with payment information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<InvoiceItem>,
    pub total_amount: Decimal,
    pub due_date: DateTime<Utc>,
    pub status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_provider: Option<PaymentProvider>,
    pub payment_intent_id: Option<String>,
    pub next_reminder_date: Option<DateTime<Utc>>,
}

/// Invoice item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Decimal,
}

/// Payment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Partial,
    PaymentFailed,
    Pending,
}

impl Invoice {
    /// Create a new invoice
    pub fn new(
        client_id: Uuid,
        client_name: String,
        client_email: String,
        items: Vec<InvoiceItem>,
        total_amount: Decimal,
        due_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            client_id,
            client_name,
            client_email,
            items,
            total_amount,
            due_date,
            status: PaymentStatus::Draft,
            created_at: now,
            updated_at: now,
            payment_provider: None,
            payment_intent_id: None,
            next_reminder_date: None,
        }
    }

    /// Update payment information
    pub fn update_payment_info(&mut self, provider: PaymentProvider, intent_id: String) {
        self.payment_provider = Some(provider);
        self.payment_intent_id = Some(intent_id);
        self.updated_at = Utc::now();
    }

    /// Update payment status
    pub fn update_status(&mut self, status: PaymentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
        
        // If the invoice is paid, clear the next reminder date
        if status == PaymentStatus::Paid {
            self.next_reminder_date = None;
        }
    }

    /// Set next reminder date
    pub fn set_next_reminder_date(&mut self, date: DateTime<Utc>) {
        self.next_reminder_date = Some(date);
        self.updated_at = Utc::now();
    }
}
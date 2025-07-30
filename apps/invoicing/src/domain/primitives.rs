//! Domain primitives for invoicing and quoting

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<QuoteItem>,
    pub total_amount: Decimal,
    pub validity_period: chrono::Duration,
    pub status: QuoteStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuoteStatus {
    Draft,
    Sent,
    Accepted,
    Rejected,
    Expired,
}
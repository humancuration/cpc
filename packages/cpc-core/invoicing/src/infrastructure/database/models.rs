//! Database models for invoicing and quoting

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceRecord {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: serde_json::Value, // JSON representation of Vec<InvoiceItem>
    pub total_amount: Decimal,
    pub due_date: DateTime<Utc>,
    pub status: String, // String representation of PaymentStatus
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRecord {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: serde_json::Value, // JSON representation of Vec<QuoteItem>
    pub total_amount: Decimal,
    pub validity_period_days: i64, // Days representation of Duration
    pub status: String, // String representation of QuoteStatus
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
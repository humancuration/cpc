use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceItem {
    pub id: Option<String>,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: Option<f64>,
}

impl InvoiceItem {
    pub fn calculate_total(&self) -> f64 {
        self.quantity * self.unit_price
    }
}

impl Default for InvoiceItem {
    fn default() -> Self {
        Self {
            id: None,
            description: String::new(),
            quantity: 1.0,
            unit_price: 0.0,
            total: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvoiceStatus::Draft => write!(f, "Draft"),
            InvoiceStatus::Sent => write!(f, "Sent"),
            InvoiceStatus::Paid => write!(f, "Paid"),
            InvoiceStatus::Overdue => write!(f, "Overdue"),
            InvoiceStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    pub id: Option<String>,
    pub recipient_id: String,
    pub recipient_name: Option<String>,
    pub recipient_email: Option<String>,
    pub items: Vec<InvoiceItem>,
    pub due_date: DateTime<Utc>,
    pub template_id: Option<String>,
    pub notes: Option<String>,
    pub tax_rate: f64,
    pub discount: f64,
    pub status: InvoiceStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Invoice {
    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.calculate_total()).sum()
    }

    pub fn tax_amount(&self) -> f64 {
        self.subtotal() * (self.tax_rate / 100.0)
    }

    pub fn total(&self) -> f64 {
        self.subtotal() + self.tax_amount() - self.discount
    }

    pub fn is_valid(&self) -> bool {
        !self.recipient_id.is_empty()
            && !self.items.is_empty()
            && self.items.iter().all(|item| !item.description.is_empty() && item.quantity > 0.0 && item.unit_price >= 0.0)
    }
}

impl Default for Invoice {
    fn default() -> Self {
        let thirty_days_from_now = Utc::now() + chrono::Duration::days(30);
        
        Self {
            id: None,
            recipient_id: String::new(),
            recipient_name: None,
            recipient_email: None,
            items: vec![InvoiceItem::default()],
            due_date: thirty_days_from_now,
            template_id: None,
            notes: None,
            tax_rate: 0.0,
            discount: 0.0,
            status: InvoiceStatus::Draft,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub default_layout: String,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}
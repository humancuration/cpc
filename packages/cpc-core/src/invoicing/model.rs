use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::accounting::money::Money;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub number: String,
    pub customer_id: Uuid,
    pub customer: Customer,
    pub line_items: Vec<LineItem>,
    pub status: InvoiceStatus,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub paid_date: Option<DateTime<Utc>>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sync_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
    pub tax_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: Uuid,
    pub description: String,
    pub quantity: f64,
    pub unit_price: Money,
    pub total: Money,
    pub tax_rate: Option<f64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceFilter {
    pub customer_id: Option<Uuid>,
    pub status: Option<InvoiceStatus>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub search_term: Option<String>,
}

impl Invoice {
    pub fn new(customer: Customer) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        Self {
            id,
            number: Self::generate_invoice_number(),
            customer_id: customer.id,
            customer,
            line_items: Vec::new(),
            status: InvoiceStatus::Draft,
            issue_date: now,
            due_date: now + chrono::Duration::days(30),
            paid_date: None,
            subtotal: Money::zero(),
            tax_amount: Money::zero(),
            total: Money::zero(),
            notes: None,
            terms: None,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            sync_version: 0,
        }
    }

    pub fn add_line_item(&mut self, description: String, quantity: f64, unit_price: Money, tax_rate: Option<f64>) {
        let total = unit_price * quantity;
        let line_item = LineItem {
            id: Uuid::new_v4(),
            description,
            quantity,
            unit_price,
            total,
            tax_rate,
            metadata: HashMap::new(),
        };
        
        self.line_items.push(line_item);
        self.recalculate_totals();
    }

    pub fn remove_line_item(&mut self, item_id: Uuid) {
        self.line_items.retain(|item| item.id != item_id);
        self.recalculate_totals();
    }

    pub fn recalculate_totals(&mut self) {
        self.subtotal = self.line_items.iter()
            .map(|item| item.total)
            .fold(Money::zero(), |acc, total| acc + total);
        
        self.tax_amount = self.line_items.iter()
            .filter_map(|item| {
                item.tax_rate.map(|rate| item.total * rate / 100.0)
            })
            .fold(Money::zero(), |acc, tax| acc + tax);
        
        self.total = self.subtotal + self.tax_amount;
        self.updated_at = Utc::now();
    }

    pub fn mark_as_sent(&mut self) {
        self.status = InvoiceStatus::Sent;
        self.updated_at = Utc::now();
    }

    pub fn mark_as_paid(&mut self, paid_date: DateTime<Utc>) {
        self.status = InvoiceStatus::Paid;
        self.paid_date = Some(paid_date);
        self.updated_at = Utc::now();
    }

    pub fn mark_as_overdue(&mut self) {
        self.status = InvoiceStatus::Overdue;
        self.updated_at = Utc::now();
    }

    fn generate_invoice_number() -> String {
        use chrono::Datelike;
        let now = Utc::now();
        format!("INV-{:04}{:02}{:02}-{:06}", 
            now.year(), 
            now.month(), 
            now.day(), 
            rand::random::<u32>() % 1000000
        )
    }
}

impl Customer {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email: None,
            phone: None,
            address: None,
            tax_id: None,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn with_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn with_tax_id(mut self, tax_id: String) -> Self {
        self.tax_id = Some(tax_id);
        self
    }
}

impl LineItem {
    pub fn new(description: String, quantity: f64, unit_price: Money) -> Self {
        let total = unit_price * quantity;
        Self {
            id: Uuid::new_v4(),
            description,
            quantity,
            unit_price,
            total,
            tax_rate: None,
            metadata: HashMap::new(),
        }
    }

    pub fn update_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
        self.total = self.unit_price * quantity;
    }

    pub fn update_unit_price(&mut self, unit_price: Money) {
        self.unit_price = unit_price;
        self.total = unit_price * self.quantity;
    }
}
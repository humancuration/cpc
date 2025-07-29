use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::{FinancialCategory, Money};

/// Expense entity representing a financial transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expense {
    pub id: Uuid,
    pub amount: Money,
    pub category: FinancialCategory,
    pub date: DateTime<Utc>,
    pub description: String,
    pub receipt_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Expense {
    pub fn new(
        amount: Money,
        category: FinancialCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            amount,
            category,
            date,
            description,
            receipt_id: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }
    
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|x| x == tag) {
            self.tags.remove(pos);
            self.updated_at = Utc::now();
        }
    }
    
    pub fn set_receipt(&mut self, receipt_id: Uuid) {
        self.receipt_id = Some(receipt_id);
        self.updated_at = Utc::now();
    }
    
    pub fn update_amount(&mut self, new_amount: Money) {
        self.amount = new_amount;
        self.updated_at = Utc::now();
    }
    
    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Utc::now();
    }
}
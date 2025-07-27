//! Expense tracking domain models

use chrono::{DateTime, Utc};
use uuid::Uuid;
use cpc_core::finance::{Money, Currency};
use serde::{Deserialize, Serialize};

/// Expense model for tracking purchases and payments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Money,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub receipt_id: Option<Uuid>,
    pub payment_method: String,
    pub merchant: Option<String>,
    pub tags: Vec<String>,
}

impl Expense {
    pub fn new(
        user_id: Uuid,
        amount: Money,
        description: String,
        payment_method: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            category: "Uncategorized".to_string(),
            description,
            date: Utc::now(),
            receipt_id: None,
            payment_method,
            merchant: None,
            tags: Vec::new(),
        }
    }
}

/// Receipt model for expense documentation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Receipt {
    pub id: Uuid,
    pub expense_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
}

impl Receipt {
    pub fn new(
        expense_id: Uuid,
        file_name: String,
        file_path: String,
        mime_type: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            expense_id,
            file_name,
            file_path,
            mime_type,
            uploaded_at: Utc::now(),
        }
    }
}

/// Common categorization for expenses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub name: String,
    pub keywords: Vec<String>,
    pub parent_category: Option<String>,
}

impl ExpenseCategory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            keywords: Vec::new(),
            parent_category: None,
        }
    }
}
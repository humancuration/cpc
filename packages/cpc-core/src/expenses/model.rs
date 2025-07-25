use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String, // ISO 4217 currency code
    pub description: String,
    pub category: ExpenseCategory,
    pub status: ExpenseStatus,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub receipts: Vec<Receipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub expense_id: Uuid,
    pub file_name: String,
    pub file_path: String, // Path in our storage system
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ExpenseStatus {
    Pending,
    Approved,
    Rejected,
    Reimbursed,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ExpenseCategory {
    Travel,
    Meals,
    Software,
    Hardware,
    OfficeSupplies,
    Other(String), // Custom category
}
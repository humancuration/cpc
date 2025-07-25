use async_trait::async_trait;
use uuid::Uuid;
use crate::expenses::model::{Expense, ExpenseStatus, Receipt, ExpenseCategory};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// DTOs for service layer
pub struct CreateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub category: ExpenseCategory,
    pub transaction_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub category: ExpenseCategory,
    pub transaction_date: DateTime<Utc>,
}

#[async_trait]
pub trait ExpenseService: Send + Sync {
    async fn create_expense(&self, user_id: Uuid, input: CreateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error>;
    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error>;
    async fn update_expense(&self, expense_id: Uuid, input: UpdateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn update_expense_status(&self, expense_id: Uuid, status: ExpenseStatus) -> Result<Expense, anyhow::Error>;
    async fn attach_receipt(&self, expense_id: Uuid, file_data: Vec<u8>, file_name: String, mime_type: String) -> Result<Receipt, anyhow::Error>;
}
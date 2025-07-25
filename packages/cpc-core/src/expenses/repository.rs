use async_trait::async_trait;
use uuid::Uuid;
use crate::expenses::model::{Expense, ExpenseStatus, Receipt};
use crate::expenses::service::{CreateExpenseInput, UpdateExpenseInput};

#[async_trait]
pub trait ExpenseRepository: Send + Sync {
    async fn create_expense(&self, user_id: Uuid, input: &CreateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error>;
    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error>;
    async fn update_expense(&self, expense_id: Uuid, input: &UpdateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn update_expense_status(&self, expense_id: Uuid, status: &ExpenseStatus) -> Result<Expense, anyhow::Error>;
    async fn create_receipt(&self, expense_id: Uuid, file_name: &str, file_path: &str, mime_type: &str) -> Result<Receipt, anyhow::Error>;
}
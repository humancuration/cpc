use async_trait::async_trait;
use cpc_core::expenses::{
    model::{Expense, ExpenseStatus, Receipt},
    repository::ExpenseRepository,
    service::{CreateExpenseInput, ExpenseService, UpdateExpenseInput},
};
use uuid::Uuid;
use std::sync::Arc;

pub struct ExpenseServiceImpl {
    expense_repo: Arc<dyn ExpenseRepository>,
}

impl ExpenseServiceImpl {
    pub fn new(expense_repo: Arc<dyn ExpenseRepository>) -> Self {
        Self { expense_repo }
    }
}

#[async_trait]
impl ExpenseService for ExpenseServiceImpl {
    async fn create_expense(&self, user_id: Uuid, input: CreateExpenseInput) -> Result<Expense, anyhow::Error> {
        self.expense_repo.create_expense(user_id, &input).await
    }

    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error> {
        self.expense_repo.get_expense_by_id(expense_id).await
    }

    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error> {
        self.expense_repo.get_expenses_for_user(user_id).await
    }

    async fn update_expense(&self, expense_id: Uuid, input: UpdateExpenseInput) -> Result<Expense, anyhow::Error> {
        self.expense_repo.update_expense(expense_id, &input).await
    }

    async fn update_expense_status(&self, expense_id: Uuid, status: ExpenseStatus) -> Result<Expense, anyhow::Error> {
        self.expense_repo.update_expense_status(expense_id, &status).await
    }
    
    async fn attach_receipt(
        &self,
        expense_id: Uuid,
        _file_data: Vec<u8>, // a real implementation would store this
        file_name: String,
        mime_type: String,
    ) -> Result<Receipt, anyhow::Error> {
        // For now, file_path is just the file_name.
        // A real implementation would store the file and get a path.
        let file_path = file_name.clone();
        self.expense_repo.create_receipt(expense_id, &file_name, &file_path, &mime_type).await
    }
}
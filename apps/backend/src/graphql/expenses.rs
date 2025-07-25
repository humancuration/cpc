// apps/backend/src/graphql/expenses.rs
use async_graphql::*;
use cpc_core::expenses::{
    model::{self, ExpenseCategory},
    service::ExpenseService,
};
use futures::Stream;
use async_stream::stream;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use std::str::FromStr;
use rust_decimal::prelude::FromPrimitive;

// GraphQL Object for Receipt
#[derive(SimpleObject, Clone)]
#[graphql(name = "Receipt")]
pub struct ReceiptObject {
    pub id: Uuid,
    pub expense_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}

impl From<model::Receipt> for ReceiptObject {
    fn from(receipt: model::Receipt) -> Self {
        Self {
            id: receipt.id,
            expense_id: receipt.expense_id,
            file_name: receipt.file_name,
            file_path: receipt.file_path,
            mime_type: receipt.mime_type,
            uploaded_at: receipt.uploaded_at,
        }
    }
}

// GraphQL Object for Expense
#[derive(SimpleObject, Clone)]
#[graphql(name = "Expense")]
pub struct ExpenseObject {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub receipts: Vec<ReceiptObject>,
}

impl From<model::Expense> for ExpenseObject {
    fn from(expense: model::Expense) -> Self {
        Self {
            id: expense.id,
            user_id: expense.user_id,
            project_id: expense.project_id,
            client_id: expense.client_id,
            category: expense.category.to_string(),
            description: expense.description,
            amount: expense.amount.to_f64().unwrap_or(0.0),
            currency: expense.currency,
            status: expense.status.to_string(),
            transaction_date: expense.transaction_date,
            created_at: expense.created_at,
            updated_at: expense.updated_at,
            receipts: expense.receipts.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(InputObject)]
pub struct CreateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub category: String,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
}

impl From<CreateExpenseInput> for cpc_core::expenses::service::CreateExpenseInput {
    fn from(input: CreateExpenseInput) -> Self {
        Self {
            project_id: input.project_id,
            client_id: input.client_id,
            amount: Decimal::from_f64(input.amount).unwrap_or_default(),
            currency: input.currency,
            description: input.description,
            category: ExpenseCategory::from_str(&input.category).unwrap_or(ExpenseCategory::Other("".to_string())),
            transaction_date: input.transaction_date,
        }
    }
}

#[derive(InputObject)]
pub struct UpdateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub category: String,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
}

impl From<UpdateExpenseInput> for cpc_core::expenses::service::UpdateExpenseInput {
    fn from(input: UpdateExpenseInput) -> Self {
        Self {
            project_id: input.project_id,
            client_id: input.client_id,
            amount: Decimal::from_f64(input.amount).unwrap_or_default(),
            currency: input.currency,
            description: input.description,
            category: ExpenseCategory::from_str(&input.category).unwrap_or(ExpenseCategory::Other("".to_string())),
            transaction_date: input.transaction_date,
        }
    }
}

// Query Root for Expenses
#[derive(Default)]
pub struct ExpensesQueryRoot;

#[Object]
impl ExpensesQueryRoot {
    /// Get a single expense by ID
    #[graphql(name = "expense")]
    async fn expense(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<ExpenseObject>> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let expense = service.get_expense_by_id(id).await
            .map_err(|e| Error::new(format!("Failed to get expense: {}", e)))?;
        
        Ok(expense.map(Into::into))
    }

    /// Get all expenses for a user
    #[graphql(name = "expenses")]
    async fn expenses(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<ExpenseObject>> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let expenses = service.get_expenses_for_user(user_id).await
            .map_err(|e| Error::new(format!("Failed to get expenses: {}", e)))?;
        
        Ok(expenses.into_iter().map(Into::into).collect())
    }

    /*
    /// Get expenses for a project
    #[graphql(name = "projectExpenses")]
    async fn project_expenses(&self, ctx: &Context<'_>, project_id: Uuid) -> Result<Vec<ExpenseObject>> {
        let service = ctx.data_unchecked::<ExpenseService>();
        let expenses = service.get_expenses_by_project(project_id).await
            .map_err(|e| Error::new(format!("Failed to get project expenses: {}", e)))?;
        
        Ok(expenses.into_iter().map(Into::into).collect())
    }
    */
}

// Mutation Root for Expenses
#[derive(Default)]
pub struct ExpensesMutationRoot;

#[Object]
impl ExpensesMutationRoot {
    /// Create a new expense
    #[graphql(name = "createExpense")]
    async fn create_expense(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
        input: CreateExpenseInput,
    ) -> Result<ExpenseObject> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        
        let created_expense = service.create_expense(user_id, input.into()).await
            .map_err(|e| Error::new(format!("Failed to create expense: {}", e)))?;
        
        Ok(created_expense.into())
    }

    /// Update an existing expense
    #[graphql(name = "updateExpense")]
    async fn update_expense(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateExpenseInput,
    ) -> Result<ExpenseObject> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        
        let updated_expense = service.update_expense(id, input.into()).await
            .map_err(|e| Error::new(format!("Failed to update expense: {}", e)))?;
        
        Ok(updated_expense.into())
    }

    /*
    /// Delete an expense
    #[graphql(name = "deleteExpense")]
    async fn delete_expense(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<ExpenseService>();
        
        service.delete_expense(id).await
            .map_err(|e| Error::new(format!("Failed to delete expense: {}", e)))?;
        
        Ok(true)
    }
    */
}

// Subscription Root for Expenses (placeholder for future real-time updates)
#[derive(Default)]
pub struct ExpensesSubscriptionRoot;

#[Subscription]
impl ExpensesSubscriptionRoot {
    /// Placeholder for real-time expense updates
    #[graphql(name = "expenseUpdates")]
    async fn expense_updates(&self, _user_id: Uuid) -> impl Stream<Item = Arc<ExpenseObject>> {
        // For now, return an empty stream
        stream! {
            // Placeholder - will implement real-time updates later
        }
    }
}
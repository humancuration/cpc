// apps/backend/src/graphql/expenses.rs
use async_graphql::*;
use cpc_core::expenses::{model, service::ExpenseService};
use futures::Stream;
use async_stream::stream;
use std::sync::Arc;

// GraphQL Object for Expense
#[derive(SimpleObject, Clone)]
#[graphql(name = "Expense")]
pub struct ExpenseObject {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Vec<MetadataEntry>,
    pub sync_version: u64,
}

#[derive(SimpleObject, Clone)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}

impl From<model::Expense> for ExpenseObject {
    fn from(expense: model::Expense) -> Self {
        Self {
            id: expense.id,
            user_id: expense.user_id,
            project_id: expense.project_id,
            category: expense.category,
            description: expense.description,
            amount: expense.amount,
            currency: expense.currency,
            transaction_date: expense.transaction_date,
            created_at: expense.created_at,
            updated_at: expense.updated_at,
            metadata: expense.metadata.into_iter()
                .map(|(k, v)| MetadataEntry { key: k, value: v })
                .collect(),
            sync_version: expense.sync_version,
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
        let service = ctx.data_unchecked::<ExpenseService>();
        let expense = service.get_expense(id).await
            .map_err(|e| Error::new(format!("Failed to get expense: {}", e)))?;
        
        Ok(expense.map(Into::into))
    }

    /// Get all expenses for a user
    #[graphql(name = "expenses")]
    async fn expenses(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<ExpenseObject>> {
        let service = ctx.data_unchecked::<ExpenseService>();
        let expenses = service.get_expenses_by_user(user_id).await
            .map_err(|e| Error::new(format!("Failed to get expenses: {}", e)))?;
        
        Ok(expenses.into_iter().map(Into::into).collect())
    }

    /// Get expenses for a project
    #[graphql(name = "projectExpenses")]
    async fn project_expenses(&self, ctx: &Context<'_>, project_id: Uuid) -> Result<Vec<ExpenseObject>> {
        let service = ctx.data_unchecked::<ExpenseService>();
        let expenses = service.get_expenses_by_project(project_id).await
            .map_err(|e| Error::new(format!("Failed to get project expenses: {}", e)))?;
        
        Ok(expenses.into_iter().map(Into::into).collect())
    }
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
        input: model::ExpenseInput,
    ) -> Result<ExpenseObject> {
        let service = ctx.data_unchecked::<ExpenseService>();
        
        let mut expense = model::Expense::new(
            user_id,
            input.category,
            input.description,
            input.amount,
            input.currency,
        );
        
        if let Some(project_id) = input.project_id {
            expense = expense.with_project_id(project_id);
        }
        
        expense = expense.with_transaction_date(input.transaction_date);
        
        let created_expense = service.create_expense(expense).await
            .map_err(|e| Error::new(format!("Failed to create expense: {}", e)))?;
        
        Ok(created_expense.into())
    }

    /// Update an existing expense
    #[graphql(name = "updateExpense")]
    async fn update_expense(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: model::ExpenseInput,
    ) -> Result<ExpenseObject> {
        let service = ctx.data_unchecked::<ExpenseService>();
        
        let expense = service.get_expense(id).await
            .map_err(|e| Error::new(format!("Failed to get expense: {}", e)))?;
        
        let mut expense = expense.ok_or_else(|| Error::new("Expense not found"))?;
        
        expense.update_category(input.category);
        expense.update_description(input.description);
        expense.update_amount(input.amount);
        
        if let Some(project_id) = input.project_id {
            expense.project_id = Some(project_id);
        }
        
        expense.update_transaction_date(input.transaction_date);
        
        let updated_expense = service.update_expense(expense).await
            .map_err(|e| Error::new(format!("Failed to update expense: {}", e)))?;
        
        Ok(updated_expense.into())
    }

    /// Delete an expense
    #[graphql(name = "deleteExpense")]
    async fn delete_expense(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<ExpenseService>();
        
        service.delete_expense(id).await
            .map_err(|e| Error::new(format!("Failed to delete expense: {}", e)))?;
        
        Ok(true)
    }
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
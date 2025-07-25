use anyhow::Context;
use async_trait::async_trait;
use cpc_core::expenses::{
    model::{Expense, ExpenseCategory, ExpenseStatus, Receipt},
    repository::ExpenseRepository,
    service::{CreateExpenseInput, UpdateExpenseInput},
};
use sqlx::{types::Decimal, PgPool, FromRow};
use std::str::FromStr;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExpenseRepositoryImpl {
    pool: PgPool,
}

impl ExpenseRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// Helper struct to map database row to our model, handling enum mapping
#[derive(FromRow, Debug)]
struct ExpenseDb {
    id: Uuid,
    user_id: Uuid,
    project_id: Option<Uuid>,
    client_id: Option<Uuid>,
    amount: bigdecimal::BigDecimal,
    currency: String,
    description: String,
    category: String,
    status: String,
    transaction_date: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<ExpenseDb> for Expense {
    type Error = anyhow::Error;

    fn try_from(db_expense: ExpenseDb) -> Result<Self, Self::Error> {
        let category = ExpenseCategory::from_str(&db_expense.category)
            .map_err(|_| anyhow::anyhow!("Invalid category: {}", db_expense.category))?;
        let status = ExpenseStatus::from_str(&db_expense.status)
            .map_err(|_| anyhow::anyhow!("Invalid status: {}", db_expense.status))?;
            

        Ok(Expense {
            id: db_expense.id,
            user_id: db_expense.user_id,
            project_id: db_expense.project_id,
            client_id: db_expense.client_id,
            amount: rust_decimal::Decimal::from_str(&db_expense.amount.to_string())?,
            currency: db_expense.currency,
            description: db_expense.description,
            category,
            status,
            transaction_date: db_expense.transaction_date,
            created_at: db_expense.created_at,
            updated_at: db_expense.updated_at,
            receipts: Vec::new(), // Receipts will be loaded separately
        })
    }
}

impl FromStr for ExpenseCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Travel" => Ok(ExpenseCategory::Travel),
            "Meals" => Ok(ExpenseCategory::Meals),
            "Software" => Ok(ExpenseCategory::Software),
            "Hardware" => Ok(ExpenseCategory::Hardware),
            "OfficeSupplies" => Ok(ExpenseCategory::OfficeSupplies),
            _ => Ok(ExpenseCategory::Other(s.to_string())),
        }
    }
}

impl ToString for ExpenseCategory {
    fn to_string(&self) -> String {
        match self {
            ExpenseCategory::Travel => "Travel".to_string(),
            ExpenseCategory::Meals => "Meals".to_string(),
            ExpenseCategory::Software => "Software".to_string(),
            ExpenseCategory::Hardware => "Hardware".to_string(),
            ExpenseCategory::OfficeSupplies => "OfficeSupplies".to_string(),
            ExpenseCategory::Other(s) => s.clone(),
        }
    }
}

impl FromStr for ExpenseStatus {
    type Err = ();
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(ExpenseStatus::Pending),
            "Approved" => Ok(ExpenseStatus::Approved),
            "Rejected" => Ok(ExpenseStatus::Rejected),
            "Reimbursed" => Ok(ExpenseStatus::Reimbursed),
            _ => Err(()),
        }
    }
}

impl ToString for ExpenseStatus {
    fn to_string(&self) -> String {
        match self {
            ExpenseStatus::Pending => "Pending".to_string(),
            ExpenseStatus::Approved => "Approved".to_string(),
            ExpenseStatus::Rejected => "Rejected".to_string(),
            ExpenseStatus::Reimbursed => "Reimbursed".to_string(),
        }
    }
}


#[async_trait]
impl ExpenseRepository for ExpenseRepositoryImpl {
    #[instrument(skip(self, input))]
    async fn create_expense(&self, user_id: Uuid, input: &CreateExpenseInput) -> anyhow::Result<Expense> {
        let expense_db: ExpenseDb = sqlx::query_as!(
            ExpenseDb,
            r#"
            INSERT INTO expenses (id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at
            "#,
            Uuid::new_v4(),
            user_id,
            input.project_id,
            input.client_id,
            input.amount,
            input.currency,
            input.description,
            input.category.to_string(),
            "Pending".to_string(), // Initial status
            input.transaction_date
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create expense in database")?;

        expense_db.try_into()
    }

    #[instrument(skip(self))]
    async fn get_expense_by_id(&self, expense_id: Uuid) -> anyhow::Result<Option<Expense>> {
        let expense_db_opt = sqlx::query_as!(
            ExpenseDb,
            r#"SELECT id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at FROM expenses WHERE id = $1"#,
            expense_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query expense by ID")?;

        if let Some(expense_db) = expense_db_opt {
            let mut expense: Expense = expense_db.try_into()?;
            expense.receipts = self.get_receipts_for_expense(expense.id).await?;
            Ok(Some(expense))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip(self))]
    async fn get_expenses_for_user(&self, user_id: Uuid) -> anyhow::Result<Vec<Expense>> {
        let expenses_db = sqlx::query_as!(
            ExpenseDb,
            r#"SELECT id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at FROM expenses WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query expenses for user")?;

        let mut expenses = Vec::new();
        for expense_db in expenses_db {
            let mut expense: Expense = expense_db.try_into()?;
            expense.receipts = self.get_receipts_for_expense(expense.id).await?;
            expenses.push(expense);
        }

        Ok(expenses)
    }

    #[instrument(skip(self, input))]
    async fn update_expense(&self, expense_id: Uuid, input: &UpdateExpenseInput) -> anyhow::Result<Expense> {
        let amount_decimal = Decimal::from(input.amount);
        let expense_db = sqlx::query_as!(
            ExpenseDb,
            r#"
            UPDATE expenses
            SET
                project_id = $1,
                client_id = $2,
                amount = $3,
                currency = $4,
                description = $5,
                category = $6,
                transaction_date = $7,
                updated_at = NOW()
            WHERE id = $8
            RETURNING id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at
            "#,
            input.project_id,
            input.client_id,
            amount_decimal,
            input.currency,
            input.description,
            input.category.to_string(),
            input.transaction_date,
            expense_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update expense in database")?;

        let mut expense: Expense = expense_db.try_into()?;
        expense.receipts = self.get_receipts_for_expense(expense.id).await?;
        Ok(expense)
    }

    #[instrument(skip(self))]
    async fn update_expense_status(&self, expense_id: Uuid, status: &ExpenseStatus) -> anyhow::Result<Expense> {
        let expense_db = sqlx::query_as!(
            ExpenseDb,
            r#"
            UPDATE expenses
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at
            "#,
            status.to_string(),
            expense_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update expense status in database")?;

        let mut expense: Expense = expense_db.try_into()?;
        expense.receipts = self.get_receipts_for_expense(expense.id).await?;
        Ok(expense)
    }

    #[instrument(skip(self))]
    async fn create_receipt(&self, expense_id: Uuid, file_name: &str, file_path: &str, mime_type: &str) -> anyhow::Result<Receipt> {
        let receipt = sqlx::query_as!(
            Receipt,
            r#"
            INSERT INTO receipts (id, expense_id, file_name, file_path, mime_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, expense_id, file_name, file_path, mime_type, uploaded_at
            "#,
            Uuid::new_v4(),
            expense_id,
            file_name,
            file_path,
            mime_type
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create receipt in database")?;

        Ok(receipt)
    }
}

impl ExpenseRepositoryImpl {
    #[instrument(skip(self))]
    async fn get_receipts_for_expense(&self, expense_id: Uuid) -> anyhow::Result<Vec<Receipt>> {
        sqlx::query_as!(
            Receipt,
            "SELECT id, expense_id, file_name, file_path, mime_type, uploaded_at FROM receipts WHERE expense_id = $1",
            expense_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query receipts for expense")
    }
}
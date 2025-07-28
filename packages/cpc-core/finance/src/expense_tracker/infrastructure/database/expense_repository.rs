//! PostgreSQL implementation of ExpenseRepository

use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::{
    domain::{
        expense_tracker::{Expense, ExpenseCategory, ExpenseStatus},
        primitives::{Money, Currency},
        FinanceError,
    },
    expense_tracker::application::expense_service::ExpenseRepository,
};

/// Database model for expenses table
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ExpenseDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: sqlx::types::Decimal,
    pub currency: String,
    pub dabloons_amount: sqlx::types::Decimal,
    pub category: String,
    pub custom_category: Option<String>,
    pub date: DateTime<Utc>,
    pub description: String,
    pub status: String,
    pub receipt_id: Option<Uuid>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,
    pub linked_budget_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ExpenseDbModel {
    /// Convert domain Expense to database model
    pub fn from_domain(expense: &Expense) -> Self {
        let (amount, currency, dabloons_amount) = match &expense.amount.currency {
            Currency::Dabloons => (sqlx::types::Decimal::ZERO, "USD".to_string(), expense.amount.amount), // Default to USD for traditional
            _ => (expense.amount.amount, expense.amount.currency.code().to_string(), sqlx::types::Decimal::ZERO),
        };
        
        let (category, custom_category) = match &expense.category {
            ExpenseCategory::Other(name) => ("Other".to_string(), Some(name.clone())),
            _ => (format!("{:?}", expense.category), None),
        };
        
        Self {
            id: expense.id,
            user_id: expense.user_id,
            amount,
            currency,
            dabloons_amount,
            category,
            custom_category,
            date: expense.date,
            description: expense.description.clone(),
            status: format!("{:?}", expense.status),
            receipt_id: expense.receipt_id,
            is_recurring: expense.is_recurring,
            recurrence_pattern: expense.recurrence_pattern.clone(),
            linked_budget_id: expense.linked_budget_id,
            created_at: expense.created_at,
            updated_at: expense.updated_at,
        }
    }
    
    /// Convert database model to domain Expense
    pub fn to_domain(&self) -> Expense {
        let currency = match self.currency.as_str() {
            "DABLOONS" => Currency::Dabloons,
            code => {
                // Try to parse as known currency, default to USD if not found
                match code {
                    "USD" => Currency::USD,
                    "EUR" => Currency::EUR,
                    "GBP" => Currency::GBP,
                    "JPY" => Currency::JPY,
                    "CAD" => Currency::CAD,
                    "AUD" => Currency::AUD,
                    "CHF" => Currency::CHF,
                    "CNY" => Currency::CNY,
                    "SEK" => Currency::SEK,
                    "NZD" => Currency::NZD,
                    "MXN" => Currency::MXN,
                    "SGD" => Currency::SGD,
                    "HKD" => Currency::HKD,
                    "NOK" => Currency::NOK,
                    "KRW" => Currency::KRW,
                    "TRY" => Currency::TRY,
                    "RUB" => Currency::RUB,
                    "INR" => Currency::INR,
                    "BRL" => Currency::BRL,
                    "ZAR" => Currency::ZAR,
                    _ => Currency::USD,
                }
            }
        };
        
        let amount = if currency == Currency::Dabloons {
            Money::new(self.dabloons_amount, currency)
        } else {
            Money::new(self.amount, currency)
        };
        
        let category = if self.category == "Other" {
            if let Some(ref custom) = self.custom_category {
                ExpenseCategory::Other(custom.clone())
            } else {
                ExpenseCategory::Other("Unknown".to_string())
            }
        } else {
            match self.category.as_str() {
                "Food" => ExpenseCategory::Food,
                "Transportation" => ExpenseCategory::Transportation,
                "Housing" => ExpenseCategory::Housing,
                "Utilities" => ExpenseCategory::Utilities,
                "Entertainment" => ExpenseCategory::Entertainment,
                "Healthcare" => ExpenseCategory::Healthcare,
                "Education" => ExpenseCategory::Education,
                "PersonalCare" => ExpenseCategory::PersonalCare,
                "Shopping" => ExpenseCategory::Shopping,
                "Travel" => ExpenseCategory::Travel,
                "Business" => ExpenseCategory::Business,
                _ => ExpenseCategory::Other(self.category.clone()),
            }
        };
        
        let status = match self.status.as_str() {
            "Draft" => ExpenseStatus::Draft,
            "Processed" => ExpenseStatus::Processed,
            "Verified" => ExpenseStatus::Verified,
            "Rejected" => ExpenseStatus::Rejected,
            "Archived" => ExpenseStatus::Archived,
            _ => ExpenseStatus::Processed,
        };
        
        Expense {
            id: self.id,
            user_id: self.user_id,
            amount,
            category,
            date: self.date,
            description: self.description.clone(),
            status,
            receipt_id: self.receipt_id,
            is_recurring: self.is_recurring,
            recurrence_pattern: self.recurrence_pattern.clone(),
            linked_budget_id: self.linked_budget_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// PostgreSQL implementation of ExpenseRepository
pub struct PostgresExpenseRepository {
    pool: PgPool,
}

impl PostgresExpenseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExpenseRepository for PostgresExpenseRepository {
    async fn save(&self, expense: &Expense) -> Result<(), FinanceError> {
        let expense_db_model = ExpenseDbModel::from_domain(expense);
        
        sqlx::query!(
            r#"
            INSERT INTO expenses (id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (id) DO UPDATE SET
                amount = EXCLUDED.amount,
                currency = EXCLUDED.currency,
                dabloons_amount = EXCLUDED.dabloons_amount,
                category = EXCLUDED.category,
                custom_category = EXCLUDED.custom_category,
                date = EXCLUDED.date,
                description = EXCLUDED.description,
                status = EXCLUDED.status,
                receipt_id = EXCLUDED.receipt_id,
                is_recurring = EXCLUDED.is_recurring,
                recurrence_pattern = EXCLUDED.recurrence_pattern,
                linked_budget_id = EXCLUDED.linked_budget_id,
                updated_at = EXCLUDED.updated_at
            "#,
            expense_db_model.id,
            expense_db_model.user_id,
            expense_db_model.amount,
            expense_db_model.currency,
            expense_db_model.dabloons_amount,
            expense_db_model.category,
            expense_db_model.custom_category,
            expense_db_model.date,
            expense_db_model.description,
            expense_db_model.status,
            expense_db_model.receipt_id,
            expense_db_model.is_recurring,
            expense_db_model.recurrence_pattern,
            expense_db_model.linked_budget_id,
            expense_db_model.created_at,
            expense_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError> {
        let expense_record = sqlx::query_as!(
            ExpenseDbModel,
            r#"
            SELECT id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at
            FROM expenses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let expense = expense_record.map(|record| record.to_domain());
        
        Ok(expense)
    }
    
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError> {
        let expense_records = if let (Some(start), Some(end)) = (start_date, end_date) {
            sqlx::query_as!(
                ExpenseDbModel,
                r#"
                SELECT id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at
                FROM expenses
                WHERE user_id = $1 AND date >= $2 AND date <= $3
                ORDER BY date DESC
                "#,
                user_id,
                start,
                end
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| FinanceError::DatabaseError(e.to_string()))?
        } else if let Some(start) = start_date {
            sqlx::query_as!(
                ExpenseDbModel,
                r#"
                SELECT id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at
                FROM expenses
                WHERE user_id = $1 AND date >= $2
                ORDER BY date DESC
                "#,
                user_id,
                start
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| FinanceError::DatabaseError(e.to_string()))?
        } else if let Some(end) = end_date {
            sqlx::query_as!(
                ExpenseDbModel,
                r#"
                SELECT id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at
                FROM expenses
                WHERE user_id = $1 AND date <= $2
                ORDER BY date DESC
                "#,
                user_id,
                end
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| FinanceError::DatabaseError(e.to_string()))?
        } else {
            sqlx::query_as!(
                ExpenseDbModel,
                r#"
                SELECT id, user_id, amount, currency, dabloons_amount, category, custom_category, date, description, status, receipt_id, is_recurring, recurrence_pattern, linked_budget_id, created_at, updated_at
                FROM expenses
                WHERE user_id = $1
                ORDER BY date DESC
                "#,
                user_id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| FinanceError::DatabaseError(e.to_string()))?
        };
        
        let expenses: Vec<Expense> = expense_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();
        
        Ok(expenses)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            DELETE FROM expenses
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn update(&self, expense: &Expense) -> Result<(), FinanceError> {
        self.save(expense).await
    }
}
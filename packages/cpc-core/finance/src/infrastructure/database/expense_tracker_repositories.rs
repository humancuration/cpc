//! Database repositories for the expense tracker module
//!
//! This file contains the PostgreSQL implementations of the repository traits
//! defined in the expense tracker application layer.

use uuid::Uuid;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::{
    domain::{
        expense_tracker::{Expense, Receipt, ExpenseSharingPreferences},
        primitives::{Money, Currency},
        FinanceError,
    },
    expense_tracker::application::expense_service::{
        ExpenseRepository, 
        ReceiptRepository, 
        ExpenseSharingPreferenceRepository
    },
    expense_tracker::infrastructure::database::{
        expense_repository::ExpenseDbModel,
        receipt_repository::ReceiptDbModel,
        sharing_preference_repository::ExpenseSharingPreferenceDbModel,
    },
};

/// PostgreSQL implementation of ExpenseRepository
pub struct PostgresExpenseRepository {
    pool: PgPool,
}

impl PostgresExpenseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
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

/// PostgreSQL implementation of ReceiptRepository
pub struct PostgresReceiptRepository {
    pool: PgPool,
}

impl PostgresReceiptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ReceiptRepository for PostgresReceiptRepository {
    async fn save(&self, receipt: &Receipt) -> Result<(), FinanceError> {
        let receipt_db_model = ReceiptDbModel::from_domain(receipt);
        
        sqlx::query!(
            r#"
            INSERT INTO receipts (id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id) DO UPDATE SET
                image_data = EXCLUDED.image_data,
                image_format = EXCLUDED.image_format,
                extracted_text = EXCLUDED.extracted_text,
                merchant_name = EXCLUDED.merchant_name,
                transaction_date = EXCLUDED.transaction_date,
                total_amount = EXCLUDED.total_amount,
                currency = EXCLUDED.currency,
                dabloons_amount = EXCLUDED.dabloons_amount,
                processing_status = EXCLUDED.processing_status,
                processing_error = EXCLUDED.processing_error,
                updated_at = EXCLUDED.updated_at
            "#,
            receipt_db_model.id,
            receipt_db_model.user_id,
            receipt_db_model.image_data,
            receipt_db_model.image_format,
            receipt_db_model.extracted_text,
            receipt_db_model.merchant_name,
            receipt_db_model.transaction_date,
            receipt_db_model.total_amount,
            receipt_db_model.currency,
            receipt_db_model.dabloons_amount,
            receipt_db_model.processing_status,
            receipt_db_model.processing_error,
            receipt_db_model.created_at,
            receipt_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Receipt>, FinanceError> {
        let receipt_record = sqlx::query_as!(
            ReceiptDbModel,
            r#"
            SELECT id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at
            FROM receipts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let receipt = receipt_record.map(|record| record.to_domain());
        
        Ok(receipt)
    }
    
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Receipt>, FinanceError> {
        let receipt_records = sqlx::query_as!(
            ReceiptDbModel,
            r#"
            SELECT id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at
            FROM receipts
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let receipts: Vec<Receipt> = receipt_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();
        
        Ok(receipts)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            DELETE FROM receipts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn update(&self, receipt: &Receipt) -> Result<(), FinanceError> {
        self.save(receipt).await
    }
}

/// PostgreSQL implementation of ExpenseSharingPreferenceRepository
pub struct PostgresExpenseSharingPreferenceRepository {
    pool: PgPool,
}

impl PostgresExpenseSharingPreferenceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ExpenseSharingPreferenceRepository for PostgresExpenseSharingPreferenceRepository {
    async fn save(&self, preference: &ExpenseSharingPreferences) -> Result<(), FinanceError> {
        let preference_db_model = ExpenseSharingPreferenceDbModel::from_domain(preference);
        
        sqlx::query!(
            r#"
            INSERT INTO expense_sharing_preferences (id, user_id, sharing_enabled, anonymized, shared_categories, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id) DO UPDATE SET
                sharing_enabled = EXCLUDED.sharing_enabled,
                anonymized = EXCLUDED.anonymized,
                shared_categories = EXCLUDED.shared_categories,
                updated_at = EXCLUDED.updated_at
            "#,
            preference_db_model.id,
            preference_db_model.user_id,
            preference_db_model.sharing_enabled,
            preference_db_model.anonymized,
            preference_db_model.shared_categories,
            preference_db_model.created_at,
            preference_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<ExpenseSharingPreferences>, FinanceError> {
        let preference_record = sqlx::query_as!(
            ExpenseSharingPreferenceDbModel,
            r#"
            SELECT id, user_id, sharing_enabled, anonymized, shared_categories, created_at, updated_at
            FROM expense_sharing_preferences
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let preference = preference_record.map(|record| record.to_domain());
        
        Ok(preference)
    }
    
    async fn create_default(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError> {
        let preference = ExpenseSharingPreferences::new(user_id);
        self.save(&preference).await?;
        Ok(preference)
    }
}
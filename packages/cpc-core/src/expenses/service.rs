use crate::expenses::model::{Expense, ExpenseFilter, ExpenseSummary};
use crate::expenses::repository::ExpenseRepository;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ExpenseService {
    repository: Arc<dyn ExpenseRepository>,
}

impl ExpenseService {
    pub fn new(repository: Arc<dyn ExpenseRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_expense(
        &self,
        user_id: Uuid,
        category: String,
        description: String,
        amount: f64,
        currency: String,
    ) -> Result<Expense, String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if currency.is_empty() {
            return Err("Currency cannot be empty".to_string());
        }

        if description.trim().is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        let expense = Expense::new(user_id, category, description, amount, currency);
        self.repository.create(expense)
    }

    pub async fn get_expense(&self, id: Uuid) -> Result<Option<Expense>, String> {
        self.repository.get_by_id(id)
    }

    pub async fn get_user_expenses(&self, user_id: Uuid) -> Result<Vec<Expense>, String> {
        self.repository.get_by_user_id(user_id)
    }

    pub async fn get_project_expenses(&self, project_id: Uuid) -> Result<Vec<Expense>, String> {
        self.repository.get_by_project_id(project_id)
    }

    pub async fn filter_expenses(&self, filter: ExpenseFilter) -> Result<Vec<Expense>, String> {
        self.repository.filter_expenses(filter)
    }

    pub async fn update_expense(&self, expense: Expense) -> Result<Expense, String> {
        // Validate the expense
        if expense.amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if expense.currency.is_empty() {
            return Err("Currency cannot be empty".to_string());
        }

        if expense.description.trim().is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        self.repository.update(expense)
    }

    pub async fn delete_expense(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub async fn get_expense_summary(&self, filter: ExpenseFilter) -> Result<ExpenseSummary, String> {
        self.repository.get_summary(filter)
    }

    pub async fn get_monthly_summary(
        &self,
        user_id: Uuid,
        year: i32,
        month: u32,
    ) -> Result<ExpenseSummary, String> {
        use chrono::{TimeZone, Datelike};

        let start_date = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
        let end_date = if month == 12 {
            Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap()
        } else {
            Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0).unwrap()
        };

        let filter = ExpenseFilter {
            user_id: Some(user_id),
            date_from: Some(start_date),
            date_to: Some(end_date),
            ..Default::default()
        };

        self.repository.get_summary(filter)
    }

    pub async fn get_category_breakdown(
        &self,
        user_id: Uuid,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
    ) -> Result<HashMap<String, f64>, String> {
        let filter = ExpenseFilter {
            user_id: Some(user_id),
            date_from,
            date_to,
            ..Default::default()
        };

        let summary = self.repository.get_summary(filter)?;
        Ok(summary.by_category)
    }

    pub async fn attach_expense_to_project(
        &self,
        expense_id: Uuid,
        project_id: Uuid,
    ) -> Result<Expense, String> {
        let mut expense = self.repository.get_by_id(expense_id)?
            .ok_or_else(|| "Expense not found".to_string())?;
        
        expense.project_id = Some(project_id);
        expense.updated_at = chrono::Utc::now();
        expense.sync_version += 1;
        
        self.repository.update(expense)
    }

    pub async fn detach_expense_from_project(
        &self,
        expense_id: Uuid,
    ) -> Result<Expense, String> {
        let mut expense = self.repository.get_by_id(expense_id)?
            .ok_or_else(|| "Expense not found".to_string())?;
        
        expense.project_id = None;
        expense.updated_at = chrono::Utc::now();
        expense.sync_version += 1;
        
        self.repository.update(expense)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expenses::repository::InMemoryExpenseRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_expense() {
        let repo = Arc::new(InMemoryExpenseRepository::new());
        let service = ExpenseService::new(repo);
        
        let user_id = Uuid::new_v4();
        let result = service.create_expense(
            user_id,
            "Office Supplies".to_string(),
            "Printer paper".to_string(),
            25.99,
            "USD".to_string(),
        ).await;
        
        assert!(result.is_ok());
        let expense = result.unwrap();
        assert_eq!(expense.user_id, user_id);
        assert_eq!(expense.amount, 25.99);
    }

    #[tokio::test]
    async fn test_create_expense_validation() {
        let repo = Arc::new(InMemoryExpenseRepository::new());
        let service = ExpenseService::new(repo);
        
        let user_id = Uuid::new_v4();
        
        // Test negative amount
        let result = service.create_expense(
            user_id,
            "Test".to_string(),
            "Test".to_string(),
            -10.0,
            "USD".to_string(),
        ).await;
        assert!(result.is_err());
        
        // Test empty currency
        let result = service.create_expense(
            user_id,
            "Test".to_string(),
            "Test".to_string(),
            10.0,
            "".to_string(),
        ).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_attach_to_project() {
        let repo = Arc::new(InMemoryExpenseRepository::new());
        let service = ExpenseService::new(repo);
        
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        
        let expense = service.create_expense(
            user_id,
            "Travel".to_string(),
            "Conference trip".to_string(),
            500.0,
            "USD".to_string(),
        ).await.unwrap();
        
        assert!(expense.project_id.is_none());
        
        let updated = service.attach_expense_to_project(expense.id, project_id).await.unwrap();
        assert_eq!(updated.project_id, Some(project_id));
    }
}

use std::collections::HashMap;
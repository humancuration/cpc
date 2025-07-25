use crate::expenses::model::{Expense, ExpenseFilter, ExpenseSummary};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait ExpenseRepository: Send + Sync {
    fn create(&self, expense: Expense) -> Result<Expense, String>;
    fn get_by_id(&self, id: Uuid) -> Result<Option<Expense>, String>;
    fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Expense>, String>;
    fn get_by_project_id(&self, project_id: Uuid) -> Result<Vec<Expense>, String>;
    fn filter_expenses(&self, filter: ExpenseFilter) -> Result<Vec<Expense>, String>;
    fn update(&self, expense: Expense) -> Result<Expense, String>;
    fn delete(&self, id: Uuid) -> Result<(), String>;
    fn get_summary(&self, filter: ExpenseFilter) -> Result<ExpenseSummary, String>;
}

#[derive(Debug, Clone)]
pub struct InMemoryExpenseRepository {
    expenses: Arc<RwLock<HashMap<Uuid, Expense>>>,
}

impl InMemoryExpenseRepository {
    pub fn new() -> Self {
        Self {
            expenses: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ExpenseRepository for InMemoryExpenseRepository {
    fn create(&self, expense: Expense) -> Result<Expense, String> {
        let mut expenses = self.expenses.write().unwrap();
        expenses.insert(expense.id, expense.clone());
        Ok(expense)
    }

    fn get_by_id(&self, id: Uuid) -> Result<Option<Expense>, String> {
        let expenses = self.expenses.read().unwrap();
        Ok(expenses.get(&id).cloned())
    }

    fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Expense>, String> {
        let expenses = self.expenses.read().unwrap();
        let user_expenses: Vec<Expense> = expenses
            .values()
            .filter(|expense| expense.user_id == user_id)
            .cloned()
            .collect();
        
        Ok(user_expenses)
    }

    fn get_by_project_id(&self, project_id: Uuid) -> Result<Vec<Expense>, String> {
        let expenses = self.expenses.read().unwrap();
        let project_expenses: Vec<Expense> = expenses
            .values()
            .filter(|expense| expense.project_id == Some(project_id))
            .cloned()
            .collect();
        
        Ok(project_expenses)
    }

    fn filter_expenses(&self, filter: ExpenseFilter) -> Result<Vec<Expense>, String> {
        let expenses = self.expenses.read().unwrap();
        let mut filtered: Vec<Expense> = expenses.values().cloned().collect();

        // Apply filters
        if let Some(user_id) = filter.user_id {
            filtered.retain(|expense| expense.user_id == user_id);
        }

        if let Some(project_id) = filter.project_id {
            filtered.retain(|expense| expense.project_id == Some(project_id));
        }

        if let Some(category) = filter.category {
            filtered.retain(|expense| expense.category == category);
        }

        if let Some(currency) = filter.currency {
            filtered.retain(|expense| expense.currency == currency);
        }

        if let Some(date_from) = filter.date_from {
            filtered.retain(|expense| expense.transaction_date >= date_from);
        }

        if let Some(date_to) = filter.date_to {
            filtered.retain(|expense| expense.transaction_date <= date_to);
        }

        if let Some(min_amount) = filter.min_amount {
            filtered.retain(|expense| expense.amount >= min_amount);
        }

        if let Some(max_amount) = filter.max_amount {
            filtered.retain(|expense| expense.amount <= max_amount);
        }

        if let Some(search_term) = filter.search_term {
            let term = search_term.to_lowercase();
            filtered.retain(|expense| {
                expense.description.to_lowercase().contains(&term) ||
                expense.category.to_lowercase().contains(&term) ||
                expense.metadata.values().any(|v| v.to_lowercase().contains(&term))
            });
        }

        // Sort by transaction date descending
        filtered.sort_by(|a, b| b.transaction_date.cmp(&a.transaction_date));

        Ok(filtered)
    }

    fn update(&self, expense: Expense) -> Result<Expense, String> {
        let mut expenses = self.expenses.write().unwrap();
        if expenses.contains_key(&expense.id) {
            expenses.insert(expense.id, expense.clone());
            Ok(expense)
        } else {
            Err("Expense not found".to_string())
        }
    }

    fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut expenses = self.expenses.write().unwrap();
        if expenses.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Expense not found".to_string())
        }
    }

    fn get_summary(&self, filter: ExpenseFilter) -> Result<ExpenseSummary, String> {
        let expenses = self.filter_expenses(filter.clone())?;
        
        let total_expenses = expenses.len();
        let currency = filter.currency.clone().unwrap_or_else(|| "USD".to_string());
        
        let total_amount: f64 = expenses
            .iter()
            .filter(|expense| expense.currency == currency)
            .map(|expense| expense.amount)
            .sum();

        let mut by_category = HashMap::new();
        for expense in expenses {
            *by_category.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
        }

        Ok(ExpenseSummary {
            total_expenses,
            total_amount,
            currency,
            date_from: filter.date_from,
            date_to: filter.date_to,
            by_category,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_get_expense() {
        let repo = InMemoryExpenseRepository::new();
        let user_id = Uuid::new_v4();
        let expense = Expense::new(
            user_id,
            "Office Supplies".to_string(),
            "Printer paper and ink".to_string(),
            45.99,
            "USD".to_string(),
        );

        let created = repo.create(expense.clone()).unwrap();
        assert_eq!(created.id, expense.id);
        
        let retrieved = repo.get_by_id(expense.id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, expense.id);
    }

    #[test]
    fn test_filter_expenses() {
        let repo = InMemoryExpenseRepository::new();
        let user_id = Uuid::new_v4();
        
        let expense1 = Expense::new(
            user_id,
            "Travel".to_string(),
            "Flight to conference".to_string(),
            450.00,
            "USD".to_string(),
        );
        
        let expense2 = Expense::new(
            user_id,
            "Meals".to_string(),
            "Team lunch".to_string(),
            85.50,
            "USD".to_string(),
        );

        repo.create(expense1.clone()).unwrap();
        repo.create(expense2.clone()).unwrap();

        let filter = ExpenseFilter {
            user_id: Some(user_id),
            category: Some("Travel".to_string()),
            ..Default::default()
        };

        let results = repo.filter_expenses(filter).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, expense1.id);
    }
}

impl Default for ExpenseFilter {
    fn default() -> Self {
        Self {
            user_id: None,
            project_id: None,
            category: None,
            date_from: None,
            date_to: None,
            min_amount: None,
            max_amount: None,
            currency: None,
            search_term: None,
        }
    }
}
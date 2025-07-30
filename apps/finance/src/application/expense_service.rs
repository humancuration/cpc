use crate::domain::{Expense, FinancialCategory, Money};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing expenses
pub struct ExpenseService {
    expenses: HashMap<Uuid, Expense>,
}

impl ExpenseService {
    pub fn new() -> Self {
        Self {
            expenses: HashMap::new(),
        }
    }
    
    /// Create a new expense
    pub fn create_expense(
        &mut self,
        amount: Money,
        category: FinancialCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Result<Expense, String> {
        let expense = Expense::new(amount, category, date, description);
        self.expenses.insert(expense.id, expense.clone());
        Ok(expense)
    }
    
    /// Get an expense by ID
    pub fn get_expense(&self, id: &Uuid) -> Option<&Expense> {
        self.expenses.get(id)
    }
    
    /// Get all expenses
    pub fn get_all_expenses(&self) -> Vec<&Expense> {
        self.expenses.values().collect()
    }
    
    /// Update expense amount
    pub fn update_expense_amount(
        &mut self,
        id: &Uuid,
        new_amount: Money,
    ) -> Result<(), String> {
        match self.expenses.get_mut(id) {
            Some(expense) => {
                expense.update_amount(new_amount);
                Ok(())
            }
            None => Err("Expense not found".to_string()),
        }
    }
    
    /// Update expense description
    pub fn update_expense_description(
        &mut self,
        id: &Uuid,
        new_description: String,
    ) -> Result<(), String> {
        match self.expenses.get_mut(id) {
            Some(expense) => {
                expense.update_description(new_description);
                Ok(())
            }
            None => Err("Expense not found".to_string()),
        }
    }
    
    /// Delete an expense
    pub fn delete_expense(&mut self, id: &Uuid) -> Result<(), String> {
        match self.expenses.remove(id) {
            Some(_) => Ok(()),
            None => Err("Expense not found".to_string()),
        }
    }
    
    /// Add a tag to an expense
    pub fn add_tag_to_expense(
        &mut self,
        id: &Uuid,
        tag: String,
    ) -> Result<(), String> {
        match self.expenses.get_mut(id) {
            Some(expense) => {
                expense.add_tag(tag);
                Ok(())
            }
            None => Err("Expense not found".to_string()),
        }
    }
    
    /// Remove a tag from an expense
    pub fn remove_tag_from_expense(
        &mut self,
        id: &Uuid,
        tag: &str,
    ) -> Result<(), String> {
        match self.expenses.get_mut(id) {
            Some(expense) => {
                expense.remove_tag(tag);
                Ok(())
            }
            None => Err("Expense not found".to_string()),
        }
    }
    
    /// Get expenses by category
    pub fn get_expenses_by_category(&self, category: &FinancialCategory) -> Vec<&Expense> {
        self.expenses
            .values()
            .filter(|expense| &expense.category == category)
            .collect()
    }
    
    /// Get expenses within a date range
    pub fn get_expenses_in_date_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Vec<&Expense> {
        self.expenses
            .values()
            .filter(|expense| {
                &expense.date >= start_date && &expense.date <= end_date
            })
            .collect()
    }
    
    /// Get total expenses for a category in a date range
    pub fn get_total_expenses_for_category(
        &self,
        category: &FinancialCategory,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Money {
        // For simplicity, we'll assume all expenses are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for expense in self.expenses.values() {
            if &expense.category == category 
                && &expense.date >= start_date 
                && &expense.date <= end_date {
                total_amount += expense.amount.amount;
                currency = expense.amount.currency.clone();
            }
        }
        
        Money::new(total_amount, &currency)
    }
    
    /// Get total expenses in a date range
    pub fn get_total_expenses_in_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Money {
        // For simplicity, we'll assume all expenses are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for expense in self.expenses.values() {
            if &expense.date >= start_date && &expense.date <= end_date {
                total_amount += expense.amount.amount;
                currency = expense.amount.currency.clone();
            }
        }
        
        Money::new(total_amount, &currency)
    }
}
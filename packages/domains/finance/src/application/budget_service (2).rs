use crate::domain::{Budget, FinancialCategory, Money, TimePeriod};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing budgets
pub struct BudgetService {
    budgets: HashMap<Uuid, Budget>,
}

impl BudgetService {
    pub fn new() -> Self {
        Self {
            budgets: HashMap::new(),
        }
    }
    
    /// Create a new budget
    pub fn create_budget(
        &mut self,
        name: String,
        category: FinancialCategory,
        amount: Money,
        period: TimePeriod,
    ) -> Result<Budget, String> {
        let budget = Budget::new(name, category, amount, period);
        self.budgets.insert(budget.id, budget.clone());
        Ok(budget)
    }
    
    /// Get a budget by ID
    pub fn get_budget(&self, id: &Uuid) -> Option<&Budget> {
        self.budgets.get(id)
    }
    
    /// Get all budgets
    pub fn get_all_budgets(&self) -> Vec<&Budget> {
        self.budgets.values().collect()
    }
    
    /// Get active budgets
    pub fn get_active_budgets(&self) -> Vec<&Budget> {
        let now = Utc::now();
        self.budgets
            .values()
            .filter(|budget| budget.period.contains(&now))
            .collect()
    }
    
    /// Update budget amount
    pub fn update_budget_amount(
        &mut self,
        id: &Uuid,
        new_amount: Money,
    ) -> Result<(), String> {
        match self.budgets.get_mut(id) {
            Some(budget) => {
                budget.update_amount(new_amount);
                Ok(())
            }
            None => Err("Budget not found".to_string()),
        }
    }
    
    /// Update budget period
    pub fn update_budget_period(
        &mut self,
        id: &Uuid,
        new_period: TimePeriod,
    ) -> Result<(), String> {
        match self.budgets.get_mut(id) {
            Some(budget) => {
                budget.update_period(new_period);
                Ok(())
            }
            None => Err("Budget not found".to_string()),
        }
    }
    
    /// Delete a budget
    pub fn delete_budget(&mut self, id: &Uuid) -> Result<(), String> {
        match self.budgets.remove(id) {
            Some(_) => Ok(()),
            None => Err("Budget not found".to_string()),
        }
    }
    
    /// Get budgets by category
    pub fn get_budgets_by_category(&self, category: &FinancialCategory) -> Vec<&Budget> {
        self.budgets
            .values()
            .filter(|budget| &budget.category == category)
            .collect()
    }
    
    /// Get total budgeted amount for a period
    pub fn get_total_budgeted_amount(&self, period: &TimePeriod) -> Money {
        // For simplicity, we'll assume all budgets are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for budget in self.budgets.values() {
            if period.contains(&budget.period.start) || period.contains(&budget.period.end) {
                total_amount += budget.amount.amount;
                currency = budget.amount.currency.clone();
            }
        }
        
        Money::new(total_amount, &currency)
    }
}
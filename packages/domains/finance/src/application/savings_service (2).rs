use crate::domain::{SavingsGoal, GoalVisualStyle, Money};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing savings goals
pub struct SavingsService {
    savings_goals: HashMap<Uuid, SavingsGoal>,
}

impl SavingsService {
    pub fn new() -> Self {
        Self {
            savings_goals: HashMap::new(),
        }
    }
    
    /// Create a new savings goal
    pub fn create_savings_goal(
        &mut self,
        name: String,
        target_amount: Money,
        target_date: DateTime<Utc>,
        visual_style: GoalVisualStyle,
    ) -> Result<SavingsGoal, String> {
        let savings_goal = SavingsGoal::new(name, target_amount, target_date, visual_style);
        self.savings_goals.insert(savings_goal.id, savings_goal.clone());
        Ok(savings_goal)
    }
    
    /// Get a savings goal by ID
    pub fn get_savings_goal(&self, id: &Uuid) -> Option<&SavingsGoal> {
        self.savings_goals.get(id)
    }
    
    /// Get all savings goals
    pub fn get_all_savings_goals(&self) -> Vec<&SavingsGoal> {
        self.savings_goals.values().collect()
    }
    
    /// Update current amount for a savings goal
    pub fn update_current_amount(
        &mut self,
        id: &Uuid,
        amount: Money,
    ) -> Result<(), String> {
        match self.savings_goals.get_mut(id) {
            Some(savings_goal) => {
                savings_goal.update_current_amount(amount);
                Ok(())
            }
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Add to current amount for a savings goal
    pub fn add_to_current_amount(
        &mut self,
        id: &Uuid,
        amount: Money,
    ) -> Result<(), String> {
        match self.savings_goals.get_mut(id) {
            Some(savings_goal) => {
                savings_goal.add_to_current_amount(amount);
                Ok(())
            }
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Update target amount for a savings goal
    pub fn update_target_amount(
        &mut self,
        id: &Uuid,
        new_target: Money,
    ) -> Result<(), String> {
        match self.savings_goals.get_mut(id) {
            Some(savings_goal) => {
                savings_goal.update_target_amount(new_target);
                Ok(())
            }
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Update target date for a savings goal
    pub fn update_target_date(
        &mut self,
        id: &Uuid,
        new_date: DateTime<Utc>,
    ) -> Result<(), String> {
        match self.savings_goals.get_mut(id) {
            Some(savings_goal) => {
                savings_goal.update_target_date(new_date);
                Ok(())
            }
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Update visual style for a savings goal
    pub fn update_visual_style(
        &mut self,
        id: &Uuid,
        style: GoalVisualStyle,
    ) -> Result<(), String> {
        match self.savings_goals.get_mut(id) {
            Some(savings_goal) => {
                savings_goal.update_visual_style(style);
                Ok(())
            }
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Delete a savings goal
    pub fn delete_savings_goal(&mut self, id: &Uuid) -> Result<(), String> {
        match self.savings_goals.remove(id) {
            Some(_) => Ok(()),
            None => Err("Savings goal not found".to_string()),
        }
    }
    
    /// Get completed savings goals
    pub fn get_completed_goals(&self) -> Vec<&SavingsGoal> {
        self.savings_goals
            .values()
            .filter(|goal| goal.is_completed())
            .collect()
    }
    
    /// Get overdue savings goals
    pub fn get_overdue_goals(&self) -> Vec<&SavingsGoal> {
        self.savings_goals
            .values()
            .filter(|goal| goal.is_overdue())
            .collect()
    }
    
    /// Get total target amount for all goals
    pub fn get_total_target_amount(&self) -> Money {
        // For simplicity, we'll assume all goals are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for goal in self.savings_goals.values() {
            total_amount += goal.target_amount.amount;
            currency = goal.target_amount.currency.clone();
        }
        
        Money::new(total_amount, &currency)
    }
    
    /// Get total current amount for all goals
    pub fn get_total_current_amount(&self) -> Money {
        // For simplicity, we'll assume all goals are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for goal in self.savings_goals.values() {
            total_amount += goal.current_amount.amount;
            currency = goal.current_amount.currency.clone();
        }
        
        Money::new(total_amount, &currency)
    }
    
    /// Get overall savings progress
    pub fn get_overall_progress(&self) -> f64 {
        let total_target = self.get_total_target_amount().amount;
        let total_current = self.get_total_current_amount().amount;
        
        if total_target > 0.0 {
            (total_current / total_target).min(1.0)
        } else {
            0.0
        }
    }
}
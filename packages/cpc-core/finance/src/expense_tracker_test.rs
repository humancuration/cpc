//! Tests for the expense tracker module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::expense_tracker::{Expense, ExpenseCategory, ExpenseStatus};
    use crate::domain::primitives::{Money, Currency};
    use rust_decimal_macros::dec;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_expense_creation() {
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(15.75), Currency::USD);
        let category = ExpenseCategory::Food;
        let date = Utc::now();
        let description = "Lunch at cafe".to_string();
        
        let expense = Expense::new(user_id, amount.clone(), category.clone(), date, description.clone());
        
        assert_eq!(expense.user_id, user_id);
        assert_eq!(expense.amount, amount);
        assert_eq!(expense.category, category);
        assert_eq!(expense.description, description);
        assert_eq!(expense.status, ExpenseStatus::Processed);
        assert!(expense.receipt_id.is_none());
        assert!(!expense.is_recurring);
        assert!(expense.recurrence_pattern.is_none());
        assert!(expense.linked_budget_id.is_none());
    }
    
    #[test]
    fn test_draft_expense_creation() {
        let user_id = Uuid::new_v4();
        let receipt_id = Uuid::new_v4();
        let description = "Scanned receipt".to_string();
        
        let expense = Expense::new_draft(user_id, receipt_id, description.clone());
        
        assert_eq!(expense.user_id, user_id);
        assert_eq!(expense.receipt_id, Some(receipt_id));
        assert_eq!(expense.description, description);
        assert_eq!(expense.status, ExpenseStatus::Draft);
        assert_eq!(expense.category, ExpenseCategory::Other("Unprocessed".to_string()));
    }
    
    #[test]
    fn test_expense_update_amount() {
        let user_id = Uuid::new_v4();
        let initial_amount = Money::new(dec!(10.0), Currency::USD);
        let category = ExpenseCategory::Food;
        let date = Utc::now();
        let description = "Initial expense".to_string();
        
        let mut expense = Expense::new(user_id, initial_amount, category, date, description);
        let new_amount = Money::new(dec!(20.0), Currency::USD);
        
        let result = expense.update_amount(new_amount.clone());
        assert!(result.is_ok());
        assert_eq!(expense.amount, new_amount);
    }
    
    #[test]
    fn test_expense_link_to_budget() {
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(10.0), Currency::USD);
        let category = ExpenseCategory::Food;
        let date = Utc::now();
        let description = "Expense to link".to_string();
        let budget_id = Uuid::new_v4();
        
        let mut expense = Expense::new(user_id, amount, category, date, description);
        expense.link_to_budget(budget_id);
        
        assert_eq!(expense.linked_budget_id, Some(budget_id));
    }
    
    #[test]
    fn test_expense_mark_as_recurring() {
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(10.0), Currency::USD);
        let category = ExpenseCategory::Food;
        let date = Utc::now();
        let description = "Recurring expense".to_string();
        let pattern = "0 0 1 * *".to_string(); // Monthly pattern
        
        let mut expense = Expense::new(user_id, amount, category, date, description);
        expense.mark_as_recurring(pattern.clone());
        
        assert!(expense.is_recurring);
        assert_eq!(expense.recurrence_pattern, Some(pattern));
    }
}
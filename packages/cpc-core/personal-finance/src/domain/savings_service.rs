//! Savings service for managing savings goals and auto-deductions

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use cpc_core::finance::{Money, Currency};

use crate::domain::models::{SavingsGoal, FinanceError};

/// Trait for Treasury Service integration
#[async_trait::async_trait]
pub trait TreasuryService: Send + Sync {
    async fn get_user_balance(&self, user_id: Uuid) -> Result<Money, FinanceError>;
    async fn transfer_to_savings(&self, user_id: Uuid, amount: Money) -> Result<(), FinanceError>;
    async fn transfer_from_savings(&self, user_id: Uuid, amount: Money) -> Result<(), FinanceError>;
}

/// Mock Treasury service for development
pub struct MockTreasuryService;

#[async_trait::async_trait]
impl TreasuryService for MockTreasuryService {
    async fn get_user_balance(&self, _user_id: Uuid) -> Result<Money, FinanceError> {
        Ok(Money::new(Decimal::new(1000, 0), Currency::USD)) // Mock $1000 balance
    }

    async fn transfer_to_savings(&self, _user_id: Uuid, _amount: Money) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn transfer_from_savings(&self, _user_id: Uuid, _amount: Money) -> Result<(), FinanceError> {
        Ok(())
    }
}

/// Savings service for managing savings goals and auto-deductions
pub struct SavingsService {
    treasury_service: Arc<dyn TreasuryService>,
    auto_deduction_percentage: Decimal,
}

impl SavingsService {
    pub fn new(treasury_service: Arc<dyn TreasuryService>, auto_deduction_percentage: Decimal) -> Self {
        Self {
            treasury_service,
            auto_deduction_percentage,
        }
    }

    /// Create a new savings goal
    pub async fn create_savings_goal(
        &self,
        user_id: Uuid,
        name: String,
        target_amount: Money,
        target_date: DateTime<Utc>,
    ) -> Result<SavingsGoal, FinanceError> {
        let goal = SavingsGoal::new(
            user_id,
            name,
            target_amount,
            target_date,
        );

        Ok(goal)
    }

    /// Calculate monthly savings needed to reach goal
    pub async fn calculate_monthly_savings(
        &self,
        goal: &SavingsGoal,
    ) -> Result<Money, FinanceError> {
        let now = Utc::now();
        let months_remaining = if goal.target_date > now {
            let duration = goal.target_date - now;
            let days = duration.num_days() as f64;
            (days / 30.44).ceil() as u64 // Average days per month
        } else {
            return Err(FinanceError::ValidationError("Target date is in the past".to_string()));
        };

        let remaining_amount = goal.remaining_amount();
        if remaining_amount.is_zero() {
            return Ok(Money::zero(remaining_amount.currency));
        }

        let monthly_amount = remaining_amount.amount / Decimal::from(months_remaining);
        Ok(Money::new(monthly_amount, remaining_amount.currency))
    }

    /// Auto-deduct savings from income
    pub async fn auto_deduct_savings(
        &self,
        user_id: Uuid,
        monthly_income: Money,
        active_goals: Vec<SavingsGoal>,
    ) -> Result<Vec<SavingsDeduction>, FinanceError> {
        let total_deduction_amount = monthly_income.amount * self.auto_deduction_percentage / Decimal::from(100);
        let total_deduction = Money::new(total_deduction_amount, monthly_income.currency);
        
        if total_deduction.is_zero() {
            return Ok(Vec::new());
        }

        // Filter active goals that need funding
        let mut active_goals: Vec<SavingsGoal> = active_goals
            .into_iter()
            .filter(|g| !g.is_complete())
            .collect();

        if active_goals.is_empty() {
            return Ok(Vec::new());
        }

        // Sort by priority (closest target date first)
        active_goals.sort_by_key(|g| g.target_date);

        let mut deductions = Vec::new();
        let mut remaining_deduction = total_deduction;

        // Distribute deduction across goals
        for goal in active_goals {
            if remaining_deduction.is_zero() {
                break;
            }

            let needed_amount = goal.remaining_amount();
            let deduction_amount = if needed_amount.amount < remaining_deduction.amount {
                needed_amount
            } else {
                remaining_deduction.clone()
            };

            self.treasury_service
                .transfer_to_savings(user_id, deduction_amount.clone())
                .await?;

            let remaining_target = needed_amount.subtract(&deduction_amount)
                .unwrap_or_else(|_| Money::zero(needed_amount.currency));
                
            deductions.push(SavingsDeduction {
                goal_id: goal.id,
                amount: deduction_amount,
                remaining_target,
            });

            remaining_deduction = remaining_deduction.subtract(&deduction_amount)
                .unwrap_or_else(|_| Money::zero(remaining_deduction.currency));
        }

        Ok(deductions)
    }

    /// Manual savings contribution
    pub async fn manual_contribution(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Money,
    ) -> Result<(), FinanceError> {
        if !amount.is_positive() {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        let balance = self.treasury_service.get_user_balance(user_id).await?;
        if balance.amount < amount.amount {
            return Err(FinanceError::InsufficientFunds {
                required: amount,
                available: balance,
            });
        }

        self.treasury_service
            .transfer_to_savings(user_id, amount.clone())
            .await?;

        let new_amount = goal.current_amount.add(&amount)
            .map_err(|e| FinanceError::InvalidAmount(e.to_string()))?;
        goal.current_amount = new_amount;
        Ok(())
    }

    /// Withdraw from savings goal
    pub async fn withdraw_from_goal(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Money,
    ) -> Result<(), FinanceError> {
        if !amount.is_positive() {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        if amount.amount > goal.current_amount.amount {
            return Err(FinanceError::InsufficientFunds {
                required: amount,
                available: goal.current_amount.clone(),
            });
        }

        self.treasury_service
            .transfer_from_savings(user_id, amount.clone())
            .await?;

        let new_amount = goal.current_amount.subtract(&amount)
            .map_err(|e| FinanceError::InvalidAmount(e.to_string()))?;
        goal.current_amount = new_amount;
        Ok(())
    }

    /// Get savings progress report
    pub async fn get_savings_report(
        &self,
        goals: Vec<SavingsGoal>,
    ) -> Result<SavingsReport, FinanceError> {
        // For analysis, we'll convert to a common currency (USD) if needed
        // In a real implementation, we'd need proper currency conversion
        let mut total_target = Money::zero(Currency::USD);
        let mut total_saved = Money::zero(Currency::USD);
        
        for goal in &goals {
            // Add target amounts (assuming same currency for simplicity)
            total_target = total_target.add(&goal.target_amount)
                .unwrap_or_else(|_| total_target.clone());
            
            // Add current amounts (assuming same currency for simplicity)
            total_saved = total_saved.add(&goal.current_amount)
                .unwrap_or_else(|_| total_saved.clone());
        }
        
        let goals_by_category = self.categorize_goals(&goals);
        
        let completion_rate = if total_target.is_zero() {
            Decimal::ZERO
        } else {
            let saved_amount = total_saved.amount;
            let target_amount = total_target.amount;
            if target_amount.is_zero() {
                Decimal::ZERO
            } else {
                (saved_amount / target_amount) * Decimal::from(100)
            }
        };

        let active_goals: Vec<&SavingsGoal> = goals.iter()
            .filter(|g| !g.is_complete())
            .collect();

        let estimated_completion = self.calculate_estimated_completion(&active_goals).await?;

        Ok(SavingsReport {
            total_saved,
            total_target,
            completion_rate,
            goals_by_category,
            active_goals_count: active_goals.len(),
            completed_goals_count: goals.len() - active_goals.len(),
            estimated_completion_date: estimated_completion,
        })
    }

    /// Categorize goals by their category
    fn categorize_goals(&self, goals: &[SavingsGoal]) -> HashMap<String, CategorySummary> {
        let mut categories = HashMap::new();

        for goal in goals {
            let summary = categories
                .entry(goal.category.clone().unwrap_or_else(|| "Uncategorized".to_string()))
                .or_insert(CategorySummary {
                    total_target: Money::zero(Currency::USD),
                    total_saved: Money::zero(Currency::USD),
                    goal_count: 0,
                });

            // Add target amounts (assuming same currency for simplicity)
            summary.total_target = summary.total_target.add(&goal.target_amount)
                .unwrap_or_else(|_| summary.total_target.clone());
            
            // Add current amounts (assuming same currency for simplicity)
            summary.total_saved = summary.total_saved.add(&goal.current_amount)
                .unwrap_or_else(|_| summary.total_saved.clone());
                
            summary.goal_count += 1;
        }

        categories
    }

    /// Calculate estimated completion date for active goals
    async fn calculate_estimated_completion(
        &self,
        active_goals: &[&SavingsGoal],
    ) -> Result<Option<DateTime<Utc>>, FinanceError> {
        if active_goals.is_empty() {
            return Ok(None);
        }

        let mut latest_date = Utc::now();
        let mut total_needed = Money::zero(Currency::USD);
        let mut total_monthly = Money::zero(Currency::USD);

        for goal in active_goals {
            let monthly_needed = self.calculate_monthly_savings(goal).await?;
            
            // Add remaining amounts (assuming same currency for simplicity)
            let remaining = goal.remaining_amount();
            total_needed = total_needed.add(&remaining)
                .unwrap_or_else(|_| total_needed.clone());
            
            // Add monthly amounts (assuming same currency for simplicity)
            total_monthly = total_monthly.add(&monthly_needed)
                .unwrap_or_else(|_| total_monthly.clone());
                
            latest_date = latest_date.max(goal.target_date);
        }

        if total_monthly.is_zero() {
            return Ok(Some(latest_date));
        }

        let months_needed = (total_needed.amount / total_monthly.amount).ceil().to_u64().unwrap_or(0);
        Ok(Some(Utc::now() + Duration::days(months_needed as i64 * 30)))
    }

    /// Rebalance savings allocation
    pub async fn rebalance_goals(
        &self,
        goals: &mut Vec<SavingsGoal>,
        new_percentages: HashMap<Uuid, Decimal>,
    ) -> Result<(), FinanceError> {
        let total_percentage: Decimal = new_percentages.values().sum();
        
        if (total_percentage - Decimal::from(100)).abs() > Decimal::new(1, 2) {
            return Err(FinanceError::ValidationError(
                "Percentages must sum to 100%".to_string()
            ));
        }

        for (goal_id, percentage) in new_percentages {
            if let Some(goal) = goals.iter_mut().find(|g| g.id == goal_id) {
                // In a real implementation, we would adjust the goal based on the percentage
                // For now, we'll just validate the percentage
                if percentage < Decimal::ZERO || percentage > Decimal::from(100) {
                    return Err(FinanceError::ValidationError(
                        "Percentage must be between 0 and 100".to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub total_target: Money,
    pub total_saved: Money,
    pub goal_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsDeduction {
    pub goal_id: Uuid,
    pub amount: Money,
    pub remaining_target: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsReport {
    pub total_saved: Money,
    pub total_target: Money,
    pub completion_rate: Decimal,
    pub goals_by_category: HashMap<String, CategorySummary>,
    pub active_goals_count: usize,
    pub completed_goals_count: usize,
    pub estimated_completion_date: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::Duration;

    #[tokio::test]
    async fn test_create_savings_goal() {
        let treasury_service = Arc::new(MockTreasuryService);
        let service = SavingsService::new(treasury_service, dec!(10));
        
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(365);
        let target_amount = Money::new(dec!(1000), Currency::USD);
        
        let goal = service
            .create_savings_goal(user_id, "Vacation Fund".to_string(), target_amount, target_date)
            .await
            .unwrap();
            
        assert_eq!(goal.name, "Vacation Fund");
        assert_eq!(goal.target_amount, target_amount);
        assert_eq!(goal.target_date, target_date);
    }

    #[tokio::test]
    async fn test_calculate_monthly_savings() {
        let treasury_service = Arc::new(MockTreasuryService);
        let service = SavingsService::new(treasury_service, dec!(10));
        
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(365);
        let target_amount = Money::new(dec!(1000), Currency::USD);
        
        let mut goal = SavingsGoal::new(user_id, "Emergency Fund".to_string(), target_amount, target_date);
        goal.current_amount = Money::new(dec!(200), Currency::USD);
        
        let monthly_savings = service
            .calculate_monthly_savings(&goal)
            .await
            .unwrap();
            
        assert!(monthly_savings.amount > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_manual_contribution() {
        let treasury_service = Arc::new(MockTreasuryService);
        let service = SavingsService::new(treasury_service, dec!(10));
        
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(365);
        let target_amount = Money::new(dec!(1000), Currency::USD);
        
        let mut goal = SavingsGoal::new(user_id, "Savings Goal".to_string(), target_amount, target_date);
        let contribution = Money::new(dec!(100), Currency::USD);
        
        let result = service
            .manual_contribution(user_id, &mut goal, contribution)
            .await;
            
        assert!(result.is_ok());
        assert_eq!(goal.current_amount.amount, dec!(100));
    }
}
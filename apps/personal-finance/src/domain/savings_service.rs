//! Savings service for managing savings goals and auto-deductions

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

use crate::domain::models::{SavingsGoal, FinanceError};

/// Trait for Treasury Service integration
#[async_trait::async_trait]
pub trait TreasuryService: Send + Sync {
    async fn get_user_balance(&self, user_id: Uuid) -> Result<Decimal, FinanceError>;
    async fn transfer_to_savings(&self, user_id: Uuid, amount: Decimal) -> Result<(), FinanceError>;
    async fn transfer_from_savings(&self, user_id: Uuid, amount: Decimal) -> Result<(), FinanceError>;
}

/// Mock Treasury service for development
pub struct MockTreasuryService;

#[async_trait::async_trait]
impl TreasuryService for MockTreasuryService {
    async fn get_user_balance(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
        Ok(Decimal::new(1000, 0)) // Mock $1000 balance
    }

    async fn transfer_to_savings(&self, _user_id: Uuid, _amount: Decimal) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn transfer_from_savings(&self, _user_id: Uuid, _amount: Decimal) -> Result<(), FinanceError> {
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
        target_amount: Decimal,
        target_date: DateTime<Utc>,
        category: String,
    ) -> Result<SavingsGoal, FinanceError> {
        let goal = SavingsGoal::new(
            user_id,
            name,
            target_amount,
            target_date,
            category,
        );

        Ok(goal)
    }

    /// Calculate monthly savings needed to reach goal
    pub async fn calculate_monthly_savings(
        &self,
        goal: &SavingsGoal,
    ) -> Result<Decimal, FinanceError> {
        let now = Utc::now();
        let months_remaining = if goal.target_date > now {
            let duration = goal.target_date - now;
            let days = duration.num_days() as f64;
            (days / 30.44).ceil() as u64 // Average days per month
        } else {
            return Err(FinanceError::ValidationError("Target date is in the past".to_string()));
        };

        let remaining_amount = goal.target_amount - goal.current_amount;
        if remaining_amount <= Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        Ok(remaining_amount / Decimal::from(months_remaining))
    }

    /// Auto-deduct savings from income
    pub async fn auto_deduct_savings(
        &self,
        user_id: Uuid,
        monthly_income: Decimal,
        active_goals: Vec<SavingsGoal>,
    ) -> Result<Vec<SavingsDeduction>, FinanceError> {
        let total_deduction = monthly_income * self.auto_deduction_percentage / Decimal::from(100);
        
        if total_deduction <= Decimal::ZERO {
            return Ok(Vec::new());
        }

        // Filter active goals that need funding
        let mut active_goals: Vec<SavingsGoal> = active_goals
            .into_iter()
            .filter(|g| g.current_amount < g.target_amount && !g.is_completed())
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
            if remaining_deduction <= Decimal::ZERO {
                break;
            }

            let needed_amount = goal.target_amount - goal.current_amount;
            let deduction_amount = needed_amount.min(remaining_deduction);

            self.treasury_service
                .transfer_to_savings(user_id, deduction_amount)
                .await?;

            deductions.push(SavingsDeduction {
                goal_id: goal.id,
                amount: deduction_amount,
                remaining_target: needed_amount - deduction_amount,
            });

            remaining_deduction -= deduction_amount;
        }

        Ok(deductions)
    }

    /// Manual savings contribution
    pub async fn manual_contribution(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Decimal,
    ) -> Result<(), FinanceError> {
        if amount <= Decimal::ZERO {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        let balance = self.treasury_service.get_user_balance(user_id).await?;
        if balance < amount {
            return Err(FinanceError::InsufficientFunds);
        }

        self.treasury_service
            .transfer_to_savings(user_id, amount)
            .await?;

        goal.add_contribution(amount);
        Ok(())
    }

    /// Withdraw from savings goal
    pub async fn withdraw_from_goal(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Decimal,
    ) -> Result<(), FinanceError> {
        if amount <= Decimal::ZERO {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        if amount > goal.current_amount {
            return Err(FinanceError::InsufficientFunds);
        }

        self.treasury_service
            .transfer_from_savings(user_id, amount)
            .await?;

        goal.withdraw(amount);
        Ok(())
    }

    /// Get savings progress report
    pub async fn get_savings_report(
        &self,
        goals: Vec<SavingsGoal>,
    ) -> Result<SavingsReport, FinanceError> {
        let total_target: Decimal = goals.iter().map(|g| g.target_amount).sum();
        let total_saved: Decimal = goals.iter().map(|g| g.current_amount).sum();
        
        let goals_by_category = self.categorize_goals(&goals);
        let completion_rate = if total_target > Decimal::ZERO {
            (total_saved / total_target) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let active_goals: Vec<&SavingsGoal> = goals.iter()
            .filter(|g| !g.is_completed())
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
                .entry(goal.category.clone())
                .or_insert(CategorySummary {
                    total_target: Decimal::ZERO,
                    total_saved: Decimal::ZERO,
                    goal_count: 0,
                });

            summary.total_target += goal.target_amount;
            summary.total_saved += goal.current_amount;
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
        let mut total_needed = Decimal::ZERO;
        let mut total_monthly = Decimal::ZERO;

        for goal in active_goals {
            let monthly_needed = self.calculate_monthly_savings(goal).await?;
            total_needed += goal.target_amount - goal.current_amount;
            total_monthly += monthly_needed;
            latest_date = latest_date.max(goal.target_date);
        }

        if total_monthly <= Decimal::ZERO {
            return Ok(Some(latest_date));
        }

        let months_needed = (total_needed / total_monthly).ceil().to_u64().unwrap_or(0);
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
                if percentage < Decimal::ZERO || percentage >
//! Savings service for managing savings goals and auto-deductions

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

use crate::domain::models::{SavingsGoal, FinanceError};

/// Trait for Treasury Service integration
#[async_trait::async_trait]
pub trait TreasuryService: Send + Sync {
    async fn get_user_balance(&self, user_id: Uuid) -> Result<Decimal, FinanceError>;
    async fn transfer_to_savings(&self, user_id: Uuid, amount: Decimal) -> Result<(), FinanceError>;
    async fn transfer_from_savings(&self, user_id: Uuid, amount: Decimal) -> Result<(), FinanceError>;
}

/// Mock Treasury service for development
pub struct MockTreasuryService;

#[async_trait::async_trait]
impl TreasuryService for MockTreasuryService {
    async fn get_user_balance(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
        Ok(Decimal::new(1000, 0)) // Mock $1000 balance
    }

    async fn transfer_to_savings(&self, _user_id: Uuid, _amount: Decimal) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn transfer_from_savings(&self, _user_id: Uuid, _amount: Decimal) -> Result<(), FinanceError> {
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
        target_amount: Decimal,
        target_date: DateTime<Utc>,
        category: String,
    ) -> Result<SavingsGoal, FinanceError> {
        let goal = SavingsGoal::new(
            user_id,
            name,
            target_amount,
            target_date,
            category,
        );

        Ok(goal)
    }

    /// Calculate monthly savings needed to reach goal
    pub async fn calculate_monthly_savings(
        &self,
        goal: &SavingsGoal,
    ) -> Result<Decimal, FinanceError> {
        let now = Utc::now();
        let months_remaining = if goal.target_date > now {
            let duration = goal.target_date - now;
            let days = duration.num_days() as f64;
            (days / 30.44).ceil() as u64 // Average days per month
        } else {
            return Err(FinanceError::ValidationError("Target date is in the past".to_string()));
        };

        let remaining_amount = goal.target_amount - goal.current_amount;
        if remaining_amount <= Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        Ok(remaining_amount / Decimal::from(months_remaining))
    }

    /// Auto-deduct savings from income
    pub async fn auto_deduct_savings(
        &self,
        user_id: Uuid,
        monthly_income: Decimal,
        active_goals: Vec<SavingsGoal>,
    ) -> Result<Vec<SavingsDeduction>, FinanceError> {
        let total_deduction = monthly_income * self.auto_deduction_percentage / Decimal::from(100);
        
        if total_deduction <= Decimal::ZERO {
            return Ok(Vec::new());
        }

        // Filter active goals that need funding
        let mut active_goals: Vec<SavingsGoal> = active_goals
            .into_iter()
            .filter(|g| g.current_amount < g.target_amount && !g.is_completed())
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
            if remaining_deduction <= Decimal::ZERO {
                break;
            }

            let needed_amount = goal.target_amount - goal.current_amount;
            let deduction_amount = needed_amount.min(remaining_deduction);

            self.treasury_service
                .transfer_to_savings(user_id, deduction_amount)
                .await?;

            deductions.push(SavingsDeduction {
                goal_id: goal.id,
                amount: deduction_amount,
                remaining_target: needed_amount - deduction_amount,
            });

            remaining_deduction -= deduction_amount;
        }

        Ok(deductions)
    }

    /// Manual savings contribution
    pub async fn manual_contribution(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Decimal,
    ) -> Result<(), FinanceError> {
        if amount <= Decimal::ZERO {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        let balance = self.treasury_service.get_user_balance(user_id).await?;
        if balance < amount {
            return Err(FinanceError::InsufficientFunds);
        }

        self.treasury_service
            .transfer_to_savings(user_id, amount)
            .await?;

        goal.add_contribution(amount);
        Ok(())
    }

    /// Withdraw from savings goal
    pub async fn withdraw_from_goal(
        &self,
        user_id: Uuid,
        goal: &mut SavingsGoal,
        amount: Decimal,
    ) -> Result<(), FinanceError> {
        if amount <= Decimal::ZERO {
            return Err(FinanceError::ValidationError("Amount must be positive".to_string()));
        }

        if amount > goal.current_amount {
            return Err(FinanceError::InsufficientFunds);
        }

        self.treasury_service
            .transfer_from_savings(user_id, amount)
            .await?;

        goal.withdraw(amount);
        Ok(())
    }

    /// Get savings progress report
    pub async fn get_savings_report(
        &self,
        goals: Vec<SavingsGoal>,
    ) -> Result<SavingsReport, FinanceError> {
        let total_target: Decimal = goals.iter().map(|g| g.target_amount).sum();
        let total_saved: Decimal = goals.iter().map(|g| g.current_amount).sum();
        
        let goals_by_category = self.categorize_goals(&goals);
        let completion_rate = if total_target > Decimal::ZERO {
            (total_saved / total_target) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let active_goals: Vec<&SavingsGoal> = goals.iter()
            .filter(|g| !g.is_completed())
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
                .entry(goal.category.clone())
                .or_insert(CategorySummary {
                    total_target: Decimal::ZERO,
                    total_saved: Decimal::ZERO,
                    goal_count: 0,
                });

            summary.total_target += goal.target_amount;
            summary.total_saved += goal.current_amount;
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
        let mut total_needed = Decimal::ZERO;
        let mut total_monthly = Decimal::ZERO;

        for goal in active_goals {
            let monthly_needed = self.calculate_monthly_savings(goal).await?;
            total_needed += goal.target_amount - goal.current_amount;
            total_monthly += monthly_needed;
            latest_date = latest_date.max(goal.target_date);
        }

        if total_monthly <= Decimal::ZERO {
            return Ok(Some(latest_date));
        }

        let months_needed = (total_needed / total_monthly).ceil().to_u64().unwrap_or(0);
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
                if percentage < Decimal::ZERO || percentage >
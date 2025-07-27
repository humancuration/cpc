//! Budget service for managing user budgets with UBI integration

use std::sync::Arc;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use cpc_core::finance::ubi::{UbiService, UbiStorage};
use cpc_core::finance::transactions::TransactionLedger;
use cpc_core::finance::{Money, Currency};

use crate::domain::models::{Budget, BudgetPeriod, FinanceError};
/// Trait for UBI service integration
#[async_trait::async_trait]
pub trait UbiServiceInterface: Send + Sync {
    async fn get_ubi_balance(&self, user_id: Uuid) -> Result<Money, FinanceError>;
    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Money, FinanceError>;
}
}

/// Implementation of UBI service interface
pub struct UbiServiceWrapper<S, L>
where
    S: UbiStorage,
    L: TransactionLedger,
{
    ubi_service: Arc<UbiService<S, L>>,
}

impl<S, L> UbiServiceWrapper<S, L>
where
    S: UbiStorage + Send + Sync + 'static,
    L: TransactionLedger + Send + Sync + 'static,
{
    pub fn new(ubi_service: Arc<UbiService<S, L>>) -> Self {
        Self { ubi_service }
    }
}
#[async_trait::async_trait]
impl<S, L> UbiServiceInterface for UbiServiceWrapper<S, L>
where
    S: UbiStorage + Send + Sync + 'static,
    L: TransactionLedger + Send + Sync + 'static,
{
    async fn get_ubi_balance(&self, user_id: Uuid) -> Result<Money, FinanceError> {
        let balance = self.ubi_service.get_ubi_balance(user_id);
        Ok(Money::new(balance, Currency::USD)) // Assuming USD for UBI
    }

    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Money, FinanceError> {
        // Calculate 30 days of UBI income
        let daily_amount = self.ubi_service.get_config()
            .map_err(|e| FinanceError::UbiServiceError(e.to_string()))?
            .daily_amount;
        
        let monthly_amount = daily_amount * Decimal::from(30);
        Ok(Money::new(monthly_amount, Currency::USD)) // Assuming USD for UBI
    }
}
}

/// Budget service for managing user budgets
pub struct BudgetService {
    ubi_service: Arc<dyn UbiServiceInterface>,
}

impl BudgetService {
    pub fn new(ubi_service: Arc<dyn UbiServiceInterface>) -> Self {
        /// Calculate monthly budget based on UBI income and user preferences
        pub async fn calculate_monthly_budget(
            &self,
            user_id: Uuid,
            category_percentages: Vec<(String, Decimal)>,
        ) -> Result<Vec<Budget>, FinanceError> {
            let monthly_income = self.ubi_service.get_monthly_ubi_income(user_id).await?;
            
            // Validate percentages sum to 100
            let total_percentage: Decimal = category_percentages.iter().map(|(_, pct)| *pct).sum();
            if (total_percentage - Decimal::from(100)).abs() > Decimal::new(1, 2) {
                return Err(FinanceError::InvalidAmount(
                    "Category percentages must sum to 100%".to_string()
                ));
            }
    
            let now = Utc::now();
            let start_of_month = now.date_naive().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap();
            let next_month = (start_of_month + chrono::Duration::days(32))
                .date()
                .with_day(1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();
            
            let mut budgets = Vec::new();
            
            for (category, percentage) in category_percentages {
                let allocated_amount = monthly_income.multiply(percentage / Decimal::from(100));
                
                let budget = Budget::new(
                    user_id,
                    category,
                    allocated_amount,
                    BudgetPeriod::Monthly,
                    DateTime::<Utc>::from_utc(start_of_month, Utc),
                    DateTime::<Utc>::from_utc(next_month, Utc),
                );
                
                budgets.push(budget);
            }
    
            Ok(budgets)
        }

        Ok(budgets)
    }

    /// Create a new budget
    pub async fn create_budget(
        &self,
        user_id: Uuid,
        category: String,
        allocated_amount: Money,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Budget, FinanceError> {
        if !allocated_amount.is_positive() {
            return Err(FinanceError::InvalidAmount(
                "Allocated amount must be positive".to_string()
            ));
        }

        if end_date <= start_date {
            return Err(FinanceError::InvalidDateRange);
        }

        let budget = Budget::new(user_id, category, allocated_amount, period, start_date, end_date);
        Ok(budget)
    }

    /// Update budget allocation
    pub async fn update_budget_allocation(
        &self,
        budget: &mut Budget,
        new_allocated_amount: Money,
    ) -> Result<(), FinanceError> {
        if !new_allocated_amount.is_positive() {
            return Err(FinanceError::InvalidAmount(
                "Allocated amount must be positive".to_string()
            ));
        }

        budget.allocated_amount = new_allocated_amount;
        Ok(())
    }

    /// Add expense to budget
    pub async fn add_expense_to_budget(
        &self,
        budget: &mut Budget,
        expense_amount: Money,
    ) -> Result<(), FinanceError> {
        if !expense_amount.is_positive() {
            return Err(FinanceError::InvalidAmount(
                "Expense amount must be positive".to_string()
            ));
        }

        let new_spent = budget.spent_amount.add(&expense_amount)
            .map_err(|e| FinanceError::InvalidAmount(e.to_string()))?;
        budget.spent_amount = new_spent;
        Ok(())
    }

    /// Get budget utilization analysis
    pub async fn analyze_budget_utilization(
        &self,
        budgets: Vec<Budget>,
    ) -> Result<BudgetAnalysis, FinanceError> {
        // For analysis, we'll convert to a common currency (USD) if needed
        // In a real implementation, we'd need proper currency conversion
        let mut total_allocated = Money::zero(Currency::USD);
        let mut total_spent = Money::zero(Currency::USD);
        
        for budget in &budgets {
            // Add allocated amounts (assuming same currency for simplicity)
            total_allocated = total_allocated.add(&budget.allocated_amount)
                .unwrap_or_else(|_| total_allocated.clone());
            
            // Add spent amounts (assuming same currency for simplicity)
            total_spent = total_spent.add(&budget.spent_amount)
                .unwrap_or_else(|_| total_spent.clone());
        }
        
        let total_remaining = total_allocated.subtract(&total_spent)
            .unwrap_or_else(|_| Money::zero(Currency::USD));

        let category_analysis = budgets.iter()
            .map(|b| CategoryAnalysis {
                category: b.category.clone(),
                allocated_amount: b.allocated_amount.clone(),
                spent_amount: b.spent_amount.clone(),
                remaining_amount: b.remaining_amount(),
                utilization_percentage: b.utilization_percentage(),
                is_over_budget: b.is_over_budget(),
            })
            .collect();

        // Calculate overall utilization percentage
        let overall_utilization = if total_allocated.is_zero() {
            Decimal::ZERO
        } else {
            let spent_amount = total_spent.amount;
            let allocated_amount = total_allocated.amount;
            if allocated_amount.is_zero() {
                Decimal::ZERO
            } else {
                (spent_amount / allocated_amount) * Decimal::from(100)
            }
        };

        Ok(BudgetAnalysis {
            total_allocated,
            total_spent,
            total_remaining,
            overall_utilization,
            category_analysis,
        })
    }
}

/// Budget analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAnalysis {
    pub total_allocated: Money,
    pub total_spent: Money,
    pub total_remaining: Money,
    pub overall_utilization: Decimal,
    pub category_analysis: Vec<CategoryAnalysis>,
}

/// Category-specific budget analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryAnalysis {
    pub category: String,
    pub allocated_amount: Money,
    pub spent_amount: Money,
    pub remaining_amount: Money,
    pub utilization_percentage: Decimal,
    pub is_over_budget: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::BudgetPeriod;
    use cpc_core::finance::ubi::InMemoryUbiStorage;
    use cpc_core::finance::transactions::InMemoryTransactionLedger;
    use cpc_core::finance::treasury::TreasuryService;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_calculate_monthly_budget() {
        let ledger = InMemoryTransactionLedger::new();
        let treasury_service = TreasuryService::new(ledger.clone());
        let storage = InMemoryUbiStorage::new();
        let ubi_service = Arc::new(UbiServiceWrapper::new(Arc::new(
            UbiService::new(storage, treasury_service, ledger)
        )));

        let budget_service = BudgetService::new(ubi_service);
        
        let user_id = Uuid::new_v4();
        let categories = vec![
            ("Housing".to_string(), dec!(40)),
            ("Food".to_string(), dec!(20)),
            ("Transport".to_string(), dec!(15)),
            ("Entertainment".to_string(), dec!(15)),
            ("Savings".to_string(), dec!(10)),
        ];

        let budgets = budget_service
            .calculate_monthly_budget(user_id, categories)
            .await
            .unwrap();

        assert_eq!(budgets.len(), 5);
        assert_eq!(budgets[0].category, "Housing");
        assert_eq!(budgets[0].allocated_amount, dec!(12)); // 40% of $30 monthly
    }

    #[tokio::test]
    async fn test_budget_utilization_analysis() {
        let ubi_service = Arc::new(MockUbiService);
        let budget_service = BudgetService::new(ubi_service);
        
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let budgets = vec![
            Budget {
                id: Uuid::new_v4(),
                user_id,
                category: "Food".to_string(),
                allocated_amount: dec!(100),
                spent_amount: dec!(75),
                period: BudgetPeriod::Monthly,
                start_date: now,
                end_date: now,
            },
            Budget {
                id: Uuid::new_v4(),
                user_id,
                category: "Transport".to_string(),
                allocated_amount: dec!(50),
                spent_amount: dec!(60),
                period: BudgetPeriod::Monthly,
                start_date: now,
                end_date: now,
            },
        ];

        let analysis = budget_service
            .analyze_budget_utilization(budgets)
            .await
            .unwrap();

        assert_eq!(analysis.total_allocated, dec!(150));
        assert_eq!(analysis.total_spent, dec!(135));
        assert_eq!(analysis.total_remaining, dec!(15));
        assert_eq!(analysis.category_analysis.len(), 2);
    }

    struct MockUbiService;

    #[async_trait::async_trait]
    impl UbiServiceInterface for MockUbiService {
        async fn get_ubi_balance(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
            Ok(dec!(100))
        }

        async fn get_monthly_ubi_income(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
            Ok(dec!(30))
        }
    }
}
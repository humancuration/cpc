//! Budget service for managing user budgets with UBI integration

use std::sync::Arc;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use cpc_core::finance::ubi::{UbiService, UbiStorage};
use cpc_core::finance::transactions::TransactionLedger;

use crate::domain::models::{Budget, BudgetPeriod, FinanceError};

/// Trait for UBI service integration
#[async_trait::async_trait]
pub trait UbiServiceInterface: Send + Sync {
    async fn get_ubi_balance(&self, user_id: Uuid) -> Result<Decimal, FinanceError>;
    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Decimal, FinanceError>;
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
    async fn get_ubi_balance(&self, user_id: Uuid) -> Result<Decimal, FinanceError> {
        Ok(self.ubi_service.get_ubi_balance(user_id))
    }

    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Decimal, FinanceError> {
        // Calculate 30 days of UBI income
        let daily_amount = self.ubi_service.get_config()
            .map_err(|e| FinanceError::UbiServiceError(e.to_string()))?
            .daily_amount;
        
        Ok(daily_amount * Decimal::from(30))
    }
}

/// Budget service for managing user budgets
pub struct BudgetService {
    ubi_service: Arc<dyn UbiServiceInterface>,
}

impl BudgetService {
    pub fn new(ubi_service: Arc<dyn UbiServiceInterface>) -> Self {
        Self { ubi_service }
    }

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
            let allocated_amount = (monthly_income * percentage) / Decimal::from(100);
            
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

    /// Create a new budget
    pub async fn create_budget(
        &self,
        user_id: Uuid,
        category: String,
        allocated_amount: Decimal,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Budget, FinanceError> {
        if allocated_amount <= Decimal::ZERO {
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
        new_allocated_amount: Decimal,
    ) -> Result<(), FinanceError> {
        if new_allocated_amount <= Decimal::ZERO {
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
        expense_amount: Decimal,
    ) -> Result<(), FinanceError> {
        if expense_amount <= Decimal::ZERO {
            return Err(FinanceError::InvalidAmount(
                "Expense amount must be positive".to_string()
            ));
        }

        budget.spent_amount += expense_amount;
        Ok(())
    }

    /// Get budget utilization analysis
    pub async fn analyze_budget_utilization(
        &self,
        budgets: Vec<Budget>,
    ) -> Result<BudgetAnalysis, FinanceError> {
        let total_allocated: Decimal = budgets.iter().map(|b| b.allocated_amount).sum();
        let total_spent: Decimal = budgets.iter().map(|b| b.spent_amount).sum();
        let total_remaining = total_allocated - total_spent;

        let category_analysis = budgets.iter()
            .map(|b| CategoryAnalysis {
                category: b.category.clone(),
                allocated_amount: b.allocated_amount,
                spent_amount: b.spent_amount,
                remaining_amount: b.remaining_amount(),
                utilization_percentage: b.utilization_percentage(),
                is_over_budget: b.is_over_budget(),
            })
            .collect();

        Ok(BudgetAnalysis {
            total_allocated,
            total_spent,
            total_remaining,
            overall_utilization: if total_allocated.is_zero() {
                Decimal::ZERO
            } else {
                (total_spent / total_allocated) * Decimal::from(100)
            },
            category_analysis,
        })
    }
}

/// Budget analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAnalysis {
    pub total_allocated: Decimal,
    pub total_spent: Decimal,
    pub total_remaining: Decimal,
    pub overall_utilization: Decimal,
    pub category_analysis: Vec<CategoryAnalysis>,
}

/// Category-specific budget analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryAnalysis {
    pub category: String,
    pub allocated_amount: Decimal,
    pub spent_amount: Decimal,
    pub remaining_amount: Decimal,
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
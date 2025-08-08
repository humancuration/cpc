use crate::domain::{Budget, Expense, SavingsGoal, Investment, Debt, Money, TimePeriod};
use crate::application::{BudgetService, ExpenseService, SavingsService, InvestmentService, DebtService};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use common_utils::financial::MonetaryValue;

/// Financial overview data structure
#[derive(Debug, Clone)]
pub struct FinancialOverview {
    pub net_worth: f64,
    pub monthly_cash_flow: f64,
    pub debt_to_income: f64,
    pub savings_rate: f64,
    pub investment_allocation: Vec<(String, f64)>, // (asset_class, percentage)
}

/// Dashboard data structure
#[derive(Debug, Clone)]
pub struct DashboardData {
    pub total_assets: Money,
    pub total_liabilities: Money,
    pub total_income: Money,
    pub total_expenses: Money,
    pub savings_goals_progress: Vec<(String, f64)>, // (goal_name, progress)
    pub investment_performance: Vec<(String, f64)>, // (investment_name, performance_percentage)
}

/// Central service that combines data from all financial apps for unified BI Dashboard
pub struct FinanceAggregator {
    budget_service: BudgetService,
    expense_service: ExpenseService,
    savings_service: SavingsService,
    investment_service: InvestmentService,
    debt_service: DebtService,
}

impl FinanceAggregator {
    pub fn new(
        budget_service: BudgetService,
        expense_service: ExpenseService,
        savings_service: SavingsService,
        investment_service: InvestmentService,
        debt_service: DebtService,
    ) -> Self {
        Self {
            budget_service,
            expense_service,
            savings_service,
            investment_service,
            debt_service,
        }
    }
    
    /// Get financial overview combining data from all financial domains
    pub fn get_financial_overview(&self) -> FinancialOverview {
        FinancialOverview {
            net_worth: self.calculate_net_worth(),
            monthly_cash_flow: self.calculate_cash_flow(),
            debt_to_income: self.calculate_debt_to_income(),
            savings_rate: self.calculate_savings_rate(),
            investment_allocation: self.get_investment_allocation(),
        }
    }
    
    /// Generate dashboard data combining data from all financial domains
    pub fn generate_dashboard_data(&self) -> DashboardData {
        DashboardData {
            total_assets: self.calculate_total_assets(),
            total_liabilities: self.calculate_total_liabilities(),
            total_income: self.calculate_total_income(),
            total_expenses: self.calculate_total_expenses(),
            savings_goals_progress: self.get_savings_goals_progress(),
            investment_performance: self.get_investment_performance(),
        }
    }
    /// Calculate net worth (assets - liabilities) using high-precision fixed-point arithmetic
    fn calculate_net_worth(&self) -> f64 {
        let total_assets_fixed = fixed::types::I64F64::from_num(self.calculate_total_assets().amount);
        let total_liabilities_fixed = fixed::types::I64F64::from_num(self.calculate_total_liabilities().amount);
        let net_worth_fixed = total_assets_fixed - total_liabilities_fixed;
        net_worth_fixed.to_num::<f64>()
    }
    
    /// Calculate monthly cash flow (income - expenses) using high-precision fixed-point arithmetic
    fn calculate_cash_flow(&self) -> f64 {
        let total_income_fixed = fixed::types::I64F64::from_num(self.calculate_total_income().amount);
        let total_expenses_fixed = fixed::types::I64F64::from_num(self.calculate_total_expenses().amount);
        let cash_flow_fixed = total_income_fixed - total_expenses_fixed;
        cash_flow_fixed.to_num::<f64>()
    }
    
    /// Calculate debt to income ratio using high-precision fixed-point arithmetic
    fn calculate_debt_to_income(&self) -> f64 {
        let total_debt_payments = self.debt_service.get_total_minimum_monthly_payments().amount;
        let total_income = self.calculate_total_income().amount;
        
        // Use fixed-point arithmetic for precise calculation
        let debt_payments_fixed = fixed::types::I64F64::from_num(total_debt_payments);
        let income_fixed = fixed::types::I64F64::from_num(total_income);
        
        if income_fixed > fixed::types::I64F64::from_num(0.0) {
            let ratio_fixed = debt_payments_fixed / income_fixed;
            ratio_fixed.to_num::<f64>()
        } else {
            0.0
        }
    }
    }
    
    /// Calculate savings rate using high-precision fixed-point arithmetic
    fn calculate_savings_rate(&self) -> f64 {
        let total_income = self.calculate_total_income().amount;
        let total_savings = self.savings_service.get_total_current_amount().amount;
        
        // Use fixed-point arithmetic for precise calculation
        let income_fixed = fixed::types::I64F64::from_num(total_income);
        let savings_fixed = fixed::types::I64F64::from_num(total_savings);
        
        if income_fixed > fixed::types::I64F64::from_num(0.0) {
            let rate_fixed = savings_fixed / income_fixed;
            rate_fixed.to_num::<f64>()
        } else {
            0.0
        }
    }
    
    /// Get investment allocation by asset class
    fn get_investment_allocation(&self) -> Vec<(String, f64)> {
        // This is a simplified implementation
        // In a real implementation, we would calculate actual percentages
        vec![
            ("Stocks".to_string(), 0.6),
            ("Bonds".to_string(), 0.3),
            ("Crypto".to_string(), 0.1),
        ]
    }
    
    /// Calculate total assets using high-precision fixed-point arithmetic
    fn calculate_total_assets(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let investment_value = self.investment_service.get_total_investment_value().amount;
        let savings_value = self.savings_service.get_total_current_amount().amount;
        
        // Use fixed-point arithmetic for precise calculation
        let investment_fixed = fixed::types::I64F64::from_num(investment_value);
        let savings_fixed = fixed::types::I64F64::from_num(savings_value);
        let total_fixed = investment_fixed + savings_fixed;
        
        let currency = "USD".to_string();
        Money::new(total_fixed.to_num::<f64>(), &currency)
    }
    
    /// Calculate total liabilities using high-precision fixed-point arithmetic
    fn calculate_total_liabilities(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let debt_balance = self.debt_service.get_total_debt_balance().amount;
        
        // Use fixed-point arithmetic for precise calculation
        let debt_fixed = fixed::types::I64F64::from_num(debt_balance);
        
        let currency = "USD".to_string();
        Money::new(debt_fixed.to_num::<f64>(), &currency)
    }
    
    /// Calculate total income (simplified implementation)
    fn calculate_total_income(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to calculate actual income
        let currency = "USD".to_string();
        Money::new(5000.0, &currency) // Placeholder value
    }
    
    /// Calculate total expenses using high-precision fixed-point arithmetic
    fn calculate_total_expenses(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let currency = "USD".to_string();
        
        // Sum up all budgeted amounts for the current period
        let now = Utc::now();
        let period = TimePeriod::new(
            now - chrono::Duration::days(30),
            now,
        );
        
        let budgeted_amount = self.budget_service.get_total_budgeted_amount(&period).amount;
        
        // Use fixed-point arithmetic for precise calculation
        let budgeted_fixed = fixed::types::I64F64::from_num(budgeted_amount);
        
        Money::new(budgeted_fixed.to_num::<f64>(), &currency)
    }
    
    /// Get savings goals progress
    fn get_savings_goals_progress(&self) -> Vec<(String, f64)> {
        self.savings_service
            .get_all_savings_goals()
            .iter()
            .map(|goal| (goal.name.clone(), goal.progress))
            .collect()
    }
    
    /// Get investment performance
    fn get_investment_performance(&self) -> Vec<(String, f64)> {
        self.investment_service
            .get_all_investments()
            .iter()
            .map(|investment| (investment.name.clone(), investment.profit_loss_percentage()))
            .collect()
    }
}
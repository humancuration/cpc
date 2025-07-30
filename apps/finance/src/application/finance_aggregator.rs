use crate::domain::{Budget, Expense, SavingsGoal, Investment, Debt, Money, TimePeriod};
use crate::application::{BudgetService, ExpenseService, SavingsService, InvestmentService, DebtService};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    
    /// Calculate net worth (assets - liabilities)
    fn calculate_net_worth(&self) -> f64 {
        let total_assets = self.calculate_total_assets().amount;
        let total_liabilities = self.calculate_total_liabilities().amount;
        total_assets - total_liabilities
    }
    
    /// Calculate monthly cash flow (income - expenses)
    fn calculate_cash_flow(&self) -> f64 {
        let total_income = self.calculate_total_income().amount;
        let total_expenses = self.calculate_total_expenses().amount;
        total_income - total_expenses
    }
    
    /// Calculate debt to income ratio
    fn calculate_debt_to_income(&self) -> f64 {
        let total_debt_payments = self.debt_service.get_total_minimum_monthly_payments().amount;
        let total_income = self.calculate_total_income().amount;
        
        if total_income > 0.0 {
            total_debt_payments / total_income
        } else {
            0.0
        }
    }
    
    /// Calculate savings rate
    fn calculate_savings_rate(&self) -> f64 {
        let total_income = self.calculate_total_income().amount;
        let total_savings = self.savings_service.get_total_current_amount().amount;
        
        if total_income > 0.0 {
            total_savings / total_income
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
    
    /// Calculate total assets
    fn calculate_total_assets(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let investment_value = self.investment_service.get_total_investment_value().amount;
        let savings_value = self.savings_service.get_total_current_amount().amount;
        let currency = "USD".to_string();
        
        Money::new(investment_value + savings_value, &currency)
    }
    
    /// Calculate total liabilities
    fn calculate_total_liabilities(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let debt_balance = self.debt_service.get_total_debt_balance().amount;
        let currency = "USD".to_string();
        
        Money::new(debt_balance, &currency)
    }
    
    /// Calculate total income (simplified implementation)
    fn calculate_total_income(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to calculate actual income
        let currency = "USD".to_string();
        Money::new(5000.0, &currency) // Placeholder value
    }
    
    /// Calculate total expenses
    fn calculate_total_expenses(&self) -> Money {
        // For simplicity, we'll assume all values are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_expenses = 0.0;
        let currency = "USD".to_string();
        
        // Sum up all budgeted amounts for the current period
        let now = Utc::now();
        let period = TimePeriod::new(
            now - chrono::Duration::days(30),
            now,
        );
        
        total_expenses += self.budget_service.get_total_budgeted_amount(&period).amount;
        
        Money::new(total_expenses, &currency)
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
//! Accounting dashboard for business intelligence
//! Provides real-time financial metrics and KPI calculations

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::accounting::Money;
use crate::accounting::PeriodType;

/// Core accounting dashboard data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountingDashboard {
    pub current_assets: Money,
    pub liabilities: Money,
    pub equity: Money,
    pub revenue_30d: Money,
    pub expenses_30d: Money,
    pub profit_margin: f32,
    pub key_metrics: HashMap<String, Metric>,
}

impl AccountingDashboard {
    /// Creates a new empty dashboard with zero values
    pub fn new(currency: &str) -> Self {
        Self {
            current_assets: Money::zero(currency),
            liabilities: Money::zero(currency),
            equity: Money::zero(currency),
            revenue_30d: Money::zero(currency),
            expenses_30d: Money::zero(currency),
            profit_margin: 0.0,
            key_metrics: HashMap::new(),
        }
    }

    /// Calculates the working capital (current assets - liabilities)
    pub fn working_capital(&self) -> Money {
        Money {
            amount: self.current_assets.amount - self.liabilities.amount,
            currency: self.current_assets.currency.clone(),
        }
    }

    /// Calculates net income (revenue - expenses)
    pub fn net_income(&self) -> Money {
        Money {
            amount: self.revenue_30d.amount - self.expenses_30d.amount,
            currency: self.revenue_30d.currency.clone(),
        }
    }

    /// Updates key metrics based on current financial data
    pub fn update_key_metrics(&mut self) {
        // Calculate current ratio
        if self.liabilities.amount > 0 {
            let current_ratio = self.current_assets.amount as f32 / self.liabilities.amount as f32;
            self.key_metrics.insert(
                "current_ratio".to_string(),
                Metric::CurrentRatio(current_ratio),
            );
        }

        // Calculate quick ratio (assuming no inventory for simplicity)
        if self.liabilities.amount > 0 {
            let quick_ratio = self.current_assets.amount as f32 / self.liabilities.amount as f32;
            self.key_metrics.insert(
                "quick_ratio".to_string(),
                Metric::QuickRatio(quick_ratio),
            );
        }

        // Calculate debt-to-equity ratio
        if self.equity.amount > 0 {
            let debt_to_equity = self.liabilities.amount as f32 / self.equity.amount as f32;
            self.key_metrics.insert(
                "debt_to_equity".to_string(),
                Metric::DebtToEquity(debt_to_equity),
            );
        }

        // Update profit margin
        if self.revenue_30d.amount > 0 {
            self.profit_margin = ((self.revenue_30d.amount - self.expenses_30d.amount) as f32 
                / self.revenue_30d.amount as f32) * 100.0;
        }
    }
}

/// Financial metrics for dashboard display
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Metric {
    CurrentRatio(f32),
    QuickRatio(f32),
    DebtToEquity(f32),
}

impl Metric {
    /// Returns the metric name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Metric::CurrentRatio(_) => "Current Ratio",
            Metric::QuickRatio(_) => "Quick Ratio",
            Metric::DebtToEquity(_) => "Debt-to-Equity",
        }
    }

    /// Returns the metric value
    pub fn value(&self) -> f32 {
        match self {
            Metric::CurrentRatio(v) => *v,
            Metric::QuickRatio(v) => *v,
            Metric::DebtToEquity(v) => *v,
        }
    }

    /// Returns the metric as a formatted string
    pub fn formatted(&self) -> String {
        match self {
            Metric::CurrentRatio(v) => format!("{:.2}:1", v),
            Metric::QuickRatio(v) => format!("{:.2}:1", v),
            Metric::DebtToEquity(v) => format!("{:.2}", v),
        }
    }
}

/// Service function to retrieve dashboard data
pub async fn get_dashboard_data(
    org_id: Uuid,
    period: PeriodType,
) -> Result<AccountingDashboard, crate::accounting::AccountingError> {
    // TODO: Implement actual data retrieval from database
    // For now, return mock data
    
    let mut dashboard = AccountingDashboard::new("USD");
    
    // Mock data for demonstration
    dashboard.current_assets = Money::new(150000.0, "USD");
    dashboard.liabilities = Money::new(75000.0, "USD");
    dashboard.equity = Money::new(75000.0, "USD");
    dashboard.revenue_30d = Money::new(50000.0, "USD");
    dashboard.expenses_30d = Money::new(35000.0, "USD");
    
    dashboard.update_key_metrics();
    
    Ok(dashboard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = AccountingDashboard::new("USD");
        assert_eq!(dashboard.current_assets.amount, 0);
        assert_eq!(dashboard.liabilities.amount, 0);
        assert_eq!(dashboard.equity.amount, 0);
        assert_eq!(dashboard.key_metrics.len(), 0);
    }

    #[test]
    fn test_working_capital_calculation() {
        let mut dashboard = AccountingDashboard::new("USD");
        dashboard.current_assets = Money::new(100000.0, "USD");
        dashboard.liabilities = Money::new(40000.0, "USD");
        
        let working_capital = dashboard.working_capital();
        assert_eq!(working_capital.amount, 60000);
    }

    #[test]
    fn test_net_income_calculation() {
        let mut dashboard = AccountingDashboard::new("USD");
        dashboard.revenue_30d = Money::new(80000.0, "USD");
        dashboard.expenses_30d = Money::new(50000.0, "USD");
        
        let net_income = dashboard.net_income();
        assert_eq!(net_income.amount, 30000);
    }

    #[test]
    fn test_metric_formatting() {
        let metric = Metric::CurrentRatio(1.75);
        assert_eq!(metric.name(), "Current Ratio");
        assert_eq!(metric.value(), 1.75);
        assert_eq!(metric.formatted(), "1.75:1");
    }
}
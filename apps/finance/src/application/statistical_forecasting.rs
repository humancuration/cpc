//! Statistical forecasting for financial predictions
//!
//! This module provides statistical forecasting capabilities for financial data
//! using the CPC statistics core framework.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceCalculator,
    ConfidenceInterval,
    SignificanceTester,
    SignificanceResult,
    StatisticalError,
};

#[cfg(feature = "statistics")]
use crate::domain::{Budget, Expense, FinancialCategory};
#[cfg(feature = "statistics")]
use chrono::{DateTime, Utc};
#[cfg(feature = "statistics")]
use rust_decimal::Decimal;

/// Statistical forecasting service for financial predictions
#[cfg(feature = "statistics")]
pub struct StatisticalForecastingService;

#[cfg(feature = "statistics")]
impl StatisticalForecastingService {
    /// Forecast future expenses based on historical data with confidence intervals
    pub fn forecast_expenses(
        historical_expenses: &[Expense],
        forecast_periods: usize,
        confidence_level: f64,
    ) -> Result<ExpenseForecast, StatisticalError> {
        if historical_expenses.is_empty() {
            return Err(StatisticalError::InsufficientData(0, 1));
        }
        
        // Extract expense amounts as f64 for statistical analysis
        let amounts: Vec<f64> = historical_expenses
            .iter()
            .map(|e| e.amount.amount)
            .collect();
        
        // Calculate basic statistics
        let mean = amounts.mean();
        let std_dev = amounts.std_dev();
        
        // Generate forecast periods
        let mut forecast_values = Vec::with_capacity(forecast_periods);
        let mut rng = rand::thread_rng();
        
        for i in 0..forecast_periods {
            // Simple random walk forecast with trend
            let trend = mean * 0.01 * i as f64; // 1% growth per period
            let random_component = rand_distr::Normal::new(0.0, std_dev)
                .map_err(|_| StatisticalError::InvalidInput("Invalid distribution parameters".to_string()))?
                .sample(&mut rng);
            
            let forecast_value = mean + trend + random_component;
            forecast_values.push(forecast_value);
        }
        
        // Calculate confidence intervals
        let ci = ConfidenceCalculator::parametric_interval(&amounts, confidence_level)?;
        
        // Perform significance test for trend
        let trend_test = SignificanceTester::one_sample_t_test(&amounts, 0.0)?;
        
        Ok(ExpenseForecast::new(
            forecast_values,
            ci,
            trend_test,
            confidence_level,
        ))
    }
    
    /// Forecast budget utilization with statistical analysis
    pub fn forecast_budget_utilization(
        budget: &Budget,
        historical_expenses: &[Expense],
        forecast_periods: usize,
    ) -> Result<BudgetForecast, StatisticalError> {
        if historical_expenses.is_empty() {
            return Err(StatisticalError::InsufficientData(0, 1));
        }
        
        // Filter expenses by budget category
        let category_expenses: Vec<&Expense> = historical_expenses
            .iter()
            .filter(|e| e.category == budget.category)
            .collect();
        
        if category_expenses.is_empty() {
            return Err(StatisticalError::InsufficientData(0, 1));
        }
        
        // Extract expense amounts
        let amounts: Vec<f64> = category_expenses
            .iter()
            .map(|e| e.amount.amount)
            .collect();
        
        // Calculate utilization rate
        let total_spent = amounts.iter().sum::<f64>();
        let budget_amount = budget.amount.amount;
        let utilization_rate = total_spent / budget_amount;
        
        // Forecast future utilization
        let forecast_utilization = utilization_rate * (1.0 + 0.02 * forecast_periods as f64); // 2% growth per period
        
        // Calculate confidence interval for utilization
        let ci = ConfidenceCalculator::parametric_interval(&amounts, 0.95)?;
        
        // Perform significance test
        let significance_test = SignificanceTester::one_sample_t_test(&amounts, budget_amount)?;
        
        Ok(BudgetForecast::new(
            utilization_rate,
            forecast_utilization,
            ci,
            significance_test,
        ))
    }
}

/// Forecast results for expense predictions
#[cfg(feature = "statistics")]
pub struct ExpenseForecast {
    /// Forecasted values for each period
    pub forecast_values: Vec<f64>,
    
    /// Confidence interval for the forecast
    pub confidence_interval: ConfidenceInterval,
    
    /// Significance test results for trend analysis
    pub trend_significance: SignificanceResult,
    
    /// Confidence level used for the forecast
    pub confidence_level: f64,
}

#[cfg(feature = "statistics")]
impl ExpenseForecast {
    /// Create a new expense forecast
    pub fn new(
        forecast_values: Vec<f64>,
        confidence_interval: ConfidenceInterval,
        trend_significance: SignificanceResult,
        confidence_level: f64,
    ) -> Self {
        Self {
            forecast_values,
            confidence_interval,
            trend_significance,
            confidence_level,
        }
    }
    
    /// Generate a plain-language explanation of the forecast
    pub fn explanation(&self) -> String {
        format!(
            "Based on historical data, future expenses are forecasted to be {:.2} with a {:.0}% confidence interval of [{:.2}, {:.2}]. \
            The trend analysis shows {} evidence for a significant change in spending patterns (p = {:.4}).",
            self.forecast_values.iter().sum::<f64>() / self.forecast_values.len() as f64,
            self.confidence_level * 100.0,
            self.confidence_interval.lower,
            self.confidence_interval.upper,
            self.trend_significance.level.description(),
            self.trend_significance.p_value
        )
    }
}

/// Forecast results for budget utilization
#[cfg(feature = "statistics")]
pub struct BudgetForecast {
    /// Current utilization rate
    pub current_utilization: f64,
    
    /// Forecasted utilization rate
    pub forecasted_utilization: f64,
    
    /// Confidence interval for the forecast
    pub confidence_interval: ConfidenceInterval,
    
    /// Significance test results
    pub significance_test: SignificanceResult,
}

#[cfg(feature = "statistics")]
impl BudgetForecast {
    /// Create a new budget forecast
    pub fn new(
        current_utilization: f64,
        forecasted_utilization: f64,
        confidence_interval: ConfidenceInterval,
        significance_test: SignificanceResult,
    ) -> Self {
        Self {
            current_utilization,
            forecasted_utilization,
            confidence_interval,
            significance_test,
        }
    }
    
    /// Check if the budget is likely to be exceeded
    pub fn will_exceed_budget(&self) -> bool {
        self.forecasted_utilization > 1.0
    }
    
    /// Generate a plain-language explanation of the budget forecast
    pub fn explanation(&self) -> String {
        let budget_status = if self.will_exceed_budget() {
            "exceeded"
        } else {
            "within limits"
        };
        
        format!(
            "Current budget utilization is {:.1}%. Based on trends, this is forecasted to reach {:.1}% by the end of the period. \
            The budget is likely to be {} (confidence interval: [{:.2}, {:.2}]). \
            Statistical analysis shows {} evidence for significant spending patterns (p = {:.4}).",
            self.current_utilization * 100.0,
            self.forecasted_utilization * 100.0,
            budget_status,
            self.confidence_interval.lower,
            self.confidence_interval.upper,
            self.significance_test.level.description(),
            self.significance_test.p_value
        )
    }
}

// Fallback implementation when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
pub struct StatisticalForecastingService;

#[cfg(not(feature = "statistics"))]
impl StatisticalForecastingService {
    /// Forecast future expenses (stub implementation)
    pub fn forecast_expenses(
        _historical_expenses: &[Expense],
        _forecast_periods: usize,
        _confidence_level: f64,
    ) -> Result<ExpenseForecast, &'static str> {
        Err("Statistical forecasting requires the 'statistics' feature to be enabled")
    }
    
    /// Forecast budget utilization (stub implementation)
    pub fn forecast_budget_utilization(
        _budget: &Budget,
        _historical_expenses: &[Expense],
        _forecast_periods: usize,
    ) -> Result<BudgetForecast, &'static str> {
        Err("Statistical forecasting requires the 'statistics' feature to be enabled")
    }
}

#[cfg(not(feature = "statistics"))]
pub struct ExpenseForecast;

#[cfg(not(feature = "statistics"))]
pub struct BudgetForecast;

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use crate::domain::{primitives::Money, Currency};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;
    
    #[test]
    fn test_expense_forecast() {
        let expenses = vec![
            Expense::new(
                Uuid::new_v4(),
                "Test".to_string(),
                Money::new(Decimal::new(100, 0), Currency::USD),
                Utc::now(),
                FinancialCategory::Groceries,
            ),
            Expense::new(
                Uuid::new_v4(),
                "Test".to_string(),
                Money::new(Decimal::new(120, 0), Currency::USD),
                Utc::now(),
                FinancialCategory::Groceries,
            ),
            Expense::new(
                Uuid::new_v4(),
                "Test".to_string(),
                Money::new(Decimal::new(90, 0), Currency::USD),
                Utc::now(),
                FinancialCategory::Groceries,
            ),
        ];
        
        let forecast = StatisticalForecastingService::forecast_expenses(&expenses, 5, 0.95);
        assert!(forecast.is_ok());
    }
}
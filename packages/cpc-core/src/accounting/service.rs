//! Accounting service layer
//! Provides high-level business logic for accounting operations

use uuid::Uuid;
use crate::accounting::{AccountingError, PeriodType};
use crate::accounting::dashboard::{AccountingDashboard, get_dashboard_data};
use crate::accounting::transaction::Transaction;
use crate::business::{FinancialForecast, ForecastParameters};

/// High-level accounting service
pub struct AccountingService;

impl AccountingService {
    /// Get dashboard data for an organization
    pub async fn get_dashboard(
        org_id: Uuid,
        period: PeriodType,
    ) -> Result<AccountingDashboard, AccountingError> {
        get_dashboard_data(org_id, period).await
    }

    /// Create a financial forecast with the given historical transactions
    pub fn create_forecast(
        &self,
        historical_transactions: Vec<Transaction>,
    ) -> Result<FinancialForecast, AccountingError> {
        let params = ForecastParameters::default();
        self.create_forecast_with_params(historical_transactions, params)
    }

    /// Create a financial forecast with custom parameters
    pub fn create_forecast_with_params(
        &self,
        historical_transactions: Vec<Transaction>,
        parameters: ForecastParameters,
    ) -> Result<FinancialForecast, AccountingError> {
        let mut forecast = FinancialForecast::new(parameters);
        
        // Add base scenario if none exists
        if forecast.scenarios.is_empty() {
            forecast.add_scenario("base".to_string(), forecast.base_parameters.clone());
        }
        
        // Project cash flow for the base scenario
        forecast.project_cash_flow("base", &historical_transactions)
            .map_err(|e| AccountingError::ValidationError(e.to_string()))?;
        
        Ok(forecast)
    }

    /// Get available forecast scenarios
    pub fn get_available_forecast_scenarios(&self) -> Result<Vec<String>, AccountingError> {
        // For now, return common scenario templates
        Ok(vec![
            "base".to_string(),
            "conservative".to_string(),
            "aggressive".to_string(),
            "worst_case".to_string(),
            "best_case".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_dashboard() {
        let org_id = Uuid::new_v4();
        let result = AccountingService::get_dashboard(org_id, crate::accounting::PeriodType::Monthly).await;
        assert!(result.is_ok());
        
        let dashboard = result.unwrap();
        assert_eq!(dashboard.current_assets.currency, "USD");
        assert!(dashboard.key_metrics.contains_key("current_ratio"));
    }
}
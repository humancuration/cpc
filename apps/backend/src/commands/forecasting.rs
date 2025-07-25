//! Financial forecasting Tauri commands
//! Implements command bindings for financial forecasting functionality

use cpc_core::business::financial_forecasting::{FinancialForecast, ForecastParameters, ForecastError};
use cpc_core::accounting::transaction::Transaction;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tauri::command;

/// Chart data point for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPoint {
    pub date: NaiveDate,
    pub value: f64,
    pub scenario: String,
}

/// Extension trait for FinancialForecast to convert to chart data
pub trait ToChartData {
    fn to_chart_data(&self) -> Vec<ChartPoint>;
}

impl ToChartData for FinancialForecast {
    /// Convert forecast data to chart-compatible format
    fn to_chart_data(&self) -> Vec<ChartPoint> {
        self.scenarios
            .iter()
            .flat_map(|scenario| {
                scenario.projections.iter().map(|projection| ChartPoint {
                    date: projection.date,
                    value: projection.net_cash_flow,
                    scenario: scenario.name.clone(),
                })
            })
            .collect()
    }
}

/// Run financial forecasting based on parameters and historical data
#[command]
pub async fn run_forecast(
    params: ForecastParameters,
    historical: Vec<Transaction>,
) -> Result<FinancialForecast, ForecastError> {
    // Create a new forecast with the provided parameters
    let mut forecast = FinancialForecast::new(params);
    
    // Add base scenario if none exists
    if forecast.scenarios.is_empty() {
        forecast.add_scenario("base".to_string(), forecast.base_parameters.clone());
    }
    
    // Project cash flow for the base scenario
    forecast.project_cash_flow("base", &historical)?;
    
    Ok(forecast)
}

/// Additional command for running multiple scenarios
#[command]
pub async fn run_forecast_with_scenarios(
    params: ForecastParameters,
    historical: Vec<Transaction>,
    scenarios: Vec<(String, ForecastParameters)>,
) -> Result<FinancialForecast, ForecastError> {
    let mut forecast = FinancialForecast::new(params);
    
    // Add all scenarios
    for (name, scenario_params) in scenarios {
        forecast.add_scenario(name, scenario_params);
        forecast.project_cash_flow(&name, &historical)?;
    }
    
    Ok(forecast)
}

/// Get chart data for a forecast
#[command]
pub async fn get_forecast_chart_data(
    forecast: FinancialForecast,
) -> Result<Vec<ChartPoint>, ForecastError> {
    Ok(forecast.to_chart_data())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::accounting::transaction::{Transaction, JournalEntry};
    use cpc_core::accounting::money::Money;
    use cpc_core::accounting::transaction::EntryType;
    use chrono::Duration;
    use uuid::Uuid;
    use chrono::Utc;

    fn create_test_transaction(date: chrono::DateTime<Utc>, amount: f64) -> Transaction {
        Transaction {
            id: Uuid::new_v4(),
            date,
            description: "Test transaction".to_string(),
            journal_entries: vec![
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(amount),
                    entry_type: EntryType::Debit,
                },
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(amount),
                    entry_type: EntryType::Credit,
                },
            ],
            reference: None,
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_run_forecast() {
        let params = ForecastParameters {
            start_date: chrono::Utc::now().date_naive(),
            end_date: chrono::Utc::now().date_naive() + Duration::days(90),
            interval: "monthly".to_string(),
            scenario_parameters: std::collections::HashMap::new(),
        };

        let historical = vec![
            create_test_transaction(Utc::now() - Duration::days(30), 1000.0),
            create_test_transaction(Utc::now() - Duration::days(60), 1200.0),
            create_test_transaction(Utc::now() - Duration::days(90), 800.0),
        ];

        let result = run_forecast(params, historical).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_to_chart_data() {
        let params = ForecastParameters {
            start_date: chrono::Utc::now().date_naive(),
            end_date: chrono::Utc::now().date_naive() + Duration::days(30),
            interval: "monthly".to_string(),
            scenario_parameters: std::collections::HashMap::new(),
        };

        let mut forecast = FinancialForecast::new(params);
        forecast.add_scenario("test".to_string(), forecast.base_parameters.clone());
        
        // This would normally require historical data, but we can test the conversion
        let chart_data = forecast.to_chart_data();
        assert!(chart_data.is_empty()); // Empty since no projections yet
    }
}
use cpc_core::{accounting::service::AccountingService, business::ForecastParameters, accounting::transaction::Transaction};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Data structure for chart points used by frontend visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPoint {
    pub date: chrono::DateTime<chrono::Utc>,
    pub value: rust_decimal::Decimal,
    pub scenario: String,
}

/// Request structure for running forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunForecastRequest {
    pub parameters: ForecastParameters,
    pub historical_transactions: Vec<Transaction>,
}

/// Response structure for forecast results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub scenarios: Vec<ScenarioResult>,
    pub summary: ForecastSummary,
}

/// Individual scenario result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub name: String,
    pub projections: Vec<ProjectionPoint>,
    pub confidence_interval: Option<(rust_decimal::Decimal, rust_decimal::Decimal)>,
}

/// Individual projection point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionPoint {
    pub date: chrono::DateTime<chrono::Utc>,
    pub net_cash_flow: rust_decimal::Decimal,
    pub total_revenue: rust_decimal::Decimal,
    pub total_expenses: rust_decimal::Decimal,
    pub confidence: Option<f64>,
}

/// Forecast summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastSummary {
    pub total_projected_revenue: rust_decimal::Decimal,
    pub total_projected_expenses: rust_decimal::Decimal,
    pub net_projected_cash_flow: rust_decimal::Decimal,
    pub risk_score: f64,
    pub confidence_level: f64,
}

/// Run a financial forecast with the given parameters and historical data
#[tauri::command]
pub async fn run_forecast(
    request: RunForecastRequest,
    service: State<'_, Arc<RwLock<AccountingService>>>,
) -> Result<ForecastResponse, String> {
    let service = service.read().await;
    
    // Create forecast using the accounting service
    let forecast = service
        .create_forecast_with_params(request.historical_transactions, request.parameters)
        .map_err(|e| format!("Failed to create forecast: {}", e))?;
    
    // Convert to response format
    let scenarios = forecast.scenarios
        .into_iter()
        .map(|scenario| {
            let projections = scenario.projections
                .into_iter()
                .map(|proj| ProjectionPoint {
                    date: proj.date,
                    net_cash_flow: proj.net_cash_flow,
                    total_revenue: proj.total_revenue,
                    total_expenses: proj.total_expenses,
                    confidence: proj.confidence,
                })
                .collect();
                
            ScenarioResult {
                name: scenario.name,
                projections,
                confidence_interval: scenario.confidence_interval,
            }
        })
        .collect();
    
    let summary = ForecastSummary {
        total_projected_revenue: forecast.summary.total_projected_revenue,
        total_projected_expenses: forecast.summary.total_projected_expenses,
        net_projected_cash_flow: forecast.summary.net_projected_cash_flow,
        risk_score: forecast.summary.risk_score,
        confidence_level: forecast.summary.confidence_level,
    };
    
    Ok(ForecastResponse {
        scenarios,
        summary,
    })
}

/// Get forecast data formatted for chart visualization
#[tauri::command]
pub async fn get_forecast_chart_data(
    request: RunForecastRequest,
    service: State<'_, Arc<RwLock<AccountingService>>>,
) -> Result<Vec<ChartPoint>, String> {
    let forecast_response = run_forecast(request, service).await?;
    
    let chart_data: Vec<ChartPoint> = forecast_response
        .scenarios
        .into_iter()
        .flat_map(|scenario| {
            scenario.projections.into_iter().map(move |projection| ChartPoint {
                date: projection.date,
                value: projection.net_cash_flow,
                scenario: scenario.name.clone(),
            })
        })
        .collect();
    
    Ok(chart_data)
}

/// Get available forecast scenarios
#[tauri::command]
pub async fn get_forecast_scenarios(
    service: State<'_, Arc<RwLock<AccountingService>>>,
) -> Result<Vec<String>, String> {
    let service = service.read().await;
    let scenarios = service.get_available_forecast_scenarios()
        .map_err(|e| format!("Failed to get scenarios: {}", e))?;
    
    Ok(scenarios)
}

/// Get forecast summary for dashboard
#[tauri::command]
pub async fn get_forecast_summary(
    historical_transactions: Vec<Transaction>,
    service: State<'_, Arc<RwLock<AccountingService>>>,
) -> Result<ForecastSummary, String> {
    let service = service.read().await;
    
    // Use default parameters for quick summary
    let default_params = ForecastParameters::default();
    let forecast = service
        .create_forecast_with_params(historical_transactions, default_params)
        .map_err(|e| format!("Failed to create forecast summary: {}", e))?;
    
    Ok(ForecastSummary {
        total_projected_revenue: forecast.summary.total_projected_revenue,
        total_projected_expenses: forecast.summary.total_projected_expenses,
        net_projected_cash_flow: forecast.summary.net_projected_cash_flow,
        risk_score: forecast.summary.risk_score,
        confidence_level: forecast.summary.confidence_level,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::accounting::{Transaction, TransactionType};
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_run_forecast_command() {
        // Create mock service
        let service = Arc::new(RwLock::new(AccountingService::new()));
        
        // Create test data
        let historical = vec![
            Transaction {
                id: uuid::Uuid::new_v4(),
                date: chrono::Utc::now(),
                description: "Test income".to_string(),
                journal_entries: vec![
                    crate::accounting::transaction::JournalEntry {
                        account_id: uuid::Uuid::new_v4(),
                        entry_type: crate::accounting::transaction::EntryType::Debit,
                        amount: crate::accounting::money::Money::new(1000.0, "USD"),
                        description: Some("Cash".to_string()),
                    },
                    crate::accounting::transaction::JournalEntry {
                        account_id: uuid::Uuid::new_v4(),
                        entry_type: crate::accounting::transaction::EntryType::Credit,
                        amount: crate::accounting::money::Money::new(1000.0, "USD"),
                        description: Some("Revenue".to_string()),
                    }
                ],
                reference: None,
                tags: vec!["test".to_string()],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];
        
        let params = ForecastParameters {
            start_date: chrono::Utc::now().date_naive(),
            end_date: chrono::Utc::now().date_naive() + chrono::Duration::days(30),
            interval: "monthly".to_string(),
            scenario_parameters: std::collections::HashMap::new(),
        };
        
        let request = RunForecastRequest {
            parameters: params,
            historical_transactions: historical,
        };
        
        let state = tauri::State::from(Arc::clone(&service));
        let result = run_forecast(request, state).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_chart_data_command() {
        let service = Arc::new(RwLock::new(AccountingService::new()));
        let historical = vec![
            Transaction {
                id: uuid::Uuid::new_v4(),
                date: chrono::Utc::now(),
                description: "Test income".to_string(),
                journal_entries: vec![
                    crate::accounting::transaction::JournalEntry {
                        account_id: uuid::Uuid::new_v4(),
                        entry_type: crate::accounting::transaction::EntryType::Debit,
                        amount: crate::accounting::money::Money::new(1000.0, "USD"),
                        description: Some("Cash".to_string()),
                    },
                    crate::accounting::transaction::JournalEntry {
                        account_id: uuid::Uuid::new_v4(),
                        entry_type: crate::accounting::transaction::EntryType::Credit,
                        amount: crate::accounting::money::Money::new(1000.0, "USD"),
                        description: Some("Revenue".to_string()),
                    }
                ],
                reference: None,
                tags: vec!["test".to_string()],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];
        
        let params = ForecastParameters {
            start_date: chrono::Utc::now().date_naive(),
            end_date: chrono::Utc::now().date_naive() + chrono::Duration::days(30),
            interval: "monthly".to_string(),
            scenario_parameters: std::collections::HashMap::new(),
        };
        
        let request = RunForecastRequest {
            parameters: params,
            historical_transactions: historical,
        };
        
        let state = tauri::State::from(Arc::clone(&service));
        let result = get_forecast_chart_data(request, state).await;
        
        assert!(result.is_ok());
        let chart_data = result.unwrap();
        assert!(!chart_data.is_empty());
    }
}
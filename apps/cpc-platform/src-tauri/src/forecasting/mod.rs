pub mod commands;

use cpc_core::accounting::service::AccountingService;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared state for the forecasting system
#[derive(Debug)]
pub struct ForecastAppState {
    pub accounting_service: Arc<RwLock<AccountingService>>,
}

impl ForecastAppState {
    pub fn new() -> Self {
        let accounting_service = AccountingService::new();
        
        Self {
            accounting_service: Arc::new(RwLock::new(accounting_service)),
        }
    }
}

/// Initialize the forecasting system
pub fn init_forecasting(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create shared state
    let state = ForecastAppState::new();
    
    // Manage state
    app.manage(state.accounting_service.clone());
    
    Ok(())
}

/// Register all Tauri commands
pub fn register_commands<R: tauri::Runtime>(invoke_handler: tauri::Invoke<R>) -> tauri::Invoke<R> {
    invoke_handler
        .invoke_handler(tauri::generate_handler![
            commands::run_forecast,
            commands::get_forecast_chart_data,
            commands::get_forecast_scenarios,
            commands::get_forecast_summary,
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::accounting::{Transaction, TransactionType};
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_forecasting_system_initialization() {
        let state = ForecastAppState::new();
        let service = state.accounting_service.read().await;
        
        // Test that service is initialized
        assert!(service.get_dashboard().is_ok());
    }

    #[tokio::test]
    async fn test_forecast_with_historical_data() {
        let state = ForecastAppState::new();
        let service = state.accounting_service.write().await;
        
        // Create some test historical data
        let historical = vec![
            Transaction {
                id: uuid::Uuid::new_v4(),
                amount: Decimal::from(1000),
                transaction_type: TransactionType::Income,
                description: "Test income".to_string(),
                date: Utc::now() - chrono::Duration::days(30),
                category: Some("Revenue".to_string()),
                tags: vec!["test".to_string()],
            },
            Transaction {
                id: uuid::Uuid::new_v4(),
                amount: Decimal::from(500),
                transaction_type: TransactionType::Expense,
                description: "Test expense".to_string(),
                date: Utc::now() - chrono::Duration::days(15),
                category: Some("Operating".to_string()),
                tags: vec!["test".to_string()],
            },
        ];
        
        // Test forecast creation
        let forecast = service.create_forecast(historical);
        assert!(forecast.is_ok());
    }
}
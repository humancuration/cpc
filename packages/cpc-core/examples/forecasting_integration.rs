//! Integration example for financial forecasting
//! Demonstrates end-to-end usage of the forecasting system

use cpc_core::accounting::transaction::{Transaction, JournalEntry, EntryType};
use cpc_core::accounting::money::Money;
use cpc_core::business::{FinancialForecast, ForecastParameters};
use chrono::Duration;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create test historical transactions
    let historical_transactions = vec![
        Transaction {
            id: Uuid::new_v4(),
            date: chrono::Utc::now() - Duration::days(90),
            description: "Initial revenue".to_string(),
            journal_entries: vec![
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(5000.0),
                    entry_type: EntryType::Debit,
                    description: Some("Cash".to_string()),
                },
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(5000.0),
                    entry_type: EntryType::Credit,
                    description: Some("Revenue".to_string()),
                },
            ],
            reference: None,
            tags: vec!["revenue".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Transaction {
            id: Uuid::new_v4(),
            date: chrono::Utc::now() - Duration::days(60),
            description: "Monthly revenue".to_string(),
            journal_entries: vec![
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(6000.0),
                    entry_type: EntryType::Debit,
                    description: Some("Cash".to_string()),
                },
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(6000.0),
                    entry_type: EntryType::Credit,
                    description: Some("Revenue".to_string()),
                },
            ],
            reference: None,
            tags: vec!["revenue".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Transaction {
            id: Uuid::new_v4(),
            date: chrono::Utc::now() - Duration::days(30),
            description: "Operating expenses".to_string(),
            journal_entries: vec![
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(2000.0),
                    entry_type: EntryType::Debit,
                    description: Some("Office Expenses".to_string()),
                },
                JournalEntry {
                    account_id: Uuid::new_v4(),
                    amount: Money::from(2000.0),
                    entry_type: EntryType::Credit,
                    description: Some("Cash".to_string()),
                },
            ],
            reference: None,
            tags: vec!["expenses".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];

    // Create forecast parameters
    let parameters = ForecastParameters {
        start_date: chrono::Utc::now().date_naive(),
        end_date: chrono::Utc::now().date_naive() + Duration::days(90),
        interval: "monthly".to_string(),
        scenario_parameters: {
            let mut params = std::collections::HashMap::new();
            params.insert("algorithm".to_string(), "exponential_smoothing".to_string());
            params.insert("alpha".to_string(), 0.3);
            params.insert("growth_rate".to_string(), 1.05);
            params
        },
    };

    // Create and run forecast
    let mut forecast = FinancialForecast::new(parameters.clone());
    forecast.add_scenario("conservative".to_string(), parameters.clone());
    
    forecast.project_cash_flow("conservative", &historical_transactions)?;
    
    // Print results
    println!("=== Financial Forecast Results ===");
    println!("Forecast Period: {} to {}", parameters.start_date, parameters.end_date);
    println!("Algorithm: Exponential Smoothing");
    
    let scenario = forecast.scenarios.first().unwrap();
    println!("\nScenario: {}", scenario.name);
    
    for projection in &scenario.projections {
        println!(
            "Date: {}, Net Cash Flow: ${:.2}",
            projection.date,
            projection.net_cash_flow
        );
    }
    
    println!("\n=== Summary ===");
    let total_net = scenario.projections.iter()
        .map(|p| p.net_cash_flow)
        .sum::<f64>();
    println!("Total projected net cash flow: ${:.2}", total_net);
    
    Ok(())
}
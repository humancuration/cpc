//! Example usage of the accounting dashboard service
//!
//! Run with:
//! cargo run --example accounting_dashboard

use cpc_core::accounting::{AccountingService, PeriodType};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CPC Accounting Dashboard Example ===\n");
    
    // Create a mock organization ID
    let org_id = Uuid::new_v4();
    
    // Get dashboard data
    let dashboard = AccountingService::get_dashboard(org_id, PeriodType::Monthly).await?;
    
    println!("📊 Financial Summary");
    println!("===================");
    println!("Current Assets: ${}", dashboard.current_assets.to_float());
    println!("Liabilities:    ${}", dashboard.liabilities.to_float());
    println!("Equity:         ${}", dashboard.equity.to_float());
    println!("Working Capital: ${}", dashboard.working_capital().to_float());
    
    println!("\n📈 Performance Metrics (Last 30 Days)");
    println!("====================================");
    println!("Revenue:        ${}", dashboard.revenue_30d.to_float());
    println!("Expenses:       ${}", dashboard.expenses_30d.to_float());
    println!("Net Income:     ${}", dashboard.net_income().to_float());
    println!("Profit Margin:  {:.1}%", dashboard.profit_margin);
    
    println!("\n🔍 Key Financial Ratios");
    println!("======================");
    for (key, metric) in &dashboard.key_metrics {
        println!("{}: {}", metric.name(), metric.formatted());
    }
    
    Ok(())
}
//! Example of statistical analysis in the finance app
//!
//! This example demonstrates how to use the statistical analysis and visualization
//! components in the finance application.

#[cfg(feature = "statistics")]
use cpc_finance::application::statistical_forecasting::{
    StatisticalForecastingService,
    ExpenseForecast,
};
#[cfg(feature = "statistics")]
use cpc_finance::application::impact_analysis::{
    ImpactAnalysisService,
    SavingsImpactAnalysis,
};
#[cfg(all(feature = "statistics", feature = "visualization"))]
use cpc_finance::presentation::bi_integration::statistical_viz::FinancialStatisticalVisualization;

#[cfg(feature = "statistics")]
use cpc_finance::domain::{
    Expense,
    Budget,
    BudgetPeriod,
    SavingsGoal,
    FinancialCategory,
    primitives::Money,
    Currency,
};
#[cfg(feature = "statistics")]
use chrono::{Utc, Duration};
#[cfg(feature = "statistics")]
use rust_decimal::Decimal;
#[cfg(feature = "statistics")]
use uuid::Uuid;

/// Create sample expense data for demonstration
#[cfg(feature = "statistics")]
fn create_sample_expenses() -> Vec<Expense> {
    let now = Utc::now();
    vec![
        Expense::new(
            Uuid::new_v4(),
            "Groceries".to_string(),
            Money::new(Decimal::new(85, 0), Currency::USD),
            now - Duration::days(30),
            FinancialCategory::Groceries,
        ),
        Expense::new(
            Uuid::new_v4(),
            "Gas".to_string(),
            Money::new(Decimal::new(45, 0), Currency::USD),
            now - Duration::days(25),
            FinancialCategory::Transportation,
        ),
        Expense::new(
            Uuid::new_v4(),
            "Dining".to_string(),
            Money::new(Decimal::new(65, 0), Currency::USD),
            now - Duration::days(20),
            FinancialCategory::Dining,
        ),
        Expense::new(
            Uuid::new_v4(),
            "Groceries".to_string(),
            Money::new(Decimal::new(92, 0), Currency::USD),
            now - Duration::days(15),
            FinancialCategory::Groceries,
        ),
        Expense::new(
            Uuid::new_v4(),
            "Entertainment".to_string(),
            Money::new(Decimal::new(35, 0), Currency::USD),
            now - Duration::days(10),
            FinancialCategory::Entertainment,
        ),
        Expense::new(
            Uuid::new_v4(),
            "Groceries".to_string(),
            Money::new(Decimal::new(78, 0), Currency::USD),
            now - Duration::days(5),
            FinancialCategory::Groceries,
        ),
    ]
}

/// Create sample budget data for demonstration
#[cfg(feature = "statistics")]
fn create_sample_budget() -> Budget {
    let now = Utc::now();
    Budget::new(
        Uuid::new_v4(),
        "Monthly Budget".to_string(),
        Money::new(Decimal::new(2000, 0), Currency::USD),
        BudgetPeriod::Monthly,
        now,
        now + Duration::days(30),
    )
}

/// Create sample savings goals for demonstration
#[cfg(feature = "statistics")]
fn create_sample_savings_goals() -> Vec<SavingsGoal> {
    let now = Utc::now();
    vec![
        SavingsGoal::new(
            Uuid::new_v4(),
            "Vacation Fund".to_string(),
            Money::new(Decimal::new(3000, 0), Currency::USD),
            now + Duration::days(180),
        ),
        SavingsGoal::new(
            Uuid::new_v4(),
            "Emergency Fund".to_string(),
            Money::new(Decimal::new(10000, 0), Currency::USD),
            now + Duration::days(365),
        ),
    ]
}

/// Demonstrate expense forecasting
#[cfg(feature = "statistics")]
fn demonstrate_expense_forecasting() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Expense Forecasting Demo ===");
    
    let expenses = create_sample_expenses();
    println!("Created {} sample expenses", expenses.len());
    
    let forecast = StatisticalForecastingService::forecast_expenses(&expenses, 12, 0.95)?;
    println!("Generated forecast for 12 periods with 95% confidence");
    
    println!("Forecast explanation: {}", forecast.explanation());
    
    #[cfg(feature = "visualization")]
    {
        println!("Generating visualization...");
        let visualization = FinancialStatisticalVisualization::generate_forecast_visualization(
            &forecast,
            &expenses,
        )?;
        println!("Generated visualization with dimensions: {}x{}", 
                 visualization.width(), visualization.height());
    }
    
    Ok(())
}

/// Demonstrate impact analysis
#[cfg(feature = "statistics")]
fn demonstrate_impact_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Impact Analysis Demo ===");
    
    let budgets = vec![create_sample_budget()];
    let expenses = create_sample_expenses();
    let savings_goals = create_sample_savings_goals();
    
    println!("Analyzing budgeting impact...");
    let budgeting_impact = ImpactAnalysisService::analyze_budgeting_impact(&budgets, &expenses)?;
    println!("Budgeting impact analysis complete");
    println!("Impact explanation: {}", budgeting_impact.cooperative_explanation());
    
    println!("Analyzing savings impact...");
    let savings_impact = ImpactAnalysisService::analyze_savings_impact(
        &savings_goals, 
        &expenses, 
        0.15, // Prior mean of 15% savings rate
        0.05, // Prior std deviation of 5%
    )?;
    println!("Savings impact analysis complete");
    println!("Impact explanation: {}", savings_impact.cooperative_explanation());
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "statistics")]
    {
        demonstrate_expense_forecasting()?;
        demonstrate_impact_analysis()?;
        println!("\nStatistical analysis demo completed successfully!");
    }
    
    #[cfg(not(feature = "statistics"))]
    {
        println!("Statistical analysis requires the 'statistics' feature to be enabled");
    }
    
    Ok(())
}
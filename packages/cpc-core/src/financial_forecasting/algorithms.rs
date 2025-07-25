// packages/cpc-core/src/financial_forecasting/algorithms.rs
use super::models::{ForecastScenario, ForecastResult, ProjectedMonth, RevenueItem, ExpenseItem};
use chrono::{Datelike, Duration, NaiveDate};

/// Projects future profit and loss based on a given scenario.
///
/// This function is the core of the forecasting engine. It iterates through
/// each month in the forecast horizon, calculating projected revenue and expenses
/// based on initial values, growth rates, and any scenario-specific assumptions.
pub fn project_profit_and_loss(scenario: &ForecastScenario) -> ForecastResult {
    let mut monthly_projections = Vec::new();
    
    // Extract the start date from the initial statement
    let start_date = scenario.initial_statement.start_date;
    let start_year = start_date.year();
    let start_month = start_date.month();
    
    // Create a map of assumptions for quick lookup
    let assumptions_map: std::collections::HashMap<String, &super::models::ForecastAssumption> = scenario
        .assumptions
        .iter()
        .map(|assumption| (assumption.item_id.clone(), assumption))
        .collect();
    
    // Track current amounts for each revenue and expense item
    let mut current_revenue_items: Vec<(RevenueItem, f64)> = scenario
        .initial_statement
        .revenue_items
        .iter()
        .map(|item| (item.clone(), item.amount))
        .collect();
    
    let mut current_expense_items: Vec<(ExpenseItem, f64)> = scenario
        .initial_statement
        .expense_items
        .iter()
        .map(|item| (item.clone(), item.amount))
        .collect();
    
    // Process each month in the forecast horizon
    for month_index in 0..scenario.forecast_horizon_months {
        let current_year = start_year + ((start_month as u32 + month_index - 1) / 12) as i32;
        let current_month = ((start_month as u32 + month_index - 1) % 12) + 1;
        
        // Calculate total revenue for this month
        let mut total_revenue = 0.0;
        for (item, current_amount) in &mut current_revenue_items {
            let mut amount = *current_amount;
            
            // Check if there's an override for this item
            if let Some(assumption) = assumptions_map.get(&item.id) {
                if let Some(new_amount) = assumption.new_amount {
                    amount = new_amount;
                    *current_amount = new_amount;
                }
                if let Some(new_growth_rate) = assumption.new_growth_rate {
                    item.growth_rate_monthly = new_growth_rate;
                }
            }
            
            total_revenue += amount;
            
            // Apply growth for next month (compound growth)
            *current_amount *= 1.0 + item.growth_rate_monthly;
        }
        
        // Calculate total expenses for this month
        let mut total_expenses = 0.0;
        for (item, current_amount) in &mut current_expense_items {
            let mut amount = *current_amount;
            
            // Check if there's an override for this item
            if let Some(assumption) = assumptions_map.get(&item.id) {
                if let Some(new_amount) = assumption.new_amount {
                    amount = new_amount;
                    *current_amount = new_amount;
                }
            }
            
            total_expenses += amount;
            
            // For variable expenses, apply a simple inflation factor (using average revenue growth)
            // Fixed expenses remain constant unless overridden
            if !item.is_fixed {
                let avg_revenue_growth: f64 = if scenario.initial_statement.revenue_items.is_empty() {
                    0.0
                } else {
                    scenario
                        .initial_statement
                        .revenue_items
                        .iter()
                        .map(|item| item.growth_rate_monthly)
                        .sum::<f64>()
                        / scenario.initial_statement.revenue_items.len() as f64
                };
                *current_amount *= 1.0 + avg_revenue_growth;
            }
        }
        
        // Calculate profit/loss
        let profit_loss = total_revenue - total_expenses;
        
        // Create the projected month
        let projected_month = ProjectedMonth {
            month: current_month as u32,
            year: current_year,
            total_revenue,
            total_expenses,
            profit_loss,
        };
        
        monthly_projections.push(projected_month);
    }
    
    ForecastResult {
        scenario_id: scenario.id.clone(),
        monthly_projections,
    }
}
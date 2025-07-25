#[cfg(test)]
mod tests {
    use super::algorithms;
    use super::models::*;
    use chrono::Utc;

    #[test]
    fn test_basic_forecast_projection() {
        let scenario = ForecastScenario {
            id: "test-1".to_string(),
            name: "Basic Test".to_string(),
            description: "Simple test scenario".to_string(),
            initial_statement: FinancialStatement {
                id: "stmt-1".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                revenue_items: vec![
                    RevenueItem {
                        id: "rev-1".to_string(),
                        name: "Product Sales".to_string(),
                        amount: 10000.0,
                        growth_rate_monthly: 0.05, // 5% monthly growth
                    },
                ],
                expense_items: vec![
                    ExpenseItem {
                        id: "exp-1".to_string(),
                        name: "Rent".to_string(),
                        amount: 2000.0,
                        is_fixed: true,
                    },
                    ExpenseItem {
                        id: "exp-2".to_string(),
                        name: "COGS".to_string(),
                        amount: 3000.0,
                        is_fixed: false,
                    },
                ],
            },
            forecast_horizon_months: 3,
            assumptions: vec![],
        };

        let result = super::algorithms::project_profit_and_loss(&scenario);
        
        assert_eq!(result.scenario_id, "test-1");
        assert_eq!(result.monthly_projections.len(), 3);
        
        // First month
        let first_month = &result.monthly_projections[0];
        assert_eq!(first_month.total_revenue, 10000.0);
        assert_eq!(first_month.total_expenses, 5000.0); // 2000 + 3000
        assert_eq!(first_month.profit_loss, 5000.0);
        
        // Second month - revenue should grow by 5%
        let second_month = &result.monthly_projections[1];
        assert!(second_month.total_revenue > 10000.0);
        assert_eq!(second_month.total_expenses, 5150.0); // 2000 + (3000 * 1.05)
    }

    #[test]
    fn test_forecast_with_assumptions() {
        let scenario = ForecastScenario {
            id: "test-assumptions".to_string(),
            name: "Assumptions Test".to_string(),
            description: "Test with assumptions".to_string(),
            initial_statement: FinancialStatement {
                id: "stmt-2".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                revenue_items: vec![
                    RevenueItem {
                        id: "rev-1".to_string(),
                        name: "Product Sales".to_string(),
                        amount: 10000.0,
                        growth_rate_monthly: 0.05,
                    },
                ],
                expense_items: vec![
                    ExpenseItem {
                        id: "exp-1".to_string(),
                        name: "Rent".to_string(),
                        amount: 2000.0,
                        is_fixed: true,
                    },
                ],
            },
            forecast_horizon_months: 2,
            assumptions: vec![
                ForecastAssumption {
                    item_id: "rev-1".to_string(),
                    new_growth_rate: Some(0.1), // Override to 10% growth
                    new_amount: None,
                },
                ForecastAssumption {
                    item_id: "exp-1".to_string(),
                    new_growth_rate: None,
                    new_amount: Some(2500.0), // Override rent to 2500
                },
            ],
        };

        let result = super::algorithms::project_profit_and_loss(&scenario);
        
        assert_eq!(result.monthly_projections.len(), 2);
        
        // First month with overrides
        let first_month = &result.monthly_projections[0];
        assert_eq!(first_month.total_revenue, 10000.0);
        assert_eq!(first_month.total_expenses, 2500.0); // Overridden rent
        assert_eq!(first_month.profit_loss, 7500.0);
        
        // Second month - revenue should grow by 10% (not 5%)
        let second_month = &result.monthly_projections[1];
        assert_eq!(second_month.total_revenue, 11000.0); // 10000 * 1.10
        assert_eq!(second_month.total_expenses, 2500.0); // Fixed rent
    }
}
//! Integration tests for accounting module

#[cfg(test)]
mod dashboard_tests {
    use crate::accounting::dashboard::{AccountingDashboard, Metric};
    use crate::accounting::Money;
    use uuid::Uuid;

    #[test]
    fn test_dashboard_integration() {
        let mut dashboard = AccountingDashboard::new("USD");
        
        // Set up realistic data
        dashboard.current_assets = Money::new(200000.0, "USD");
        dashboard.liabilities = Money::new(80000.0, "USD");
        dashboard.equity = Money::new(120000.0, "USD");
        dashboard.revenue_30d = Money::new(75000.0, "USD");
        dashboard.expenses_30d = Money::new(45000.0, "USD");
        
        dashboard.update_key_metrics();
        
        // Verify calculations
        let working_capital = dashboard.working_capital();
        assert_eq!(working_capital.amount, 120000); // 200k - 80k
        
        let net_income = dashboard.net_income();
        assert_eq!(net_income.amount, 30000); // 75k - 45k
        
        // Verify key metrics
        assert!(dashboard.key_metrics.contains_key("current_ratio"));
        assert!(dashboard.key_metrics.contains_key("quick_ratio"));
        assert!(dashboard.key_metrics.contains_key("debt_to_equity"));
        
        if let Some(Metric::CurrentRatio(ratio)) = dashboard.key_metrics.get("current_ratio") {
            assert!((ratio - 2.5).abs() < 0.01); // 200k/80k = 2.5
        }
    }

    #[test]
    fn test_dashboard_serialization() {
        let mut dashboard = AccountingDashboard::new("USD");
        dashboard.current_assets = Money::new(100000.0, "USD");
        dashboard.liabilities = Money::new(50000.0, "USD");
        dashboard.equity = Money::new(50000.0, "USD");
        
        dashboard.update_key_metrics();
        
        // Test JSON serialization
        let json = serde_json::to_string(&dashboard).unwrap();
        assert!(json.contains("\"current_assets\":{\"amount\":10000000,\"currency\":\"USD\"}"));
        
        // Test JSON deserialization
        let deserialized: AccountingDashboard = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.current_assets.amount, 10000000);
    }
}
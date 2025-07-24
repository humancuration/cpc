# Financial Forecasting Permission Testing Plan

## 1. Unit Tests for Permission System

### Test Cases for `permissions.rs`
```rust
// Test file: packages/cpc-core/src/permissions.rs

#[cfg(test)]
mod tests {
    use super::*;

    // Test valid permission check
    #[test]
    fn test_valid_permission_check() {
        let user_perms = vec![Permission::RunSensitivityAnalysis];
        assert!(Permission::RunSensitivityAnalysis.check(&user_perms));
    }

    // Test missing permission
    #[test]
    fn test_missing_permission() {
        let user_perms = vec![Permission::ViewFinancialDashboard];
        assert!(!Permission::RunSensitivityAnalysis.check(&user_perms));
    }

    // Test admin vs non-admin
    #[test]
    fn test_admin_permissions() {
        let admin_perms = vec![
            Permission::ViewFinancialDashboard,
            Permission::EditFinancialScenarios,
            Permission::RunSensitivityAnalysis,
            Permission::ManageBusinessAccounts
        ];
        let user_perms = vec![Permission::ViewFinancialDashboard];
        
        // Admin should have all permissions
        assert!(Permission::RunSensitivityAnalysis.check(&admin_perms));
        
        // Regular user should not have admin permissions
        assert!(!Permission::RunSensitivityAnalysis.check(&user_perms));
    }

    // Test invalid permission scenario
    #[test]
    fn test_invalid_permission_scenario() {
        let user_perms = vec![];
        assert!(!Permission::EditFinancialScenarios.check(&user_perms));
    }
}
```

## 2. Integration Tests for Financial Forecasting API

### Test Cases for `financial_forecasting.rs`
```rust
// Test file: apps/backend/src/api/business_tools/financial_forecasting.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use axum::http::StatusCode;

    // Test run_sensitivity_analysis with valid permissions
    #[tokio::test]
    async fn test_sensitivity_analysis_valid_permission() {
        let app = test_app().await;
        let user = create_test_user_with_perms(vec!["RunSensitivityAnalysis"]).await;
        
        let response = app
            .post("/api/financial-forecasting/sensitivity/scenario1")
            .json(&SensitivityParameters::default())
            .with_user(user)
            .send()
            .await;
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Test run_sensitivity_analysis with invalid permissions
    #[tokio::test]
    async fn test_sensitivity_analysis_invalid_permission() {
        let app = test_app().await;
        let user = create_test_user_with_perms(vec!["ViewFinancialDashboard"]).await;
        
        let response = app
            .post("/api/financial-forecasting/sensitivity/scenario1")
            .json(&SensitivityParameters::default())
            .with_user(user)
            .send()
            .await;
        
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(response.json::<ApiError>().await.unwrap().message, "Insufficient permissions for sensitivity analysis");
    }

    // Test update_projections with valid permissions
    #[tokio::test]
    async fn test_update_projections_valid_permission() {
        let app = test_app().await;
        let user = create_test_user_with_perms(vec!["EditFinancialScenarios"]).await;
        
        let response = app
            .put("/api/financial-forecasting/projections")
            .json(&FinancialForecastingParams::default())
            .with_user(user)
            .send()
            .await;
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Test permission failure logging
    #[tokio::test]
    async fn test_permission_failure_logging() {
        let app = test_app().await;
        let user = create_test_user_with_perms(vec![]).await;
        
        let response = app
            .put("/api/financial-forecasting/projections")
            .json(&FinancialForecastingParams::default())
            .with_user(user)
            .send()
            .await;
        
        // Verify permission failure was logged
        let logs = app.logs().await;
        assert!(logs.contains("Permission denied for EditFinancialScenarios"));
    }
}
```

## 3. Manual Testing Scenarios

### Step-by-Step Instructions for QA Testers

1. **Test Setup**:
   - Deploy latest staging build
   - Create test users with different roles:
     - Admin: All permissions
     - Financial Analyst: `ViewFinancialDashboard` + `RunSensitivityAnalysis`
     - Editor: `EditFinancialScenarios`
     - Viewer: `ViewFinancialDashboard`

2. **Scenario 1: Run Sensitivity Analysis**
   - Steps:
     1. Log in as Financial Analyst
     2. Navigate to Financial Forecasting > Sensitivity Analysis
     3. Select scenario and parameters
     4. Run analysis
   - Expected: Success message with results
   - Permission: `RunSensitivityAnalysis`

3. **Scenario 2: Attempt Sensitivity Analysis without Permission**
   - Steps:
     1. Log in as Viewer
     2. Attempt to access Sensitivity Analysis page
   - Expected: "Access Denied" message
   - Permission: `RunSensitivityAnalysis`

4. **Scenario 3: Update Projections**
   - Steps:
     1. Log in as Editor
     2. Navigate to Financial Forecasting > Projections
     3. Update parameters and save
   - Expected: Success message with updated projections
   - Permission: `EditFinancialScenarios`

5. **Scenario 4: Attempt Projection Update without Permission**
   - Steps:
     1. Log in as Financial Analyst
     2. Attempt to access Projections editor
   - Expected: "Access Denied" message
   - Permission: `EditFinancialScenarios`

## 4. Monitoring Recommendations

### Permission Failure Logging
- Log format: `[ERROR] Permission denied: {permission_name} for user {user_id} at {timestamp}`
- Include contextual information:
  - Endpoint accessed
  - User role
  - Request parameters

### Metrics to Track
```prometheus
# Permission-related metrics
cpc_permission_denied_total{permission="EditFinancialScenarios"} 5
cpc_permission_denied_total{permission="RunSensitivityAnalysis"} 3
```

### Alerting Rules
- Alert when permission denial rate exceeds 5% of total requests
- Alert on repeated permission failures for the same user-endpoint combination

### Dashboard Recommendations
- Grafana dashboard showing:
  - Permission denial rate by endpoint
  - Top users with permission failures
  - Permission usage heatmap by time of day
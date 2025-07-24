//! Permission definitions for CPC ecosystem
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    ViewFinancialDashboard,
    EditFinancialScenarios,
    RunSensitivityAnalysis,
    ManageBusinessAccounts,
    ManageFinancialForecasting,
    ManageUserTesting,
    // Add other permissions as needed
}

impl Permission {
    /// Checks if a user has a specific permission
    pub fn check(&self, user_permissions: &[Permission]) -> bool {
        user_permissions.contains(self)
    }
}

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
//! # CPC RBAC (Role-Based Access Control)
//!
//! A flexible RBAC system for the CPC ecosystem that provides role and permission management
//! across all applications.

use std::collections::HashMap;

/// RBAC Engine for managing roles and permissions
pub struct RbacEngine {
    /// Map of roles to their permissions
    roles: HashMap<String, Vec<String>>,
}

impl RbacEngine {
    /// Create a new RBAC engine
    pub fn new() -> Self {
        Self {
            roles: HashMap::new(),
        }
    }

    /// Add a role with its permissions
    pub fn add_role(&mut self, role: String, permissions: Vec<String>) {
        self.roles.insert(role, permissions);
    }

    /// Check if a role has a specific permission
    pub fn check_permission(&self, role: &str, permission: &str) -> bool {
        if let Some(permissions) = self.roles.get(role) {
            permissions.contains(&permission.to_string())
        } else {
            false
        }
    }

    /// Get all permissions for a role
    pub fn get_permissions(&self, role: &str) -> Option<&Vec<String>> {
        self.roles.get(role)
    }

    /// Get all roles
    pub fn get_roles(&self) -> Vec<&String> {
        self.roles.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbac_engine() {
        let mut rbac = RbacEngine::new();
        
        // Add roles
        rbac.add_role("user".to_string(), vec!["read".to_string(), "write".to_string()]);
        rbac.add_role("admin".to_string(), vec!["read".to_string(), "write".to_string(), "delete".to_string()]);
        
        // Check permissions
        assert!(rbac.check_permission("user", "read"));
        assert!(rbac.check_permission("user", "write"));
        assert!(!rbac.check_permission("user", "delete"));
        
        assert!(rbac.check_permission("admin", "read"));
        assert!(rbac.check_permission("admin", "write"));
        assert!(rbac.check_permission("admin", "delete"));
        
        // Check non-existent role
        assert!(!rbac.check_permission("guest", "read"));
    }
}
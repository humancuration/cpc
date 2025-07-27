//! Tests for module dependency resolution
//!
//! These tests verify that the module registry correctly resolves dependencies
//! using topological sorting and detects circular dependencies.

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use sqlx::PgPool;
    use axum::Router;
    use async_graphql::{EmptySubscription, Object, SchemaBuilder};
    use anyhow::Result;
    use semver::VersionReq;

    // Mock module for testing
    struct MockModule {
        name: String,
        version: String,
        enabled: bool,
    }

    #[async_trait::async_trait]
    impl crate::module_registry::Module for MockModule {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn is_enabled(&self) -> bool {
            self.enabled
        }

        fn router(&self) -> Option<Router> {
            None
        }

        fn register_schema(&self, _builder: &mut SchemaBuilder<Object, Object, EmptySubscription>) {
            // Mock implementation
        }

        async fn enable(&mut self, _pool: &PgPool) -> Result<()> {
            self.enabled = true;
            Ok(())
        }

        async fn disable(&mut self, _pool: &PgPool) -> Result<()> {
            self.enabled = false;
            Ok(())
        }
    }

    #[test]
    fn test_linear_dependency_resolution() {
        let mut registry = ModuleRegistry::new(PgPool::connect_lazy("postgresql://localhost/test").unwrap());
        
        // Create modules with linear dependencies: A -> B -> C
        let module_c = Arc::new(RwLock::new(MockModule {
            name: "module-c".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_b = Arc::new(RwLock::new(MockModule {
            name: "module-b".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_a = Arc::new(RwLock::new(MockModule {
            name: "module-a".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        // Register modules with dependencies
        registry.register_module_with_dependencies(
            module_c.clone(),
            vec![]  // C has no dependencies
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_b.clone(),
            vec![DependencyRequirement::Required {
                name: "module-c".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_a.clone(),
            vec![DependencyRequirement::Required {
                name: "module-b".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        // Test dependency resolution for module A
        let resolution = registry.resolve_dependencies("module-a").unwrap();
        
        // Should be resolved in order: C, B, A (dependencies first)
        assert_eq!(resolution, vec!["module-c", "module-b", "module-a"]);
    }

    #[test]
    fn test_branching_dependency_resolution() {
        let mut registry = ModuleRegistry::new(PgPool::connect_lazy("postgresql://localhost/test").unwrap());
        
        // Create modules with branching dependencies:
        //    A
        //   / \
        //  B   C
        //   \ /
        //    D
        let module_d = Arc::new(RwLock::new(MockModule {
            name: "module-d".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_c = Arc::new(RwLock::new(MockModule {
            name: "module-c".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_b = Arc::new(RwLock::new(MockModule {
            name: "module-b".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_a = Arc::new(RwLock::new(MockModule {
            name: "module-a".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        // Register modules with dependencies
        registry.register_module_with_dependencies(
            module_d.clone(),
            vec![]  // D has no dependencies
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_c.clone(),
            vec![DependencyRequirement::Required {
                name: "module-d".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_b.clone(),
            vec![DependencyRequirement::Required {
                name: "module-d".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_a.clone(),
            vec![
                DependencyRequirement::Required {
                    name: "module-b".to_string(),
                    constraint: VersionReq::parse(">=1.0.0").unwrap(),
                },
                DependencyRequirement::Required {
                    name: "module-c".to_string(),
                    constraint: VersionReq::parse(">=1.0.0").unwrap(),
                }
            ]
        ).unwrap();
        
        // Test dependency resolution for module A
        let resolution = registry.resolve_dependencies("module-a").unwrap();
        
        // Should be resolved in order: D, B/C (order may vary), A
        // D must come first, A must come last
        assert_eq!(resolution[0], "module-d");
        assert_eq!(resolution[3], "module-a");
        assert!(resolution.contains(&"module-b"));
        assert!(resolution.contains(&"module-c"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut registry = ModuleRegistry::new(PgPool::connect_lazy("postgresql://localhost/test").unwrap());
        
        // Create modules with circular dependency: A -> B -> A
        let module_a = Arc::new(RwLock::new(MockModule {
            name: "module-a".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        let module_b = Arc::new(RwLock::new(MockModule {
            name: "module-b".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        // Register modules with circular dependencies
        registry.register_module_with_dependencies(
            module_a.clone(),
            vec![DependencyRequirement::Required {
                name: "module-b".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        registry.register_module_with_dependencies(
            module_b.clone(),
            vec![DependencyRequirement::Required {
                name: "module-a".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        // Test circular dependency detection
        let result = registry.resolve_dependencies("module-a");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular dependency detected"));
    }

    #[test]
    fn test_optional_dependency_handling() {
        let mut registry = ModuleRegistry::new(PgPool::connect_lazy("postgresql://localhost/test").unwrap());
        
        // Create modules with optional dependency
        let module_a = Arc::new(RwLock::new(MockModule {
            name: "module-a".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
        }));
        
        // Register module with optional dependency on non-existent module
        registry.register_module_with_dependencies(
            module_a.clone(),
            vec![DependencyRequirement::Optional {
                name: "non-existent-module".to_string(),
                constraint: VersionReq::parse(">=1.0.0").unwrap(),
            }]
        ).unwrap();
        
        // Should resolve successfully even though optional dependency doesn't exist
        let resolution = registry.resolve_dependencies("module-a").unwrap();
        assert_eq!(resolution, vec!["module-a"]);
    }
}
//! Dynamic GraphQL schema builder for modular architecture
//!
//! This module provides functionality to build GraphQL schemas dynamically
//! based on enabled modules in the module registry.

use async_graphql::{EmptySubscription, Object, Schema, SchemaBuilder as GraphQLSchemaBuilder};
use crate::module_registry::ModuleRegistry;

/// Dynamic schema builder that constructs schemas based on enabled modules
pub struct SchemaBuilder;

impl SchemaBuilder {
    /// Build a GraphQL schema based on the enabled modules in the registry
    pub fn build(registry: &ModuleRegistry) -> Schema<Object, Object, EmptySubscription> {
        let mut builder = GraphQLSchemaBuilder::new();
        
        // Process modules in dependency order
        if let Ok(modules) = registry.modules_in_dependency_order() {
            for registered_module in modules {
                let module_guard = registered_module.module.blocking_read();
                module_guard.register_schema(&mut builder);
            }
        }
        
        builder.finish()
    }
}
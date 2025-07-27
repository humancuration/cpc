//! Module registry system for dynamic module management
//!
//! This module provides the infrastructure for registering, enabling, and disabling
//! application modules at runtime without requiring a restart.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use axum::Router;
use async_graphql::{EmptySubscription, Object, Schema, SchemaBuilder as GraphQLSchemaBuilder};
use anyhow::Result;
use semver::{Version, VersionReq};

/// Trait that all modules must implement to be registered in the system
#[async_trait::async_trait]
pub trait Module: Send + Sync {
    /// Get the module name
    fn name(&self) -> &str;
    
    /// Get the module version
    fn version(&self) -> &str;
    
    /// Check if the module is currently enabled
    fn is_enabled(&self) -> bool;
    
    /// Get the module's router for HTTP endpoints
    fn router(&self) -> Option<Router>;
    
    /// Register module types with the schema builder
    fn register_schema(&self, builder: &mut GraphQLSchemaBuilder<Object, Object, EmptySubscription>);
    
    /// Enable the module
    async fn enable(&mut self, pool: &PgPool) -> Result<()>;
    
    /// Disable the module
    async fn disable(&mut self, pool: &PgPool) -> Result<()>;
}

/// Module dependency requirement
#[derive(Debug, Clone)]
pub enum DependencyRequirement {
    /// Required dependency with version constraint
    Required { name: String, constraint: VersionReq },
    /// Optional dependency with version constraint
    Optional { name: String, constraint: VersionReq },
}

/// Registered module with metadata
pub struct RegisteredModule {
    /// The module instance
    pub module: Arc<RwLock<dyn Module>>,
    /// Module dependencies
    pub dependencies: Vec<DependencyRequirement>,
    /// Module version
    pub version: Version,
}

/// Central registry for all modules
pub struct ModuleRegistry {
    /// All registered modules
    modules: HashMap<String, RegisteredModule>,
    
    /// Names of currently enabled modules
    enabled_modules: Vec<String>,
    
    /// Database connection pool
    pool: PgPool,
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new(pool: PgPool) -> Self {
        Self {
            modules: HashMap::new(),
            enabled_modules: Vec::new(),
            pool,
        }
    }
    
    /// Register a new module
    pub fn register_module_with_dependencies(
        &mut self,
        module: Arc<RwLock<dyn Module>>,
        dependencies: Vec<DependencyRequirement>
    ) -> Result<()> {
        let (name, version_str) = {
            let module_guard = module.blocking_read();
            (module_guard.name().to_string(), module_guard.version().to_string())
        };
        
        let version = Version::parse(&version_str)
            .map_err(|e| anyhow::anyhow!("Invalid version for module {}: {}", name, e))?;
        
        self.modules.insert(name, RegisteredModule {
            module,
            dependencies,
            version,
        });
        
        Ok(())
    }
    
    /// Enable a module by name, including its dependencies
    pub async fn enable_module(&mut self, name: &str) -> Result<()> {
        // Check if module exists
        if !self.modules.contains_key(name) {
            return Err(anyhow::anyhow!("Module {} not found", name));
        }
        
        // Check if already enabled
        if self.enabled_modules.contains(&name.to_string()) {
            return Ok(());
        }
        
        // Get dependency order using topological sorting
        let enable_order = self.resolve_dependencies(name)?;
        
        // Enable modules in dependency order
        for module_name in enable_order {
            if !self.enabled_modules.contains(&module_name) {
                if let Some(registered_module) = self.modules.get(&module_name) {
                    registered_module.module.write().await.enable(&self.pool).await?;
                    self.enabled_modules.push(module_name.clone());
                    
                    // Persist this change to database
                    self.persist_module_state(&module_name, true).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Disable a module by name
    pub async fn disable_module(&mut self, name: &str) -> Result<()> {
        // Check if module exists
        if !self.modules.contains_key(name) {
            return Err(anyhow::anyhow!("Module {} not found", name));
        }
        
        // Check if already disabled
        if !self.enabled_modules.contains(&name.to_string()) {
            return Ok(());
        }
        
        // Disable the module
        if let Some(registered_module) = self.modules.get(name) {
            registered_module.module.write().await.disable(&self.pool).await?;
            
            // Remove from enabled modules
            if let Some(pos) = self.enabled_modules.iter().position(|m| m == name) {
                self.enabled_modules.remove(pos);
            }
            
            // Persist this change to database
            self.persist_module_state(name, false).await?;
        }
        
        Ok(())
    }
    
    /// Get a list of all available modules
    pub fn available_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
    
    /// Get a list of all enabled modules
    pub fn enabled_modules(&self) -> &Vec<String> {
        &self.enabled_modules
    }
    
    /// Get modules in dependency order for schema building
    pub fn modules_in_dependency_order(&self) -> Result<Vec<&RegisteredModule>> {
        self.resolve_all_dependencies()
    }
    
    /// Resolve dependencies for a module using Kahn's algorithm for topological sorting
    fn resolve_dependencies(&self, target_module: &str) -> Result<Vec<String>> {
        // Build dependency graph for the target module and its dependencies
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize with target module
        in_degree.insert(target_module.to_string(), 0);
        
        // Collect all dependencies recursively
        let mut to_process = VecDeque::new();
        to_process.push_back(target_module.to_string());
        
        while let Some(current_module) = to_process.pop_front() {
            if let Some(registered_module) = self.modules.get(&current_module) {
                for dep in &registered_module.dependencies {
                    let dep_name = match dep {
                        DependencyRequirement::Required { name, .. } => name,
                        DependencyRequirement::Optional { name, .. } => name,
                    };
                    
                    // Skip if we don't have this module (for optional dependencies)
                    if !self.modules.contains_key(dep_name) {
                        continue;
                    }
                    
                    // Add to adjacency list
                    adjacency_list.entry(current_module.clone()).or_insert_with(Vec::new).push(dep_name.clone());
                    
                    // Update in-degree
                    *in_degree.entry(dep_name.clone()).or_insert(0) += 1;
                    
                    // Add to processing queue if not already there
                    if !in_degree.contains_key(dep_name) {
                        in_degree.insert(dep_name.clone(), 0);
                        to_process.push_back(dep_name.clone());
                    }
                }
            }
        }
        
        // Apply Kahn's algorithm for topological sorting
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut result: Vec<String> = Vec::new();
        
        // Find all nodes with in-degree 0
        for (module, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(module.clone());
            }
        }
        
        // Process nodes
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            // For each dependent of this node
            if let Some(dependents) = adjacency_list.get(&node) {
                for dependent in dependents {
                    // Reduce in-degree
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        // If in-degree becomes 0, add to queue
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
        
        // Check for circular dependencies
        if result.len() != in_degree.len() {
            return Err(anyhow::anyhow!("Circular dependency detected"));
        }
        
        // Reverse the result to get dependency order (dependencies first)
        result.reverse();
        Ok(result)
    }
    
    /// Resolve all dependencies for enabled modules
    fn resolve_all_dependencies(&self) -> Result<Vec<&RegisteredModule>> {
        // Build dependency graph for all enabled modules
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize with enabled modules
        for module_name in &self.enabled_modules {
            in_degree.insert(module_name.clone(), 0);
        }
        
        // Build adjacency list and in-degree map
        for module_name in &self.enabled_modules {
            if let Some(registered_module) = self.modules.get(module_name) {
                for dep in &registered_module.dependencies {
                    let dep_name = match dep {
                        DependencyRequirement::Required { name, .. } => name,
                        DependencyRequirement::Optional { name, .. } => name,
                    };
                    
                    // Skip if dependency is not enabled
                    if !self.enabled_modules.contains(dep_name) {
                        continue;
                    }
                    
                    // Add to adjacency list
                    adjacency_list.entry(module_name.clone()).or_insert_with(Vec::new).push(dep_name.clone());
                    
                    // Update in-degree
                    *in_degree.entry(dep_name.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Apply Kahn's algorithm for topological sorting
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut result: Vec<String> = Vec::new();
        
        // Find all nodes with in-degree 0
        for module_name in &self.enabled_modules {
            let degree = *in_degree.get(module_name).unwrap_or(&0);
            if degree == 0 {
                queue.push_back(module_name.clone());
            }
        }
        
        // Process nodes
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            // For each dependent of this node
            if let Some(dependents) = adjacency_list.get(&node) {
                for dependent in dependents {
                    // Reduce in-degree
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        // If in-degree becomes 0, add to queue
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
        
        // Check for circular dependencies
        if result.len() != self.enabled_modules.len() {
            return Err(anyhow::anyhow!("Circular dependency detected in enabled modules"));
        }
        
        // Reverse the result to get dependency order (dependencies first)
        result.reverse();
        
        // Return modules in dependency order
        let mut ordered_modules = Vec::new();
        for module_name in result {
            if let Some(module) = self.modules.get(&module_name) {
                ordered_modules.push(module);
            }
        }
        
        Ok(ordered_modules)
    }
    
    /// Rebuild the GraphQL schema based on enabled modules
    pub fn rebuild_schema(&self) -> Schema<Object, Object, EmptySubscription> {
        let mut builder = GraphQLSchemaBuilder::new();
        
        // Process modules in dependency order
        if let Ok(modules) = self.modules_in_dependency_order() {
            for registered_module in modules {
                let module_guard = registered_module.module.blocking_read();
                module_guard.register_schema(&mut builder);
            }
        }
        
        builder.finish()
    }
    
    /// Rebuild the router based on enabled modules
    pub fn rebuild_router(&self) -> Router {
        let mut router = Router::new();
        
        for module_name in &self.enabled_modules {
            if let Some(registered_module) = self.modules.get(module_name) {
                let module_guard = registered_module.module.blocking_read();
                if let Some(module_router) = module_guard.router() {
                    router = router.merge(module_router);
                }
            }
        }
        
        router
    }
    
    /// Load enabled modules from database
    pub async fn load_enabled_modules(&mut self) -> Result<()> {
        // Create module registry table if it doesn't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS module_registry (
                module_name TEXT PRIMARY KEY,
                is_enabled BOOLEAN NOT NULL DEFAULT false,
                enabled_at TIMESTAMP,
                dependencies JSONB NOT NULL DEFAULT '{}',
                version TEXT NOT NULL DEFAULT '0.0.0'
            )"
        )
        .execute(&self.pool)
        .await?;
        
        // Load enabled modules from database
        let rows = sqlx::query!(
            "SELECT module_name FROM module_registry WHERE is_enabled = true"
        )
        .fetch_all(&self.pool)
        .await?;
        
        self.enabled_modules = rows.into_iter()
            .map(|row| row.module_name)
            .collect();
        
        Ok(())
    }
    
    /// Persist module state to database
    async fn persist_module_state(&self, module_name: &str, is_enabled: bool) -> Result<()> {
        sqlx::query(
            "INSERT INTO module_registry (module_name, is_enabled, enabled_at, dependencies, version)
             VALUES ($1, $2, CASE WHEN $2 THEN NOW() ELSE NULL END, $3, $4)
             ON CONFLICT (module_name)
             DO UPDATE SET is_enabled = $2, enabled_at = CASE WHEN $2 THEN NOW() ELSE NULL END, dependencies = $3, version = $4"
        )
        .bind(module_name)
        .bind(is_enabled)
        .bind(serde_json::json!([])) // TODO: Store actual dependencies
        .bind("0.0.0") // TODO: Store actual version
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
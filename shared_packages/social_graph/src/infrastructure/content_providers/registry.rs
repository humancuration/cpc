//! Registry for dynamic content provider management

use crate::domain::model::{ContentProvider, ContentType};
use crate::domain::service::consent_service::ConsentService;
use crate::infrastructure::consent_middleware::ConsentMiddleware;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use uuid::Uuid;
use semver::Version;

/// Metadata for a content provider
#[derive(Debug, Clone)]
pub struct ProviderMetadata {
    /// Unique identifier for the provider
    pub id: Uuid,
    
    /// Name of the provider
    pub name: String,
    
    /// Content type this provider handles
    pub content_type: ContentType,
    
    /// Version of the provider
    pub version: String,
    
    /// Dependencies required by this provider
    pub dependencies: Vec<String>,
    
    /// State schema version for migration purposes
    pub state_schema_version: String,
    
    /// Compatible previous versions for migration
    pub compatible_previous_versions: Vec<String>,
    
    /// Required interfaces this provider implements
    pub required_interfaces: Vec<String>,
}

/// Listener for content provider registry changes
#[async_trait::async_trait]
pub trait ProviderChangeListener: Send + Sync {
    async fn on_provider_added(&self, provider_id: Uuid);
    async fn on_provider_removed(&self, provider_id: Uuid);
}

/// Error types for dependency resolution
#[derive(Debug)]
pub enum DependencyError {
    /// Circular dependency detected
    CircularDependency(String),
    
    /// Missing dependency
    MissingDependency(String),
    
    /// Version conflict
    VersionConflict {
        dependency: String,
        required: String,
        available: String,
    },
    
    /// Invalid version format
    InvalidVersion(String),
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::CircularDependency(dep) => {
                write!(f, "Circular dependency detected: {}", dep)
            }
            DependencyError::MissingDependency(dep) => {
                write!(f, "Missing dependency: {}", dep)
            }
            DependencyError::VersionConflict { dependency, required, available } => {
                write!(f, "Version conflict for {}: required {}, available {}", dependency, required, available)
            }
            DependencyError::InvalidVersion(version) => {
                write!(f, "Invalid version format: {}", version)
            }
        }
    }
}

impl std::error::Error for DependencyError {}

/// Resolves dependencies between content providers
pub struct DependencyResolver {
    metadata: std::collections::HashMap<Uuid, ProviderMetadata>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Add provider metadata to the resolver
    pub fn add_metadata(&mut self, metadata: ProviderMetadata) {
        self.metadata.insert(metadata.id, metadata);
    }
    
    /// Resolve dependencies for a provider
    pub fn resolve(&self, metadata: &ProviderMetadata) -> Result<(), DependencyError> {
        let mut visited = std::collections::HashSet::new();
        self.resolve_dependencies(metadata, &mut visited)
    }
    
    /// Recursively resolve dependencies
    fn resolve_dependencies(
        &self,
        metadata: &ProviderMetadata,
        visited: &mut std::collections::HashSet<Uuid>,
    ) -> Result<(), DependencyError> {
        // Check for circular dependencies
        if !visited.insert(metadata.id) {
            return Err(DependencyError::CircularDependency(metadata.name.clone()));
        }
        
        // Check each dependency
        for dependency_name in &metadata.dependencies {
            // Find the dependency in our metadata
            let dependency_metadata = self.metadata.values()
                .find(|m| &m.name == dependency_name)
                .ok_or_else(|| DependencyError::MissingDependency(dependency_name.clone()))?;
            
            // Check version compatibility using semver
            // For this implementation, we'll assume the dependency name might include version info
            // like "provider_name@1.2.3" or just "provider_name"
            if let Some(at_pos) = dependency_name.find('@') {
                let dep_name = &dependency_name[..at_pos];
                let required_version_str = &dependency_name[at_pos + 1..];
                
                if dep_name == &dependency_metadata.name {
                    let required_version = Version::parse(required_version_str)
                        .map_err(|_| DependencyError::InvalidVersion(required_version_str.to_string()))?;
                    let available_version = Version::parse(&dependency_metadata.version)
                        .map_err(|_| DependencyError::InvalidVersion(dependency_metadata.version.clone()))?;
                    
                    // Check if the available version satisfies the required version
                    // For simplicity, we'll just check if they're equal
                    // In a real implementation, we would use semver requirements
                    if required_version != available_version {
                        return Err(DependencyError::VersionConflict {
                            dependency: dep_name.to_string(),
                            required: required_version_str.to_string(),
                            available: dependency_metadata.version.clone(),
                        });
                    }
                }
            }
            
            // Recursively check the dependency's dependencies
            self.resolve_dependencies(dependency_metadata, visited)?;
        }
        
        // Remove from visited when backtracking
        visited.remove(&metadata.id);
        
        Ok(())
    }
    
    /// Check for circular dependencies in the entire dependency graph
    pub fn check_circular_dependencies(&self) -> Result<(), DependencyError> {
        for metadata in self.metadata.values() {
            let mut visited = std::collections::HashSet::new();
            self.check_circular_for_provider(metadata, &mut visited)?;
        }
        Ok(())
    }
    
    /// Check for circular dependencies for a specific provider
    fn check_circular_for_provider(
        &self,
        metadata: &ProviderMetadata,
        visited: &mut std::collections::HashSet<Uuid>,
    ) -> Result<(), DependencyError> {
        if !visited.insert(metadata.id) {
            return Err(DependencyError::CircularDependency(metadata.name.clone()));
        }
        
        for dependency_name in &metadata.dependencies {
            let dependency_metadata = self.metadata.values()
                .find(|m| &m.name == dependency_name)
                .ok_or_else(|| DependencyError::MissingDependency(dependency_name.clone()))?;
            
            self.check_circular_for_provider(dependency_metadata, visited)?;
        }
        
        visited.remove(&metadata.id);
        Ok(())
    }
}

/// Registry for content providers
pub struct ContentProviderRegistry {
    providers: RwLock<HashMap<Uuid, Arc<dyn ContentProvider>>>,
    metadata: RwLock<HashMap<Uuid, ProviderMetadata>>,
    consent_service: Arc<dyn ConsentService>,
    listeners: RwLock<Vec<Arc<dyn ProviderChangeListener>>>,
}

impl ContentProviderRegistry {
    /// Create a new content provider registry
    pub fn new(consent_service: Arc<dyn ConsentService>) -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            metadata: RwLock::new(HashMap::new()),
            consent_service,
            listeners: RwLock::new(Vec::new()),
        }
    }
    
    /// Add a change listener
    pub fn add_change_listener(&self, listener: Arc<dyn ProviderChangeListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }
    
    /// Register a new content provider
    pub fn register_provider(
        &self,
        provider: Arc<dyn ContentProvider>,
        metadata: ProviderMetadata,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        // Validate dependencies using DependencyResolver
        let mut resolver = DependencyResolver::new();
        
        // Add all existing metadata to the resolver
        {
            let meta = self.metadata.read()?;
            for existing_metadata in meta.values() {
                resolver.add_metadata(existing_metadata.clone());
            }
        }
        
        // Add the new metadata to the resolver
        resolver.add_metadata(metadata.clone());
        
        // Check for circular dependencies
        resolver.check_circular_dependencies()?;
        
        // Resolve dependencies for the new provider
        resolver.resolve(&metadata)?;
        
        let provider_id = metadata.id;
        
        // Wrap provider with consent middleware
        let wrapped_provider = Arc::new(ConsentMiddleware::new(
            provider,
            self.consent_service.clone()
        ));
        
        // Store the provider
        {
            let mut providers = self.providers.write()?;
            providers.insert(provider_id, wrapped_provider);
        }
        
        // Store the metadata
        {
            let mut meta = self.metadata.write()?;
            meta.insert(provider_id, metadata);
        }
        
        // Notify listeners
        {
            let listeners = self.listeners.read().unwrap();
            for listener in listeners.iter() {
                // In a real implementation, this would be async
                // For now, we'll use a simple approach
                tokio::spawn({
                    let listener = listener.clone();
                    let provider_id = provider_id.clone();
                    async move {
                        listener.on_provider_added(provider_id).await;
                    }
                });
            }
        }
        
        Ok(provider_id)
    }
    
    /// Update an existing content provider with state migration
    pub fn update_provider(
        &self,
        provider: Arc<dyn ContentProvider>,
        metadata: ProviderMetadata,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let provider_id = metadata.id;
        
        // Validate dependencies using DependencyResolver
        let mut resolver = DependencyResolver::new();
        
        // Add all existing metadata to the resolver
        {
            let meta = self.metadata.read()?;
            for existing_metadata in meta.values() {
                // Skip the provider we're updating
                if existing_metadata.id != provider_id {
                    resolver.add_metadata(existing_metadata.clone());
                }
            }
        }
        
        // Add the new metadata to the resolver
        resolver.add_metadata(metadata.clone());
        
        // Check for circular dependencies
        resolver.check_circular_dependencies()?;
        
        // Resolve dependencies for the new provider
        resolver.resolve(&metadata)?;
        
        // Serialize state from the old provider if it exists
        let serialized_state = {
            let providers = self.providers.read()?;
            if let Some(old_provider) = providers.get(&provider_id) {
                old_provider.serialize_state()
                    .map_err(|_| crate::domain::model::ContentProviderError::StateSerializationError)?
            } else {
                Vec::new() // No state to migrate
            }
        };
        
        // Wrap provider with consent middleware
        let wrapped_provider = Arc::new(ConsentMiddleware::new(
            provider,
            self.consent_service.clone()
        ));
        
        // Deserialize state to the new provider if there was state to migrate
        if !serialized_state.is_empty() {
            wrapped_provider.deserialize_state(&serialized_state)
                .map_err(|_| crate::domain::model::ContentProviderError::StateDeserializationError)?;
        }
        
        // Atomically swap the provider using RwLock
        {
            let mut providers = self.providers.write()?;
            providers.insert(provider_id, wrapped_provider);
        }
        
        // Update the metadata
        {
            let mut meta = self.metadata.write()?;
            meta.insert(provider_id, metadata);
        }
        
        // Notify listeners
        {
            let listeners = self.listeners.read().unwrap();
            for listener in listeners.iter() {
                // In a real implementation, this would be async
                // For now, we'll use a simple approach
                tokio::spawn({
                    let listener = listener.clone();
                    let provider_id = provider_id.clone();
                    async move {
                        listener.on_provider_added(provider_id).await;
                    }
                });
            }
        }
        
        Ok(provider_id)
    }
    
    /// Get all provider metadata
    pub fn get_all_metadata(&self) -> Result<Vec<ProviderMetadata>, Box<dyn std::error::Error>> {
        let meta = self.metadata.read()?;
        Ok(meta.values().cloned().collect())
    }
    
    /// Get all registered providers
    pub fn get_all_providers(&self) -> Result<Vec<Arc<dyn ContentProvider>>, Box<dyn std::error::Error>> {
        let providers = self.providers.read()?;
        Ok(providers.values().cloned().collect())
    }
    
    /// Unregister a provider by ID
    pub fn unregister_provider(&self, provider_id: &Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        // Remove from providers
        let mut providers = self.providers.write()?;
        let removed_provider = providers.remove(provider_id);
        
        // Remove from metadata
        let mut metadata = self.metadata.write()?;
        let removed_metadata = metadata.remove(provider_id);
        
        // Notify listeners if provider was actually removed
        if removed_provider.is_some() {
            let listeners = self.listeners.read().unwrap();
            for listener in listeners.iter() {
                let listener = listener.clone();
                let provider_id = provider_id.clone();
                tokio::spawn(async move {
                    listener.on_provider_removed(provider_id).await;
                });
            }
        }
        
        Ok(removed_provider.is_some() && removed_metadata.is_some())
    }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::content_providers::SocialPostProvider;
    use crate::domain::model::ContentType;
    use crate::infrastructure::consent_service_impl::ConsentServiceImpl;
    use crate::infrastructure::in_memory_repository::InMemoryRelationshipRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    #[test]
    fn test_registry_creation() {
        let repository = Arc::new(InMemoryRelationshipRepository::new());
        let consent_service = Arc::new(ConsentServiceImpl::new(repository));
        let registry = ContentProviderRegistry::new(consent_service);
        assert!(registry.get_all_providers().unwrap().is_empty());
    }
    
    #[test]
    fn test_provider_registration() {
        let repository = Arc::new(InMemoryRelationshipRepository::new());
        let consent_service = Arc::new(ConsentServiceImpl::new(repository));
        let registry = ContentProviderRegistry::new(consent_service);
        let provider = Arc::new(SocialPostProvider);
        let metadata = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "TestProvider".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec![],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        let result = registry.register_provider(provider, metadata);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_get_all_providers() {
        let repository = Arc::new(InMemoryRelationshipRepository::new());
        let consent_service = Arc::new(ConsentServiceImpl::new(repository));
        let registry = ContentProviderRegistry::new(consent_service);
        let provider = Arc::new(SocialPostProvider);
        let metadata = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "TestProvider".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec![],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        registry.register_provider(provider, metadata).unwrap();
        let providers = registry.get_all_providers().unwrap();
        assert_eq!(providers.len(), 1);
    }
    
    #[test]
    fn test_dependency_resolver() {
        let mut resolver = DependencyResolver::new();
        
        let metadata1 = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "Provider1".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec![],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        let metadata2 = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "Provider2".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec!["Provider1".to_string()],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        resolver.add_metadata(metadata1.clone());
        resolver.add_metadata(metadata2.clone());
        
        // Should resolve successfully
        assert!(resolver.resolve(&metadata2).is_ok());
        
        // Check for circular dependencies
        assert!(resolver.check_circular_dependencies().is_ok());
    }
    
    #[test]
    fn test_dependency_resolver_circular_dependency() {
        let mut resolver = DependencyResolver::new();
        
        let metadata1 = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "Provider1".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec!["Provider2".to_string()],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        let metadata2 = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "Provider2".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec!["Provider1".to_string()],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        resolver.add_metadata(metadata1.clone());
        resolver.add_metadata(metadata2.clone());
        
        // Should detect circular dependency
        assert!(resolver.check_circular_dependencies().is_err());
    }
    
    #[test]
    fn test_update_provider() {
        let repository = Arc::new(InMemoryRelationshipRepository::new());
        let consent_service = Arc::new(ConsentServiceImpl::new(repository));
        let registry = ContentProviderRegistry::new(consent_service);
        
        let provider = Arc::new(SocialPostProvider);
        let metadata = ProviderMetadata {
            id: Uuid::new_v4(),
            name: "TestProvider".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.0.0".to_string(),
            dependencies: vec![],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        };
        
        // Register the provider
        let provider_id = registry.register_provider(provider.clone(), metadata.clone()).unwrap();
        
        // Update the provider
        let updated_metadata = ProviderMetadata {
            id: provider_id,
            name: "TestProvider".to_string(),
            content_type: ContentType::SocialPost,
            version: "1.1.0".to_string(),
            dependencies: vec![],
            state_schema_version: "1.1.0".to_string(),
            compatible_previous_versions: vec!["1.0.0".to_string()],
            required_interfaces: vec![],
        };
        
        let result = registry.update_provider(provider, updated_metadata);
        assert!(result.is_ok());
    }
}
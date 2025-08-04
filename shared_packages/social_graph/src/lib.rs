//! Social Graph - A package for managing social relationships and interactions
//!
//! This package provides functionality for managing user relationships, activities,
//! and social graph operations within the CPC ecosystem.

pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod application;

pub use domain::model::{User, Relationship, RelationshipType, Activity, ActivityType, ContentType, Visibility, ContentItem, FeedFilter};
pub use domain::repository::RelationshipRepository;
pub use infrastructure::consent_adapter::ConsentAdapter;
pub use infrastructure::in_memory_repository::InMemoryRelationshipRepository;
pub use infrastructure::postgres_repository::PostgresRelationshipRepository;
pub use application::SocialService;
pub use presentation::graphql::{create_schema, SocialGraphSchema};

// Re-export GraphQL types for convenience
pub use presentation::graphql::{
    GraphQLUser,
    GraphQLActivity,
    GraphQLRelationship,
    GraphQLActivityType,
    GraphQLRelationshipType,
    GraphQLContentType,
    GraphQLVisibility,
    GraphQLActivityFeedItem
};

// Re-export consent_manager for convenience
pub use consent_manager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // This test ensures that all expected modules and types are properly exported
        // and accessible from the crate root.
        // It's a placeholder test that will be expanded with actual tests.
        assert!(true);
    }
}
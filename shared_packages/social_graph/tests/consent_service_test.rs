//! Tests for the consent service implementation

use social_graph::{
    domain::{
        model::{Visibility},
        service::consent_service::{ConsentService, ConsentError},
    },
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_consent_service_public_content() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = ConsentServiceImpl::new(repository);
    
    let viewer_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    
    // Public content should be visible to everyone
    let can_view = consent_service
        .can_view_content(viewer_id, owner_id, Visibility::Public)
        .await
        .unwrap();
    
    assert!(can_view);
}

#[tokio::test]
async fn test_consent_service_private_content() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = ConsentServiceImpl::new(repository);
    
    let user_id = Uuid::new_v4();
    
    // Private content should only be visible to the owner
    let can_view = consent_service
        .can_view_content(user_id, user_id, Visibility::Private)
        .await
        .unwrap();
    
    assert!(can_view);
    
    // Private content should not be visible to others
    let other_user_id = Uuid::new_v4();
    let can_view = consent_service
        .can_view_content(other_user_id, user_id, Visibility::Private)
        .await
        .unwrap();
    
    assert!(!can_view);
}
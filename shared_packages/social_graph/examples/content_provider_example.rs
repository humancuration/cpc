//! Example demonstrating the ContentProvider system for universal feed

use social_graph::{
    application::SocialService,
    domain::{
        model::{ContentType, FeedFilter},
        repository::RelationshipRepository,
    },
    infrastructure::{
        content_providers::register_providers,
        in_memory_repository::InMemoryRelationshipRepository,
        consent_adapter::ConsentAdapter,
    },
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ContentProvider System Example");
    println!("============================");
    
    // Create repository and consent adapter
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = consent_manager::ConsentService::new();
    let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));
    
    // Create social service
    let mut social_service = SocialService::new(repository, consent_adapter);
    
    // Register content providers
    register_providers(&mut social_service);
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await?;
    
    println!("Universal feed contains {} items", feed.len());
    
    for (i, item) in feed.iter().enumerate() {
        println!("{}. {:?} from {} (score: {})",
                 i + 1,
                 item.content_type,
                 item.source_package,
                 item.relevance_score);
    }
    
    // Test filtering by content type
    println!("\nFiltering by SocialPost content type:");
    let social_post_filter = vec![FeedFilter {
        content_type: Some(ContentType::SocialPost),
        package: None,
        visibility: None,
    }];
    
    let social_posts = social_service.get_universal_feed(
        user_id,
        None,
        10,
        Some(social_post_filter)
    ).await?;
    
    println!("Found {} social posts", social_posts.len());
    
    Ok(())
}
mod social_post;
mod video;
mod registry;

use crate::domain::model::ContentProvider;
use std::sync::Arc;

pub use social_post::SocialPostProvider;
pub use video::VideoProvider;
pub use registry::{ContentProviderRegistry, ProviderMetadata, ProviderChangeListener, DependencyError, DependencyResolver};

pub fn create_default_providers() -> Vec<Arc<dyn ContentProvider>> {
    vec![
        Arc::new(SocialPostProvider) as Arc<dyn ContentProvider>,
        Arc::new(VideoProvider) as Arc<dyn ContentProvider>,
    ]
}

pub fn create_default_registry(consent_service: Arc<dyn ConsentService>) -> Arc<ContentProviderRegistry> {
    let registry = Arc::new(ContentProviderRegistry::new(consent_service));
    
    // Register default providers
    let social_post_provider = Arc::new(SocialPostProvider) as Arc<dyn ContentProvider>;
    let video_provider = Arc::new(VideoProvider) as Arc<dyn ContentProvider>;
    
    let social_post_metadata = ProviderMetadata {
        id: uuid::Uuid::new_v4(),
        name: "SocialPostProvider".to_string(),
        content_type: crate::domain::model::ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    let video_metadata = ProviderMetadata {
        id: uuid::Uuid::new_v4(),
        name: "VideoProvider".to_string(),
        content_type: crate::domain::model::ContentType::Video,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    registry.register_provider(social_post_provider, social_post_metadata).unwrap();
    registry.register_provider(video_provider, video_metadata).unwrap();
    
    registry
}
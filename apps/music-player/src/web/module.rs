//! Module initialization and wiring for the music player

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::application::streaming_service::StreamingService;
use crate::application::social_service::SocialService;
use crate::application::visualizer_service::VisualizerService;
use crate::application::cache_service::CacheService;
use crate::application::privacy_service::PrivacyService;
use crate::infrastructure::database::{
    TrackRepository, CommentRepository, InteractionRepository,
    VisualizerRepository, CacheRepository
};
use crate::infrastructure::database::{
    consent_repository::ConsentRepository,
    pg_consent_repository::PgConsentRepository,
};
use crate::infrastructure::p2p::P2PStreamManager;
use crate::infrastructure::audio_processor::AudioProcessor;
use crate::web::graphql::{MusicPlayerQuery, MusicPlayerMutation, MusicPlayerSubscription};
use crate::web::routes::create_music_player_router;

/// This struct holds all the pieces the backend needs from this module
pub struct MusicPlayerModule {
    pub router: Router,
    pub query: MusicPlayerQuery,
    pub mutation: MusicPlayerMutation,
    pub subscription: MusicPlayerSubscription,
}

/// This function initializes the module and its dependencies
pub fn initialize(db_pool: PgPool) -> MusicPlayerModule {
    // Initialize infrastructure components
    let db_pool = Arc::new(db_pool);
    let track_repository = Arc::new(TrackRepository::new(db_pool.clone()));
    let comment_repository = Arc::new(CommentRepository::new(db_pool.clone()));
    let interaction_repository = Arc::new(InteractionRepository::new(db_pool.clone()));
    let visualizer_repository = Arc::new(VisualizerRepository::new(db_pool.clone()));
    let cache_repository = Arc::new(CacheRepository::new(db_pool.clone()));
    let consent_repository = Arc::new(PgConsentRepository::new(db_pool.clone()));
    
    let p2p_manager = Arc::new(P2PStreamManager::new(vec![
        "stun.l.google.com:19302".to_string(),
        "stun1.l.google.com:19302".to_string(),
    ]).expect("Failed to initialize P2P manager"));
    
    let audio_processor = Arc::new(AudioProcessor::new());
    
    let privacy_service = Arc::new(PrivacyService::new(consent_repository));

    // Initialize application services
    let streaming_service = Arc::new(StreamingService::new(
        track_repository.clone(),
        p2p_manager.clone(),
        audio_processor.clone(),
        privacy_service.clone(),
    ));
    
    let social_service = Arc::new(SocialService::new(
        comment_repository.clone(),
        interaction_repository.clone(),
        privacy_service.clone(),
    ));
    
    let visualizer_service = Arc::new(VisualizerService::new(
        visualizer_repository.clone(),
        audio_processor.clone(),
    ));
    let cache_service = Arc::new(CacheService::new(
        cache_repository.clone(),
        privacy_service.clone(),
    ));
    ));

    // Initialize web components
    let router = create_music_player_router(
        streaming_service.clone(),
        social_service.clone(),
        cache_service.clone(),
    );

    let query = MusicPlayerQuery;
    let mutation = MusicPlayerMutation;
    let subscription = MusicPlayerSubscription;

    MusicPlayerModule {
        router,
        query,
        mutation,
        subscription,
    }
}
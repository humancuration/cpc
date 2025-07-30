use axum::{Router, routing::get};
use std::sync::Arc;
use sled::Config;
use async_channel::unbounded;

use yapper::infrastructure::{
    post_repository::PostRepository,
    feed_repository::FeedRepository,
    user_profile_repository::UserProfileRepository,
    user_repository::SledUserRepository,
    session_repository::SledSessionRepository,
    password_reset_repository::SledPasswordResetRepository,
    event_publisher::ChannelEventPublisher,
    auth_service_client::YapperAuthServiceClient,
};
use yapper::application::{
    post_service::PostService,
    auth_service::YapperAuthService,
    user_service::UserServiceImpl,
    session_service::SessionServiceImpl,
};
use yapper::api::routes;
use yapper::domain::events::EventPublisher;
use yapper::domain::auth_service::AuthService;
use yapper::domain::user_repository::UserRepository;
use yapper::domain::session_repository::SessionRepository;
use yapper::domain::password_reset_repository::PasswordResetRepository;
use yapper::domain::session_management::SessionManagement;
use yapper::infrastructure::consent_manager::YapperConsentManager;
use cpc_rbac::RbacEngine;
use cpc_karma::KarmaService;
use cpc_consent::ConsentService;

#[tokio::main]
async fn main() {
    println!("Yapper - Twitter-style microblogging");

    // Initialize database
    let db_config = Config::new().path("yapper.db");
    let db = db_config.open().expect("Failed to open database");

    // Initialize repositories
    let post_repo = PostRepository::new(db.clone());
    let _feed_repo = FeedRepository::new(db.clone());
    let _user_profile_repo = UserProfileRepository::new(db.clone());
    let user_repo = Arc::new(SledUserRepository::new(db.clone())) as Arc<dyn UserRepository>;
    let session_repo = Arc::new(SledSessionRepository::new(db.clone())) as Arc<dyn SessionRepository>;
    let password_reset_repo = Arc::new(SledPasswordResetRepository::new(db.clone())) as Arc<dyn PasswordResetRepository>;
    
    // Initialize auth service client
    let auth_client = Arc::new(YapperAuthServiceClient::new("http://[::1]:50051".to_string()).await
        .expect("Failed to connect to auth service"));

    // Initialize event system
    let (sender, receiver) = unbounded();
    let event_publisher = Arc::new(ChannelEventPublisher::new(sender));

    // Initialize RBAC engine
    let rbac_engine = Arc::new(RbacEngine::new());
    
    // Initialize karma service
    let karma_service = Arc::new(KarmaService::new());
    
    // Initialize consent service
    let consent_service = Arc::new(ConsentService::new());
    let consent_manager = Arc::new(YapperConsentManager::new(consent_service));
    
    // Initialize session service with auth client
    let session_service = Arc::new(SessionServiceImpl::new(auth_client.clone()));
    
    // Initialize services
    let post_service = Arc::new(PostService::new(post_repo, event_publisher.clone()));
    let auth_service = Arc::new(YapperAuthService::new(
        user_repo.clone(),
        session_repo.clone(),
        password_reset_repo.clone(),
        event_publisher.clone(),
    )) as Arc<dyn AuthService>;
    let _user_service = Arc::new(UserServiceImpl::new(user_repo.clone(), karma_service.clone()));

    // Set up API routes
    let app = Router::new()
        .nest("/api", routes::routes(post_service, auth_service, consent_manager.clone()))
        .route("/health", get(|| async { "OK" }));
    
    // Add RBAC middleware
    // In a real implementation, you would add the middleware to specific routes
    // that require RBAC checks

    // Start event listener in background task
    tokio::spawn(async move {
        while let Ok(event) = receiver.recv().await {
            println!("Received event: {:?}", event);
            // In a real implementation, we would process the event here
        }
    });

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
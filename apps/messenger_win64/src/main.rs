//! Main entry point for the Messenger application

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::{info, error};
use tracing_subscriber;
use async_graphql::Schema;

use cpc_messenger::infrastructure::{
    websocket::WebSocketServer,
};
use cpc_messenger::services::{
    reaction::ReactionServiceImpl,
    thread::ThreadServiceImpl,
    group::GroupServiceImpl,
    moderation::ModerationServiceImpl,
};
use cpc_messenger::repositories::{
    reaction::ReactionRepository,
    thread::ThreadRepository,
    group::GroupRepository,
    media::MediaRepository,
};
use messenger_domain::services::{ConversationService, MessageService, MediaService, PresenceService};
use messenger_domain::graphql::{Mutation, Subscription};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Messenger application");
    
    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/messenger".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    // Initialize Sled database for presence
    let sled_path = std::env::var("SLED_PATH")
        .unwrap_or_else(|_| "./data/sled".to_string());
    
    // Create repositories
    let conversation_repository = PostgresConversationRepository::new(pool.clone());
    let message_repository = PostgresMessageRepository::new(pool.clone());
    let media_repository = PostgresMediaRepository::new(pool.clone());
    let presence_repository = SledPresenceRepository::new(&sled_path)?;
    
    // Initialize our new repositories
    let reaction_repository = ReactionRepository::new(/* social reaction service */);
    let thread_repository = ThreadRepository::new(pool.clone());
    let group_repository = GroupRepository::new(pool.clone());
    let our_media_repository = MediaRepository::new(pool.clone());
    
    // Initialize external services
    // In a real implementation, we would initialize the consent manager
    // For now, we'll create a mock implementation
    let consent_manager = ConsentManagerImpl::new(std::sync::Arc::new(
        // This would be a real consent service in production
        todo!("Initialize real consent manager")
    ));
    
    // Create application services
    let conversation_service = ConversationServiceImpl::new(
        std::sync::Arc::new(conversation_repository),
        std::sync::Arc::new(consent_manager),
    );
    
    let message_service = MessageServiceImpl::new(
        std::sync::Arc::new(message_repository),
        std::sync::Arc::new(conversation_service),
        std::sync::Arc::new(consent_manager),
    );
    
    let media_service = MediaServiceImpl::new(
        std::sync::Arc::new(media_repository),
        std::sync::Arc::new(consent_manager),
    );
    
    let presence_service = PresenceServiceImpl::new(
        std::sync::Arc::new(presence_repository),
    );
    
    // Create our new services
    let reaction_service = ReactionServiceImpl::new(std::sync::Arc::new(reaction_repository));
    let thread_service = ThreadServiceImpl::new(std::sync::Arc::new(thread_repository));
    let group_service = GroupServiceImpl::new(std::sync::Arc::new(group_repository));
    let moderation_service = ModerationServiceImpl::new();
    let our_media_service = MediaServiceImpl::new(std::sync::Arc::new(our_media_repository));
    
    // Create GraphQL schema
    let schema = async_graphql::Schema::build(
        async_graphql::EmptyMutation,
        messenger_domain::graphql::Mutation,
        messenger_domain::graphql::Subscription
    )
    .data(std::sync::Arc::new(conversation_service))
    .data(std::sync::Arc::new(message_service))
    .data(std::sync::Arc::new(presence_service))
    .data(std::sync::Arc::new(reaction_service))
    .data(std::sync::Arc::new(thread_service))
    .data(std::sync::Arc::new(group_service))
    .finish();
    
    // Create WebSocket server
    let websocket_server = WebSocketServer::new(std::sync::Arc::new(message_service));
    
    // Create OAuth2 identity provider
    let identity_provider = OAuth2IdentityProvider::new();
    
    // Build the Axum application
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/ws", get(websocket_handler))
        .route("/health", get(health_check))
        .with_state(AppState {
            schema,
            websocket_server,
        });
    
    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

// Application state
#[derive(Clone)]
struct AppState {
    schema: async_graphql::Schema<async_graphql::EmptyMutation, messenger_domain::graphql::Mutation, messenger_domain::graphql::Subscription>,
    websocket_server: WebSocketServer,
}

// GraphQL playground handler
async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
            <link rel="shortcut icon" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
            <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
        </head>
        <body>
            <div id="root"></div>
            <script>
                window.addEventListener('load', function (event) {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/graphql'
                    });
                });
            </script>
        </body>
        </html>
        "#,
    )
}

// GraphQL handler
async fn graphql_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    req.into_inner().execute(&state.schema).await.into()
}

// WebSocket handler
async fn websocket_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    ws: axum::extract::ws::WebSocketUpgrade,
) -> axum::response::Response {
    ws.on_upgrade(|socket| async move {
        // In a real implementation, we would authenticate the user
        // and pass their user_id to the WebSocket server
        let user_id = uuid::Uuid::nil(); // Placeholder
        
        if let Err(e) = state.websocket_server.handle_connection(user_id, socket).await {
            error!("WebSocket connection error: {}", e);
        }
    })
}

// Health check handler
async fn health_check() -> impl axum::response::IntoResponse {
    "OK"
}
use cpc_auth::auth_service::AuthServiceImpl as BaseAuthService;
use crate::domain::auth::{Credentials, User};
use crate::domain::auth_service::{AuthService, AllatAuthService};
use crate::infrastructure::repositories::user_repository::SledUserRepository;
use crate::infrastructure::repositories::community_repo::{CommunityRepository, PgCommunityRepository};
use crate::infrastructure::repositories::post_repo::{PostRepository, PgPostRepository};
use crate::infrastructure::repositories::comment_repo::{CommentRepository, PgCommentRepository};
use crate::infrastructure::repositories::vote_repo::{VoteRepository, PgVoteRepository};
use crate::application::community_service::{CommunityService, CommunityServiceImpl};
use crate::application::post_service::{PostService, PostServiceImpl};
use crate::application::comment_service::{CommentService, CommentServiceImpl};
use crate::application::vote_service::{VoteService, VoteServiceImpl};
use crate::api::schema::create_schema;
use std::sync::Arc;
use sqlx::PgPool;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Extension;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

mod domain {
    pub mod auth;
    pub mod auth_service;
    pub mod community;
    pub mod post;
    pub mod comment;
    pub mod vote;
    pub mod media_asset;
    pub mod karma_update_service;
}

mod infrastructure {
    pub mod community_repository;
    pub mod post_repository;
    pub mod event_bus;
    pub mod repositories {
        pub mod user_repository;
        pub mod community_repo;
        pub mod post_repo;
        pub mod comment_repo;
        pub mod vote_repo;
    }
    pub mod middleware;
}

mod application {
    pub mod community_service;
    pub mod comment_service;
    pub mod error;
    pub mod moderation_service;
    pub mod post_service;
    pub mod vote_service;
}

mod api {
    pub mod schema;
    pub mod queries;
    pub mod mutations;
    pub mod subscriptions;
    pub mod objects;
}

async fn setup_database() -> Result<PgPool, Box<dyn std::error::Error>> {
    // Get database URL from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/allat_dev".to_string());
    
    // Create connection pool
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

#[tokio::main]
async fn main() {
    println!("Allat - Reddit-style forums");
    
    // Setup database
    let pool = match setup_database().await {
        Ok(pool) => {
            println!("Database connected and migrations applied successfully");
            pool
        },
        Err(e) => {
            eprintln!("Failed to setup database: {:?}", e);
            return;
        }
    };
    
    // Initialize base auth service
    let base_auth_service = Arc::new(BaseAuthService::new());
    
    // Initialize user repository
    let user_db = sled::open("allat_users").expect("Failed to open user DB");
    let user_repo = Arc::new(SledUserRepository::new(user_db));
    
    // Initialize Allat auth service
    let auth_service: Arc<dyn AuthService> = Arc::new(AllatAuthService::new(
        base_auth_service,
        user_repo,
    ));
    
    // Example usage
    let credentials = Credentials {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    match auth_service.register(credentials).await {
        Ok(user) => {
            println!("User registered with ID: {} and initial karma: {}", user.base.id, user.karma);
            
            // Example of incrementing karma
            if let Err(e) = auth_service.increment_karma(user.base.id, 10).await {
                eprintln!("Failed to increment karma: {:?}", e);
            } else {
                println!("Karma incremented successfully");
            }
            
            // Example of retrieving karma
            match auth_service.get_karma(user.base.id).await {
                Ok(karma) => println!("User's current karma: {}", karma),
                Err(e) => eprintln!("Failed to get karma: {:?}", e),
            }
        },
        Err(e) => eprintln!("Failed to register user: {:?}", e),
    }
    
    // Initialize repositories
    let community_repo: Arc<dyn CommunityRepository> = Arc::new(PgCommunityRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PgPostRepository::new(pool.clone()));
    let comment_repo: Arc<dyn CommentRepository> = Arc::new(PgCommentRepository::new(pool.clone()));
    let vote_repo: Arc<dyn VoteRepository> = Arc::new(PgVoteRepository::new(pool.clone()));
    
    // Initialize services
    let community_service: Arc<dyn CommunityService> = Arc::new(CommunityServiceImpl::new(community_repo.clone()));
    let post_service: Arc<dyn PostService> = Arc::new(PostServiceImpl::new(post_repo.clone(), community_repo.clone()));
    let comment_service: Arc<dyn CommentService> = Arc::new(CommentServiceImpl::new(comment_repo.clone(), post_repo.clone()));
    let vote_service: Arc<dyn VoteService> = Arc::new(VoteServiceImpl::new(vote_repo.clone(), post_repo.clone(), auth_service.clone()));
    
    // Create GraphQL schema
    let schema = create_schema(
        community_service.clone(),
        post_service.clone(),
        comment_service.clone(),
        vote_service.clone(),
    );
    
    // Set up Axum router
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .layer(CorsLayer::new().allow_origin(Any));
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("GraphQL server running on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    // Serve GraphQL playground
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
    ))
}

async fn graphql_handler(
    schema: Extension<crate::api::schema::AllatSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.0.execute(req.0).await.into()
}
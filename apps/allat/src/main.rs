use cpc_auth::auth_service::AuthServiceImpl as BaseAuthService;
use crate::domain::auth::{Credentials, User};
use crate::domain::auth_service::{AuthService, AllatAuthService};
use crate::infrastructure::repositories::user_repository::SledUserRepository;
use std::sync::Arc;

mod domain {
    pub mod auth;
    pub mod auth_service;
    pub mod community;
    pub mod post;
    pub mod vote;
    pub mod karma_update_service;
}

mod infrastructure {
    pub mod community_repository;
    pub mod post_repository;
    pub mod event_bus;
    pub mod repositories {
        pub mod user_repository;
    }
    pub mod middleware;
}

mod application {
    pub mod vote_service;
}

#[tokio::main]
async fn main() {
    println!("Allat - Reddit-style forums");
    
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
    
    println!("Authentication system initialized");
}
//! # Auth Service
//!
//! gRPC authentication service for the CPC ecosystem.

use tonic::{transport::Server, Request, Response, Status};
use auth_service::auth_server::{Auth, AuthServer};
use auth_service::{SessionRequest, SessionResponse, CreateSessionRequest, InvalidateSessionRequest, InvalidateSessionResponse};
use cpc_auth::models::{Session, User};
use cpc_auth::auth_service::AuthService;
use cpc_auth::session::RedisSessionStore;
use uuid::Uuid;
use std::sync::Arc;

// Include the generated gRPC code
pub mod auth_service {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct MyAuth {
    session_store: Arc<RedisSessionStore>,
}

impl MyAuth {
    pub fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let session_store = Arc::new(RedisSessionStore::new(redis_url)?);
        Ok(Self { session_store })
    }
}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn validate_session(
        &self,
        request: Request<SessionRequest>,
    ) -> Result<Response<SessionResponse>, Status> {
        let req = request.into_inner();
        
        match Uuid::parse_str(&req.session_id) {
            Ok(session_id) => {
                // Convert UUID to string for Redis
                let session_key = session_id.to_string();
                
                match self.session_store.get_session(&session_key) {
                    Ok(Some(session)) => {
                        let response = SessionResponse {
                            valid: !session.is_expired(),
                            user_id: session.user_id.to_string(),
                            session_id: session.id.to_string(),
                            error_message: String::new(),
                        };
                        Ok(Response::new(response))
                    }
                    Ok(None) => {
                        let response = SessionResponse {
                            valid: false,
                            user_id: String::new(),
                            session_id: String::new(),
                            error_message: "Session not found".to_string(),
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = SessionResponse {
                            valid: false,
                            user_id: String::new(),
                            session_id: String::new(),
                            error_message: format!("Failed to retrieve session: {}", e),
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(_) => {
                let response = SessionResponse {
                    valid: false,
                    user_id: String::new(),
                    session_id: String::new(),
                    error_message: "Invalid session ID format".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
    
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<SessionResponse>, Status> {
        let req = request.into_inner();
        
        match Uuid::parse_str(&req.user_id) {
            Ok(user_id) => {
                let session = Session::new(user_id, req.device_info);
                
                match self.session_store.save_session(&session) {
                    Ok(_) => {
                        let response = SessionResponse {
                            valid: true,
                            user_id: session.user_id.to_string(),
                            session_id: session.id.to_string(),
                            error_message: String::new(),
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = SessionResponse {
                            valid: false,
                            user_id: String::new(),
                            session_id: String::new(),
                            error_message: format!("Failed to save session: {}", e),
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(_) => {
                let response = SessionResponse {
                    valid: false,
                    user_id: String::new(),
                    session_id: String::new(),
                    error_message: "Invalid user ID format".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
    
    async fn invalidate_session(
        &self,
        request: Request<InvalidateSessionRequest>,
    ) -> Result<Response<InvalidateSessionResponse>, Status> {
        let req = request.into_inner();
        
        match Uuid::parse_str(&req.session_id) {
            Ok(_) => {
                let session_key = req.session_id;
                
                match self.session_store.delete_session(&session_key) {
                    Ok(_) => {
                        let response = InvalidateSessionResponse {
                            success: true,
                            error_message: String::new(),
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = InvalidateSessionResponse {
                            success: false,
                            error_message: format!("Failed to delete session: {}", e),
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(_) => {
                let response = InvalidateSessionResponse {
                    success: false,
                    error_message: "Invalid session ID format".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    
    // In a real implementation, you would get this from environment variables
    let redis_url = "redis://127.0.0.1/";
    let auth_service = MyAuth::new(redis_url)?;

    println!("Auth service listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
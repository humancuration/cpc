#[cfg(test)]
mod tests {
    use learning_platform_server::middleware::auth::{AuthMiddleware, Claims};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
    use axum::middleware::Next;
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_auth_middleware_valid_token() {
        // Create a test user ID
        let user_id = Uuid::new_v4();
        
        // Create claims for JWT
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();
        
        let claims = Claims {
            sub: user_id,
            exp: exp as usize,
        };
        
        // Create JWT token
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_bytes()))
            .expect("Failed to create token");
        
        // Create headers with valid token
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        
        // Create a test request
        let request = Request::builder().body(()).unwrap();
        
        // Create a test next function
        let next = Next::new(|req| async move { 
            Ok(axum::response::Response::new(req)) 
        });
        
        // Test the middleware
        let result = AuthMiddleware::auth_middleware(headers, request, next).await;
        
        // Should succeed
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_auth_middleware_invalid_token() {
        // Create headers with invalid token
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str("Bearer invalid_token").unwrap());
        
        // Create a test request
        let request = Request::builder().body(()).unwrap();
        
        // Create a test next function
        let next = Next::new(|req| async move { 
            Ok(axum::response::Response::new(req)) 
        });
        
        // Test the middleware
        let result = AuthMiddleware::auth_middleware(headers, request, next).await;
        
        // Should fail with unauthorized
        assert_eq!(result, Err(StatusCode::UNAUTHORIZED));
    }
    
    #[tokio::test]
    async fn test_auth_middleware_missing_header() {
        // Create headers without authorization header
        let headers = HeaderMap::new();
        
        // Create a test request
        let request = Request::builder().body(()).unwrap();
        
        // Create a test next function
        let next = Next::new(|req| async move { 
            Ok(axum::response::Response::new(req)) 
        });
        
        // Test the middleware
        let result = AuthMiddleware::auth_middleware(headers, request, next).await;
        
        // Should fail with unauthorized
        assert_eq!(result, Err(StatusCode::UNAUTHORIZED));
    }
}
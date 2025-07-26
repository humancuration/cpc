use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use crate::auth::AuthenticatedUser;

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub max_requests_per_minute: u32,
    pub max_requests_per_hour: u32,
    pub enable_csrf_protection: bool,
    pub enable_content_security_policy: bool,
    pub enable_request_logging: bool,
    pub max_request_size: usize,
    pub blocked_user_agents: Vec<String>,
    pub allowed_origins: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            max_requests_per_hour: 1000,
            enable_csrf_protection: true,
            enable_content_security_policy: true,
            enable_request_logging: true,
            max_request_size: 10 * 1024 * 1024, // 10MB
            blocked_user_agents: vec![
                "bot".to_string(),
                "crawler".to_string(),
                "spider".to_string(),
            ],
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8080".to_string(),
                "https://localhost:3000".to_string(),
                "https://localhost:8080".to_string(),
            ],
        }
    }
}

#[derive(Debug)]
struct RateLimitEntry {
    requests_per_minute: u32,
    requests_per_hour: u32,
    minute_window_start: Instant,
    hour_window_start: Instant,
}

impl RateLimitEntry {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            requests_per_minute: 0,
            requests_per_hour: 0,
            minute_window_start: now,
            hour_window_start: now,
        }
    }

    fn check_and_increment(&mut self, config: &SecurityConfig) -> bool {
        let now = Instant::now();
        
        // Reset minute window if needed
        if now.duration_since(self.minute_window_start) >= Duration::from_secs(60) {
            self.requests_per_minute = 0;
            self.minute_window_start = now;
        }
        
        // Reset hour window if needed
        if now.duration_since(self.hour_window_start) >= Duration::from_secs(3600) {
            self.requests_per_hour = 0;
            self.hour_window_start = now;
        }
        
        // Check limits
        if self.requests_per_minute >= config.max_requests_per_minute {
            return false;
        }
        
        if self.requests_per_hour >= config.max_requests_per_hour {
            return false;
        }
        
        // Increment counters
        self.requests_per_minute += 1;
        self.requests_per_hour += 1;
        
        true
    }
}

#[derive(Debug)]
pub struct SecurityMiddleware {
    config: SecurityConfig,
    rate_limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl SecurityMiddleware {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn middleware(
        &self,
        req: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Check user agent for blocked patterns
        if let Some(user_agent) = req.headers().get("user-agent") {
            if let Ok(ua_str) = user_agent.to_str() {
                let ua_lower = ua_str.to_lowercase();
                for blocked_pattern in &self.config.blocked_user_agents {
                    if ua_lower.contains(blocked_pattern) {
                        tracing::warn!("Blocked request from user agent: {}", ua_str);
                        return Err(StatusCode::FORBIDDEN);
                    }
                }
            }
        }
        
        // Check origin for CORS
        if let Some(origin) = req.headers().get("origin") {
            if let Ok(origin_str) = origin.to_str() {
                if !self.config.allowed_origins.contains(&origin_str.to_string()) && 
                   !self.config.allowed_origins.is_empty() {
                    tracing::warn!("Blocked request from origin: {}", origin_str);
                    return Err(StatusCode::FORBIDDEN);
                }
            }
        }
        
        // Extract client identifier (IP address or user ID)
        let client_id = self.get_client_identifier(&req).await;
        
        // Check rate limits
        if !self.check_rate_limit(&client_id).await {
            tracing::warn!("Rate limit exceeded for client: {}", client_id);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
        
        // Log request if enabled
        if self.config.enable_request_logging {
            tracing::info!(
                method = %req.method(),
                uri = %req.uri(),
                client_id = %client_id,
                "Processing request"
            );
        }
        
        // Process request
        let mut response = next.run(req).await;
        
        // Add security headers
        self.add_security_headers(response.headers_mut());
        
        Ok(response)
    }
    
    async fn get_client_identifier(&self, req: &Request) -> String {
        // Try to get user ID from authenticated user
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            return format!("user:{}", user.user_id);
        }
        
        // Fall back to IP address
        if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
            if let Ok(ip) = forwarded_for.to_str() {
                return format!("ip:{}", ip.split(',').next().unwrap_or(ip).trim());
            }
        }
        
        if let Some(real_ip) = req.headers().get("x-real-ip") {
            if let Ok(ip) = real_ip.to_str() {
                return format!("ip:{}", ip);
            }
        }
        
        // Default fallback
        "unknown".to_string()
    }
    
    async fn check_rate_limit(&self, client_id: &str) -> bool {
        let mut rate_limits = self.rate_limits.write().await;
        let entry = rate_limits.entry(client_id.to_string())
            .or_insert_with(RateLimitEntry::new);
        
        entry.check_and_increment(&self.config)
    }
    
    fn add_security_headers(&self, headers: &mut HeaderMap) {
        // Add security headers
        headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
        headers.insert("X-Frame-Options", "DENY".parse().unwrap());
        headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
        headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());
        
        // Add additional security headers for API protection
        headers.insert("X-Permitted-Cross-Domain-Policies", "none".parse().unwrap());
        headers.insert("X-Download-Options", "noopen".parse().unwrap());
        headers.insert("Cache-Control", "no-store, no-cache, must-revalidate, proxy-revalidate".parse().unwrap());
        headers.insert("Pragma", "no-cache".parse().unwrap());
        headers.insert("Expires", "0".parse().unwrap());
        
        if self.config.enable_content_security_policy {
            headers.insert(
                "Content-Security-Policy",
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' ws: wss:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'".parse().unwrap()
            );
        }
        
        // Add HSTS header for HTTPS
        headers.insert(
            "Strict-Transport-Security",
            "max-age=31536000; includeSubDomains; preload".parse().unwrap()
        );
        
        // Add feature policy headers
        headers.insert(
            "Permissions-Policy",
            "geolocation=(), microphone=(), camera=(), payment=(), usb=()".parse().unwrap()
        );
    }
}

pub async fn security_middleware(
    State(security): State<Arc<SecurityMiddleware>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    security.middleware(req, next).await
}

pub async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut().insert("x-request-id", request_id.parse().unwrap());
    
    let mut response = next.run(req).await;
    response.headers_mut().insert("x-request-id", request_id.parse().unwrap());
    
    response
}

pub async fn logging_middleware(
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let request_id = req.headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    tracing::info!(
        request_id = request_id,
        method = %method,
        uri = %uri,
        "Request started"
    );
    
    let response = next.run(req).await;
    let duration = start.elapsed();
    
    tracing::info!(
        request_id = request_id,
        method = %method,
        uri = %uri,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );
    
    response
}
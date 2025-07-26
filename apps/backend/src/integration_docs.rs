//! # Backend Integration Documentation
//! 
//! This module documents the integration of new services from the android-rust-migration
//! with the existing Axum backend infrastructure.
//! 
//! ## Integrated Services
//! 
//! ### Authentication Service
//! - **Endpoint**: `/api/auth/*`
//! - **Features**: User registration, login, token refresh
//! - **Security**: JWT tokens, bcrypt password hashing, rate limiting
//! - **Middleware**: No authentication required for login/register endpoints
//! 
//! ### Social Service
//! - **Endpoint**: `/api/social/*`
//! - **Features**: Posts, comments, likes, follows, user feeds
//! - **Security**: Requires authentication for all endpoints
//! - **Middleware**: `auth_middleware` applied to all routes
//! 
//! ### Forum Service
//! - **Endpoint**: `/api/forum/*`
//! - **Features**: Forums, threads, replies, voting, moderation
//! - **Security**: Optional authentication (some endpoints public, others require auth)
//! - **Middleware**: `optional_auth_middleware` applied to all routes
//! - **Role-based Access**: Forum creation and moderation require specific roles
//! 
//! ### Governance Service
//! - **Endpoint**: `/api/governance/*`
//! - **Features**: Proposals, voting, governance participation
//! - **Security**: Requires authentication for all endpoints
//! - **Middleware**: `auth_middleware` applied to all routes
//! - **Role-based Access**: Proposal creation requires specific roles
//! 
//! ## Security Measures
//! 
//! ### Rate Limiting
//! - **Per-minute limit**: 60 requests per client
//! - **Per-hour limit**: 1000 requests per client
//! - **Client identification**: User ID (if authenticated) or IP address
//! - **Implementation**: Custom middleware with in-memory tracking
//! 
//! ### Authentication & Authorization
//! - **JWT Tokens**: HS256 algorithm with configurable secret
//! - **Token Expiration**: 24 hours (configurable)
//! - **Password Security**: bcrypt with default cost factor
//! - **Role-based Access**: Configurable roles for different operations
//! 
//! ### Security Headers
//! - **X-Content-Type-Options**: nosniff
//! - **X-Frame-Options**: DENY
//! - **X-XSS-Protection**: 1; mode=block
//! - **Referrer-Policy**: strict-origin-when-cross-origin
//! - **Content-Security-Policy**: Restrictive policy for XSS protection
//! - **Strict-Transport-Security**: HSTS with preload
//! - **Permissions-Policy**: Restrictive feature policy
//! - **Cache-Control**: No-cache headers for sensitive endpoints
//! 
//! ### Request Validation
//! - **User Agent Filtering**: Blocks known bot patterns
//! - **Origin Validation**: CORS protection with allowed origins
//! - **Request Size Limits**: 10MB maximum request size
//! - **Input Validation**: Comprehensive validation for all endpoints
//! 
//! ## GraphQL Integration
//! 
//! All services are also exposed through GraphQL:
//! - **Queries**: Read operations for all services
//! - **Mutations**: Write operations with proper authentication
//! - **Subscriptions**: Real-time updates for social interactions, forum activity, and governance
//! 
//! ### GraphQL Security
//! - **Query Complexity**: Limited to prevent DoS attacks
//! - **Authentication Context**: JWT tokens validated for protected operations
//! - **Rate Limiting**: Applied at the HTTP layer before GraphQL processing
//! 
//! ## Configuration
//! 
//! ### Environment Variables
//! - `CPC_JWT_SECRET`: JWT signing secret (minimum 32 characters)
//! - `CPC_ENCRYPTION_KEY`: 64-character hex string for data encryption
//! - `CPC_BACKEND_PORT`: Server port (default: 8080)
//! - `CPC_ENV`: Environment (dev/test/prod)
//! 
//! ### Security Configuration
//! ```rust
//! SecurityConfig {
//!     max_requests_per_minute: 60,
//!     max_requests_per_hour: 1000,
//!     enable_csrf_protection: true,
//!     enable_content_security_policy: true,
//!     enable_request_logging: true,
//!     max_request_size: 10 * 1024 * 1024, // 10MB
//!     blocked_user_agents: vec!["bot", "crawler", "spider"],
//!     allowed_origins: vec!["http://localhost:3000", "https://localhost:3000"],
//! }
//! ```
//! 
//! ## API Endpoints
//! 
//! ### Authentication
//! - `POST /api/auth/register` - User registration
//! - `POST /api/auth/login` - User login
//! - `POST /api/auth/refresh` - Token refresh (requires auth)
//! 
//! ### Social
//! - `POST /api/social/posts` - Create post
//! - `GET /api/social/posts/:id` - Get post
//! - `PUT /api/social/posts/:id` - Update post
//! - `DELETE /api/social/posts/:id` - Delete post
//! - `POST /api/social/posts/:id/comments` - Create comment
//! - `GET /api/social/posts/:id/comments` - Get comments
//! - `GET /api/social/feed` - Get user feed
//! - `POST /api/social/follow` - Follow user
//! - `DELETE /api/social/unfollow/:user_id` - Unfollow user
//! 
//! ### Forum
//! - `POST /api/forum/forums` - Create forum (requires role)
//! - `GET /api/forum/forums` - List forums
//! - `GET /api/forum/forums/:id` - Get forum
//! - `POST /api/forum/forums/:id/threads` - Create thread
//! - `GET /api/forum/forums/:id/threads` - List threads
//! - `GET /api/forum/threads/:id` - Get thread
//! - `POST /api/forum/threads/:id/replies` - Create reply
//! - `GET /api/forum/threads/:id/replies` - List replies
//! - `POST /api/forum/posts/:id/vote` - Vote on post
//! - `PUT /api/forum/threads/:id/pin` - Pin thread (requires role)
//! - `PUT /api/forum/threads/:id/lock` - Lock thread (requires role)
//! - `DELETE /api/forum/posts/:id` - Delete post
//! 
//! ### Governance
//! - `POST /api/governance/proposals` - Create proposal (requires role)
//! - `GET /api/governance/proposals` - List active proposals
//! - `GET /api/governance/proposals/:id` - Get proposal
//! - `PUT /api/governance/proposals/:id` - Update proposal
//! - `POST /api/governance/proposals/:id/vote` - Vote on proposal
//! - `GET /api/governance/proposals/:id/results` - Get results
//! - `PUT /api/governance/proposals/:id/close` - Close proposal (requires role)
//! - `GET /api/governance/votes` - Get user votes
//! 
//! ## Error Handling
//! 
//! All endpoints return consistent error responses:
//! ```json
//! {
//!   "success": false,
//!   "data": null,
//!   "error": "Error message"
//! }
//! ```
//! 
//! Common HTTP status codes:
//! - `200 OK`: Success
//! - `400 Bad Request`: Invalid input
//! - `401 Unauthorized`: Authentication required
//! - `403 Forbidden`: Insufficient permissions
//! - `404 Not Found`: Resource not found
//! - `409 Conflict`: Resource already exists
//! - `429 Too Many Requests`: Rate limit exceeded
//! - `500 Internal Server Error`: Server error
//! 
//! ## Monitoring and Logging
//! 
//! ### Request Logging
//! - All requests logged with method, URI, client ID
//! - Security events logged (blocked requests, rate limits)
//! - Error conditions logged with context
//! 
//! ### Metrics
//! - Request count and timing
//! - Authentication success/failure rates
//! - Rate limiting statistics
//! - Error rates by endpoint
//! 
//! ### Health Checks
//! - `GET /health` - Basic health check endpoint
//! - Database connectivity checks
//! - Service dependency checks
//! 
//! ## Development and Testing
//! 
//! ### Local Development
//! 1. Set required environment variables
//! 2. Run database migrations
//! 3. Start the server: `cargo run`
//! 4. Access GraphQL playground at `/graphql`
//! 
//! ### Testing
//! - Unit tests for individual services
//! - Integration tests for API endpoints
//! - Security tests for authentication and authorization
//! - Load tests for rate limiting and performance
//! 
//! ## Future Enhancements
//! 
//! ### Planned Security Improvements
//! - OAuth2/OpenID Connect integration
//! - Multi-factor authentication
//! - Advanced fraud detection
//! - Audit logging
//! - API key management
//! 
//! ### Performance Optimizations
//! - Redis-based rate limiting
//! - Database query optimization
//! - Response caching
//! - Connection pooling improvements
//! 
//! ### Monitoring Enhancements
//! - Prometheus metrics
//! - Distributed tracing
//! - Real-time alerting
//! - Performance dashboards
//! API client modules
//!
//! This module provides API client functionality with support for
//! GraphQL, gRPC-web, request batching, and rate limiting.

pub mod api_client;
pub mod grpc;
pub mod queue;
pub mod rate_limit;

// Re-export the main ApiClient
pub use api_client::ApiClient;
pub use api_client::ApiResponse;

// Re-export key types from submodules
pub use grpc::{GrpcClient, GrpcConfig};
pub use queue::{BatchQueue, BatchRequest, RequestBatch};
pub use rate_limit::{RateLimiter, RateLimitConfig, RateLimitResult};
//! Domain entities and business logic for the API & Integration Hub module

pub mod api_endpoint;
pub mod adapter_config;
pub mod rate_limit;

// Re-export key types
pub use api_endpoint::ApiEndpoint;
pub use adapter_config::AdapterConfig;
pub use rate_limit::RateLimitRule;
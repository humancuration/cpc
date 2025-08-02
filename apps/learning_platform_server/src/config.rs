use config::{Config, Environment};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: SocketAddr,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("APP"))
            .set_override("database_url", "postgresql://localhost/learning_platform")?
            .set_override("server_addr", "127.0.0.1:50051")?
            .set_override("jwt_secret", "secret")?
            .build()?
            .try_deserialize()
    }
}
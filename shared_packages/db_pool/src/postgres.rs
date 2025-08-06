//! PostgreSQL connection manager for bb8

use crate::config::DatabaseConfig;
use bb8::ManageConnection;
use tokio_postgres::{Client, Config, NoTls};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// PostgreSQL connection manager for bb8
pub struct PostgresConnectionManager {
    config: Config,
}

impl PostgresConnectionManager {
    /// Create a new PostgreSQL connection manager
    pub fn new(db_config: &DatabaseConfig) -> Self {
        let mut config = Config::new();
        config
            .host(&db_config.host)
            .port(db_config.port)
            .dbname(&db_config.database)
            .user(&db_config.username)
            .password(&db_config.password);
        
        if let Some(ssl_mode) = &db_config.ssl_mode {
            config.ssl_mode(match ssl_mode.as_str() {
                "disable" => tokio_postgres::SslMode::Disable,
                "require" => tokio_postgres::SslMode::Require,
                _ => tokio_postgres::SslMode::Prefer,
            });
        }
        
        Self { config }
    }
}

impl ManageConnection for PostgresConnectionManager {
    type Connection = Client;
    type Error = tokio_postgres::Error;

    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Self::Connection, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            let (client, connection) = self.config.connect(NoTls).await?;
            
            // Spawn the connection task to drive the connection
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("PostgreSQL connection error: {}", e);
                }
            });
            
            Ok(client)
        })
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + '_>> {
        Box::pin(async move {
            // Simple query to check if connection is still valid
            conn.simple_query("").await.map(|_| ())
        })
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        // For tokio-postgres, we rely on the connection task to detect broken connections
        false
    }
}
//! SQLite connection manager for bb8

use bb8::ManageConnection;
use rusqlite::{Connection, OpenFlags};
use std::future::Future;
use std::pin::Pin;

/// SQLite connection manager for bb8
pub struct SqliteConnectionManager {
    path: String,
}

impl SqliteConnectionManager {
    /// Create a new SQLite connection manager
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Self::Connection, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            // Open the database connection
            let conn = Connection::open_with_flags(
                &self.path,
                OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
            )?;
            
            // Enable WAL mode for better concurrency
            conn.execute_batch("PRAGMA journal_mode = WAL;")?;
            
            Ok(conn)
        })
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + '_>> {
        Box::pin(async move {
            // Simple query to check if connection is still valid
            conn.execute_batch("").map(|_| ())
        })
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        // Check if the connection is still valid
        conn.execute_batch("").is_err()
    }
}
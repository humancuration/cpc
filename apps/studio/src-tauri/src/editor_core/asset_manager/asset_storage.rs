//! Content-addressable storage for assets with SQLite metadata

use bevy::prelude::*;
use rusqlite::{Connection, params, OptionalExtension};
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use valkey::ValkeyClient;

pub struct AssetStorage {
    db: Mutex<Connection>,
    base_path: PathBuf,
    asset_cache: Mutex<HashMap<String, HandleUntyped>>,
    valkey_client: ValkeyClient,
}

impl AssetStorage {
    pub fn new() -> Self {
        // Create storage directory if needed
        let base_path = Path::new("assets").to_path_buf();
        std::fs::create_dir_all(&base_path).expect("Failed to create assets directory");
        
        // Initialize SQLite database
        let db_path = base_path.join("asset_metadata.db");
        let conn = Connection::open(&db_path).expect("Failed to open asset database");
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS assets (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL,
                format TEXT NOT NULL,
                size INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        ).expect("Failed to create assets table");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS asset_versions (
                asset_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                hash TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (asset_id, version)
            )",
            [],
        ).expect("Failed to create asset_versions table");
        
        Self {
            db: Mutex::new(conn),
            base_path,
            asset_cache: Mutex::new(HashMap::new()),
            valkey_client: ValkeyClient::new("redis://127.0.0.1:6379").expect("Failed to connect to Valkey"),
        }
    }

    pub fn setup_bevy(&self, app: &mut App) {
        // Register asset types and setup hot reloading
        app.add_asset::<crate::asset_manager::bevy_asset_bridge::CustomAsset>()
            .add_system(Self::hot_reload_assets);
    }

    pub fn store_asset(&self, data: &[u8], format: &str) -> String {
        // Calculate content hash
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hex::encode(hasher.finalize());
        
        // Create storage path
        let storage_path = self.base_path.join(&hash);
        
        // Write file if it doesn't exist
        if !storage_path.exists() {
            std::fs::write(&storage_path, data).expect("Failed to write asset");
        }
        
        // Insert metadata into database
        let conn = self.db.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO assets (id, path, format, size, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                &hash,
                storage_path.to_str().unwrap(),
                format,
                data.len() as i64,
                chrono::Utc::now().timestamp(),
            ],
        ).expect("Failed to insert asset metadata");
        
        // Cache asset data in Valkey with 1 hour expiration
        self.valkey_client.set_ex(&hash, data, 3600)
            .expect("Failed to cache asset in Valkey");
        
        hash
    }

    pub fn get_asset_path(&self, asset_id: &str) -> Option<PathBuf> {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT path FROM assets WHERE id = ?1")
            .expect("Failed to prepare statement");
        
        stmt.query_row(params![asset_id], |row| {
            Ok(PathBuf::from(row.get::<_, String>(0)?)
        }).ok()
    }
    
    /// Get asset data with Valkey caching
    pub fn get_asset_data(&self, asset_id: &str) -> Option<Vec<u8>> {
        // Try to get from Valkey cache
        if let Ok(Some(data)) = self.valkey_client.get::<Vec<u8>>(asset_id) {
            return Some(data);
        }
        
        // Fall back to disk if not in cache
        if let Some(path) = self.get_asset_path(asset_id) {
            if let Ok(data) = std::fs::read(path) {
                // Cache in Valkey for future requests
                self.valkey_client.set_ex(asset_id, &data, 3600)
                    .expect("Failed to cache asset in Valkey");
                return Some(data);
            }
        }
        
        None
    }
    
    /// Get all asset versions that haven't been synchronized
    pub fn get_pending_updates(&self) -> Vec<(String, u64, Vec<u8>)> {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT av.asset_id, av.version
             FROM asset_versions av
             WHERE av.synced = 0"
        ).expect("Failed to prepare statement");
        
        let rows = stmt.query_map([], |row| {
            let asset_id: String = row.get(0)?;
            let version: u64 = row.get(1)?;
            Ok((asset_id, version))
        }).expect("Failed to query pending updates");
        
        let mut updates = Vec::new();
        for row in rows {
            let (asset_id, version) = row.unwrap();
            if let Some(data) = self.get_asset_data(&asset_id) {
                updates.push((asset_id, version, data));
            } else {
                eprintln!("Failed to read asset {}", asset_id);
            }
        }
        updates
    }

    /// Mark an asset version as synchronized
    pub fn mark_as_synced(&self, asset_id: &str, version: u64) {
        let conn = self.db.lock().unwrap();
        conn.execute(
            "UPDATE asset_versions SET synced = 1 WHERE asset_id = ?1 AND version = ?2",
            params![asset_id, version],
        ).expect("Failed to mark asset version as synced");
    }
    
    /// Get version history for an asset
    pub fn get_version_history(&self, asset_id: &str) -> Vec<crate::assets::AssetVersionInfo> {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT version, created_at FROM asset_versions
             WHERE asset_id = ?1 ORDER BY version DESC"
        ).expect("Failed to prepare statement");
        
        let rows = stmt.query_map(params![asset_id], |row| {
            Ok(crate::assets::AssetVersionInfo {
                version: row.get(0)?,
                timestamp: row.get(1)?,
                author: None, // Placeholder for now
            })
        }).expect("Failed to query version history");
        
        rows.map(|r| r.unwrap()).collect()
    }
    
    /// Get asset data for a specific version
    pub fn get_asset_data_for_version(&self, asset_id: &str, version: u64) -> Option<Vec<u8>> {
        let conn = self.db.lock().unwrap();
        let hash: Option<String> = conn.query_row(
            "SELECT hash FROM asset_versions WHERE asset_id = ?1 AND version = ?2",
            params![asset_id, version],
            |row| row.get(0)
        ).ok()?;
        
        hash.and_then(|h| self.get_asset_data(&h))
    }
    
    /// Restore a previous version of an asset
    pub fn restore_asset_version(&self, asset_id: &str, version: u64) -> Result<(), String> {
        // Get the content hash for the version we want to restore
        let hash = self.get_asset_data_for_version(asset_id, version)
            .ok_or_else(|| "Version not found".to_string())?;
        
        // Store the asset again to create a new version
        self.store_asset(&hash, "restored")
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
    
    /// Get diff between two versions (stub for now)
    pub fn get_asset_version_diff(&self, _asset_id: &str, _version_a: u64, _version_b: u64) -> Result<crate::assets::DiffResult, String> {
        // TODO: Implement actual diff logic
        Ok(crate::assets::DiffResult::Text("Diff not implemented".to_string()))
    }

    fn hot_reload_assets(
        asset_server: Res<AssetServer>,
        storage: Res<Arc<AssetStorage>>,
    ) {
        // Check for modified assets and trigger reload
        // Implementation would monitor file changes and reload assets
    }
}
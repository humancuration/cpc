//! Asset synchronization with vector clocks and conflict resolution

use bevy::prelude::*;
use cpc_core::p2p::{NetworkHandler, ReconciliationEngine};
use crate::asset_manager::asset_storage::AssetStorage;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use crate::assets::LockInfo;

pub struct AssetSync {
    network_handler: Arc<NetworkHandler>,
    storage: Arc<AssetStorage>,
    reconciliation: ReconciliationEngine,
    vector_clocks: HashMap<String, u64>, // asset_id to version
    locks: HashMap<String, LockInfo>, // asset_id to lock info
}

impl AssetSync {
    pub fn new(network_handler: Arc<NetworkHandler>, storage: Arc<AssetStorage>) -> Self {
        let reconciliation = ReconciliationEngine::new();
        Self {
            network_handler,
            storage,
            reconciliation,
            vector_clocks: HashMap::new(),
            locks: HashMap::new(),
        }
    }

    pub fn setup_bevy(&self, app: &mut App) {
        app.add_system(sync_assets);
    }

    pub fn handle_asset_update(&self, asset_id: &str, version: u64, data: Vec<u8>) {
        // Handle incoming asset updates from network
        self.reconciliation.reconcile(asset_id, version, data);
    }
    
    /// Acquire lock for an asset
    pub fn acquire_lock(&mut self, asset_id: &str, user_id: Uuid, user_name: String) -> Result<(), String> {
        if let Some(lock_info) = self.locks.get(asset_id) {
            if lock_info.user_id == user_id {
                return Ok(()); // Already owned by user
            }
            return Err(format!("Asset is locked by {}", lock_info.user_name));
        }
        
        self.locks.insert(asset_id.to_string(), LockInfo { user_id, user_name });
        
        // Broadcast lock event
        self.network_handler.broadcast_lock_event(
            asset_id,
            user_id,
            user_name.clone(),
            true
        );
        
        Ok(())
    }
    
    /// Release lock for an asset
    pub fn release_lock(&mut self, asset_id: &str, user_id: Uuid) -> Result<(), String> {
        if let Some(lock_info) = self.locks.get(asset_id) {
            if lock_info.user_id != user_id {
                return Err("Only lock owner can release lock".to_string());
            }
            
            self.locks.remove(asset_id);
            
            // Broadcast unlock event
            self.network_handler.broadcast_lock_event(
                asset_id,
                user_id,
                lock_info.user_name.clone(),
                false
            );
            
            return Ok(());
        }
        Err("Asset not locked".to_string())
    }
    
    /// Handle incoming lock event
    pub fn handle_lock_event(&mut self, asset_id: &str, user_id: Uuid, user_name: String, locked: bool) {
        if locked {
            self.locks.insert(asset_id.to_string(), LockInfo { user_id, user_name });
        } else {
            self.locks.remove(asset_id);
        }
    }
    
    /// Get lock info for an asset
    pub fn get_lock_info(&self, asset_id: &str) -> Option<&LockInfo> {
        self.locks.get(asset_id)
    }
    
    /// Get current vector clock version for an asset
    pub fn get_version(&self, asset_id: &str) -> u64 {
        *self.vector_clocks.get(asset_id).unwrap_or(&0)
    }
    
    /// Increment vector clock version for an asset
    pub fn increment_version(&mut self, asset_id: &str) -> u64 {
        let version = self.vector_clocks.entry(asset_id.to_string()).or_insert(0);
        *version += 1;
        *version
    }
}

fn sync_assets(
    mut sync: ResMut<AssetSync>,
    storage: Res<Arc<AssetStorage>>,
) {
    // Propagate local asset changes to network
    for (asset_id, version, data) in storage.get_pending_updates() {
        // Only sync if we have the lock or asset isn't locked
        if sync.locks.get(&asset_id).is_none() {
            let new_version = sync.increment_version(&asset_id);
            sync.network_handler.broadcast_asset_update(&asset_id, new_version, data);
            storage.mark_as_synced(&asset_id, version);
        }
    }
    
    // Process incoming updates
    while let Some((asset_id, version, data)) = sync.network_handler.get_incoming_asset_update() {
        // Apply conflict resolution based on vector clocks
        let current_version = sync.get_version(&asset_id);
        if version > current_version {
            sync.handle_asset_update(&asset_id, version, data);
            sync.vector_clocks.insert(asset_id.to_string(), version);
        }
    }
    
    // Process incoming lock events
    while let Some((asset_id, user_id, user_name, locked)) = sync.network_handler.get_incoming_lock_event() {
        sync.handle_lock_event(&asset_id, user_id, user_name, locked);
    }
}
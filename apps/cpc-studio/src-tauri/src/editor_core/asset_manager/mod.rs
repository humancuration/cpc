//! Asset management system for CPC Studio editor

pub mod asset_processor;
pub mod asset_sync;
pub mod bevy_asset_bridge;
pub mod asset_storage;

use bevy::prelude::*;
use cpc_core::p2p::NetworkHandler;

/// Main asset manager resource
pub struct AssetManager {
    processor: asset_processor::AssetProcessor,
    sync: asset_sync::AssetSync,
    storage: asset_storage::AssetStorage,
}

impl AssetManager {
    pub fn new(network_handler: Arc<NetworkHandler>) -> Self {
        let storage = asset_storage::AssetStorage::new();
        let sync = asset_sync::AssetSync::new(network_handler.clone(), storage.clone());
        let processor = asset_processor::AssetProcessor::new(storage.clone());
        
        Self {
            processor,
            sync,
            storage,
        }
    }

    pub fn setup_bevy(&self, app: &mut App) {
        self.storage.setup_bevy(app);
        self.sync.setup_bevy(app);
        self.processor.setup_bevy(app);
    }
}
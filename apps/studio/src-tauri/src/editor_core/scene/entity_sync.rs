use bevy::prelude::*;
use cpc_core::p2p::{NetworkHandler, ReconciliationEngine};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use crate::assets::LockInfo;
use crate::editor_core::scene::hierarchy::SceneHierarchy;

pub struct EntitySync {
    network_handler: Arc<NetworkHandler>,
    hierarchy: Arc<SceneHierarchy>,
    reconciliation: ReconciliationEngine,
    vector_clocks: HashMap<Uuid, u64>, // entity_id to version
    locks: HashMap<Uuid, LockInfo>,    // entity_id to lock info
}

impl EntitySync {
    pub fn new(network_handler: Arc<NetworkHandler>, hierarchy: Arc<SceneHierarchy>) -> Self {
        let reconciliation = ReconciliationEngine::new();
        Self {
            network_handler,
            hierarchy,
            reconciliation,
            vector_clocks: HashMap::new(),
            locks: HashMap::new(),
        }
    }

    pub fn setup_bevy(&self, app: &mut App) {
        app.add_system(sync_entities);
    }

    pub fn acquire_lock(&mut self, entity_id: Uuid, user_id: Uuid, user_name: String) -> Result<(), String> {
        if let Some(lock_info) = self.locks.get(&entity_id) {
            if lock_info.user_id == user_id {
                return Ok(()); // Already owned by user
            }
            return Err(format!("Entity is locked by {}", lock_info.user_name));
        }
        
        self.locks.insert(entity_id, LockInfo { user_id, user_name });
        
        // Broadcast lock event
        self.network_handler.broadcast_entity_lock_event(
            entity_id,
            user_id,
            user_name.clone(),
            true
        );
        
        Ok(())
    }

    pub fn release_lock(&mut self, entity_id: Uuid, user_id: Uuid) -> Result<(), String> {
        if let Some(lock_info) = self.locks.get(&entity_id) {
            if lock_info.user_id != user_id {
                return Err("Only lock owner can release lock".to_string());
            }
            
            self.locks.remove(&entity_id);
            
            // Broadcast unlock event
            self.network_handler.broadcast_entity_lock_event(
                entity_id,
                user_id,
                lock_info.user_name.clone(),
                false
            );
            
            return Ok(());
        }
        Err("Entity not locked".to_string())
    }

    pub fn handle_lock_event(&mut self, entity_id: Uuid, user_id: Uuid, user_name: String, locked: bool) {
        if locked {
            self.locks.insert(entity_id, LockInfo { user_id, user_name });
        } else {
            self.locks.remove(&entity_id);
        }
    }

    pub fn get_lock_info(&self, entity_id: Uuid) -> Option<&LockInfo> {
        self.locks.get(&entity_id)
    }
}

fn sync_entities(
    mut sync: ResMut<EntitySync>,
    hierarchy: Res<Arc<SceneHierarchy>>,
) {
    // Process incoming lock events
    while let Some((entity_id, user_id, user_name, locked)) = 
        sync.network_handler.get_incoming_entity_lock_event() 
    {
        sync.handle_lock_event(entity_id, user_id, user_name, locked);
    }
    
    // TODO: Implement entity change synchronization
}
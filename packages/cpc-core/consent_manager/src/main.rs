//! Example application demonstrating the consent manager
//!
//! This example shows how to use the consent manager in a complete application,
//! including integration with Bevy ECS for real-time updates.

use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain, ConsentProfile},
        audit::{AuditEvent, Actor, ConsentAction},
        errors::ConsentError,
    },
    application::service::{ConsentService, ConsentStorage},
    infrastructure::events::bevy::{ConsentEventChannel, ConsentEventPlugin, handle_consent_updates},
};
use std::collections::HashMap;
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use std::time::Duration;

/// In-memory storage implementation for demonstration purposes
struct InMemoryStorage {
    profiles: std::sync::Mutex<HashMap<String, ConsentProfile>>,
    audit_events: std::sync::Mutex<HashMap<String, Vec<AuditEvent>>>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            profiles: std::sync::Mutex::new(HashMap::new()),
            audit_events: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl ConsentStorage for InMemoryStorage {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let profiles = self.profiles.lock().unwrap();
        Ok(profiles.get(&key).cloned())
    }

    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", profile.user_id, profile.domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.insert(key, profile.clone());
        Ok(())
    }

    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.remove(&key);
        Ok(())
    }

    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        let audit_events = self.audit_events.lock().unwrap();
        Ok(audit_events.get(user_id).cloned().unwrap_or_default())
    }

    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
        let mut audit_events = self.audit_events.lock().unwrap();
        audit_events.entry(event.user_id.clone()).or_insert_with(Vec::new).push(event.clone());
        Ok(())
    }
}

/// Component that represents a UI element that needs to be updated when consent changes
#[derive(Component)]
struct ConsentIndicator {
    user_id: String,
    domain: Domain,
    level: DataSharingLevel,
}

fn main() {
    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();
    
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Create a Bevy app with the consent event plugin
    let mut app = App::new();
    app
        .add_plugins((
            ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)),
            ConsentEventPlugin,
        ))
        .add_systems(Update, handle_consent_updates)
        .add_systems(Update, update_consent_example);
    
    // Insert the consent service as a resource so systems can access it
    app.insert_resource(consent_service);
    
    // Run the app
    app.run();
}

/// Example system that updates consent and shows how the event system works
fn update_consent_example(
    consent_service: Res<ConsentService>,
) {
    // In a real application, this would be triggered by user actions
    // For this example, we'll just update consent for a test user
    
    let user_id = "test_user_123";
    let domain = Domain::FinancialData;
    let new_level = DataSharingLevel::Standard;
    let actor = Actor::User(user_id.to_string());
    
    // Spawn a task to update consent asynchronously
    tokio::spawn(async move {
        match consent_service.update_consent_level(user_id, domain, new_level, actor).await {
            Ok(()) => {
                tracing::info!("Successfully updated consent for user {}", user_id);
            }
            Err(e) => {
                tracing::error!("Failed to update consent for user {}: {:?}", user_id, e);
            }
        }
    });
}
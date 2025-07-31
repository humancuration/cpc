//! Pub/sub implementation
//! 
//! This module provides a pub/sub implementation for event distribution.

use tokio::sync::broadcast;
use crate::domain::event::DomainEvent;
use crate::domain::subscription::EventFilter;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Pub/sub system for event distribution
pub struct PubSubSystem {
    /// Channels for different topics
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<DomainEvent>>>>,
}

impl PubSubSystem {
    /// Create a new pub/sub system
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Subscribe to a topic
    pub async fn subscribe(&self, topic: &str) -> broadcast::Receiver<DomainEvent> {
        let mut channels = self.channels.write().await;
        
        // Get or create the channel for this topic
        let sender = channels.entry(topic.to_string()).or_insert_with(|| {
            broadcast::channel(1000).0
        });
        
        sender.subscribe()
    }
    
    /// Publish an event to a topic
    pub async fn publish(&self, topic: &str, event: DomainEvent) -> Result<(), broadcast::error::SendError<DomainEvent>> {
        let channels = self.channels.read().await;
        
        if let Some(sender) = channels.get(topic) {
            sender.send(event)
        } else {
            // No subscribers, that's fine
            Ok(())
        }
    }
    
    /// Subscribe to events with a filter
    pub async fn subscribe_with_filter(&self, filter: EventFilter) -> broadcast::Receiver<DomainEvent> {
        // For now, we'll use a simple topic based on domain
        let topic = filter.domain.clone().unwrap_or("all".to_string());
        self.subscribe(&topic).await
    }
    
    /// Publish an event based on its properties
    pub async fn publish_event(&self, event: DomainEvent) -> Result<(), broadcast::error::SendError<DomainEvent>> {
        // Publish to domain-specific topic
        self.publish(&event.domain, event.clone()).await?;
        
        // Also publish to "all" topic
        self.publish("all", event).await
    }
}

impl Default for PubSubSystem {
    fn default() -> Self {
        Self::new()
    }
}
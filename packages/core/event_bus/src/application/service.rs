//! EventBus orchestrator
//! 
//! This module contains the EventBus which is the primary entry point for event operations.

use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use crate::domain::{
    event::DomainEvent,
    subscription::{Subscription, EventFilter},
    EventError,
};
use storage_abstraction::{DataStore, StorageError};
use tracing::{trace, debug};

/// Event storage trait
#[async_trait::async_trait]
pub trait EventStorage: Send + Sync {
    /// Store an event
    async fn store_event(&self, event: &DomainEvent) -> Result<(), EventError>;
    
    /// Retrieve events based on a query
    async fn get_events(&self, query: EventQuery) -> Result<Vec<DomainEvent>, EventError>;
}

/// Query for retrieving events
#[derive(Debug, Clone)]
pub struct EventQuery {
    /// Filter by domain (None means all domains)
    pub domain: Option<String>,
    
    /// Filter by event types (empty means all event types)
    pub event_types: Vec<String>,
    
    /// Limit the number of results
    pub limit: usize,
    
    /// Offset for pagination
    pub offset: usize,
}

/// Primary entry point for applications
pub struct EventBus {
    /// Event storage backend
    storage: Arc<dyn EventStorage>,
    
    /// Event router for distributing events
    router: Arc<EventRouter>,
    
    /// Channel for internal event distribution
    event_tx: mpsc::UnboundedSender<DomainEvent>,
}

impl EventBus {
    /// Create new event bus instance
    pub fn new(storage: Arc<dyn EventStorage>) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let router = Arc::new(EventRouter::new());
        
        // Start the event distribution task
        let router_clone = router.clone();
        tokio::spawn(async move {
            Self::event_distribution_task(event_rx, router_clone).await;
        });
        
        Self {
            storage,
            router,
            event_tx,
        }
    }
    
    /// Publish event to all subscribers
    pub async fn publish(&self, event: DomainEvent) -> Result<(), EventError> {
        trace!("Publishing event: {}", event);
        
        // Store the event
        self.storage.store_event(&event).await?;
        
        // Send to router for distribution
        if let Err(_) = self.event_tx.send(event.clone()) {
            return Err(EventError::SubscriptionError("Failed to send event to router".to_string()));
        }
        
        debug!("Event published successfully: {}", event.event_id);
        Ok(())
    }
    
    /// Subscribe to specific event types
    pub async fn subscribe(&self, filters: EventFilter) -> Subscription {
        self.router.subscribe(filters).await
    }
    
    /// Get historical events (for sync/recovery)
    pub async fn get_history(&self, query: EventQuery) -> Result<Vec<DomainEvent>, EventError> {
        self.storage.get_events(query).await
    }
    
    /// Event distribution task that runs in the background
    async fn event_distribution_task(
        mut event_rx: mpsc::UnboundedReceiver<DomainEvent>,
        router: Arc<EventRouter>,
    ) {
        while let Some(event) = event_rx.recv().await {
            router.distribute_event(event).await;
        }
    }
}

/// Event router that handles event distribution to subscribers
pub struct EventRouter {
    /// Active subscriptions
    subscriptions: RwLock<Vec<SubscriptionEntry>>,
}

/// Entry for tracking subscriptions
struct SubscriptionEntry {
    /// Sender for the subscription
    sender: mpsc::UnboundedSender<DomainEvent>,
    
    /// Filter for the subscription
    filter: EventFilter,
}

impl EventRouter {
    /// Create a new event router
    pub fn new() -> Self {
        Self {
            subscriptions: RwLock::new(Vec::new()),
        }
    }
    
    /// Subscribe to events
    pub async fn subscribe(&self, filter: EventFilter) -> Subscription {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let entry = SubscriptionEntry {
            sender,
            filter: filter.clone(),
        };
        
        // Add to subscriptions
        {
            let mut subscriptions = self.subscriptions.write().await;
            subscriptions.push(entry);
        }
        
        Subscription::new(receiver, filter)
    }
    
    /// Distribute an event to all matching subscribers
    pub async fn distribute_event(&self, event: DomainEvent) {
        let subscriptions = self.subscriptions.read().await;
        
        // Send to all matching subscribers
        for entry in subscriptions.iter() {
            if entry.filter.matches(&event) {
                // Try to send, but don't fail if the receiver is closed
                let _ = entry.sender.send(event.clone());
            }
        }
    }
}

// Implementation of EventStorage for storage_abstraction::DataStore
#[async_trait::async_trait]
impl<T> EventStorage for T
where
    T: DataStore,
{
    async fn store_event(&self, event: &DomainEvent) -> Result<(), EventError> {
        let key = format!("event:{}:{}", event.domain, event.event_id);
        let value = serde_json::to_vec(event)
            .map_err(|e| EventError::SerializationError(e.to_string()))?;
        
        self.set(&key, value)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }
    
    async fn get_events(&self, query: EventQuery) -> Result<Vec<DomainEvent>, EventError> {
        // This is a simplified implementation
        // In a real implementation, we would have a more sophisticated query mechanism
        todo!("Implement event querying")
    }
}
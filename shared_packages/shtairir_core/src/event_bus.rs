//! Redis-based cross-app communication bus for Shtairir Core

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{
    ShtairirResult, ShtairirError,
    Event, EventSystem, EventHandler, EventFilter,
};

/// Redis-based event bus implementation
pub struct RedisEventBus {
    /// Redis connection pool
    redis_pool: Pool<RedisConnectionManager>,
    /// Event handlers by event type
    handlers: RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>,
    /// Event bus configuration
    config: EventBusConfig,
    /// Event bus ID
    bus_id: Uuid,
}

/// Event bus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusConfig {
    /// Redis connection URL
    pub redis_url: String,
    /// Event prefix for Redis keys
    pub event_prefix: String,
    /// Event stream name
    pub stream_name: String,
    /// Event TTL in seconds
    pub event_ttl_seconds: u64,
    /// Maximum number of events to keep in memory
    pub max_memory_events: usize,
    /// Whether to persist events to Redis
    pub persist_events: bool,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://localhost:6379".to_string(),
            event_prefix: "shtairir:event".to_string(),
            stream_name: "shtairir_events".to_string(),
            event_ttl_seconds: 86400, // 24 hours
            max_memory_events: 10000,
            persist_events: true,
        }
    }
}

impl RedisEventBus {
    /// Create a new Redis event bus
    pub async fn new(config: EventBusConfig) -> ShtairirResult<Self> {
        // Create Redis connection manager
        let manager = RedisConnectionManager::new(config.redis_url.as_str())
            .map_err(|e| ShtairirError::Redis(format!("Failed to create Redis manager: {}", e)))?;
        
        // Create connection pool
        let redis_pool = Pool::builder()
            .build(manager)
            .await
            .map_err(|e| ShtairirError::Redis(format!("Failed to create Redis pool: {}", e)))?;
        
        Ok(Self {
            redis_pool,
            handlers: RwLock::new(HashMap::new()),
            config,
            bus_id: Uuid::new_v4(),
        })
    }
    
    /// Create a new Redis event bus with default configuration
    pub async fn default() -> ShtairirResult<Self> {
        Self::new(EventBusConfig::default()).await
    }
    
    /// Get the event bus ID
    pub fn bus_id(&self) -> &Uuid {
        &self.bus_id
    }
    
    /// Get the event bus configuration
    pub fn config(&self) -> &EventBusConfig {
        &self.config
    }
    
    /// Store an event in Redis
    async fn store_event(&self, event: &Event) -> ShtairirResult<()> {
        if !self.config.persist_events {
            return Ok(());
        }
        
        let mut conn = self.redis_pool.get().await?;
        
        // Serialize event
        let event_json = serde_json::to_string(event)
            .map_err(|e| ShtairirError::Serialization(format!("Failed to serialize event: {}", e)))?;
        
        // Store event in Redis stream
        let _: () = redis::cmd("XADD")
            .arg(&self.config.stream_name)
            .arg("*")
            .arg("event_json")
            .arg(event_json)
            .query(&mut conn)
            .map_err(|e| ShtairirError::Redis(format!("Failed to add event to stream: {}", e)))?;
        
        // Set TTL for the event
        let _: () = redis::cmd("EXPIRE")
            .arg(&self.config.stream_name)
            .arg(self.config.event_ttl_seconds)
            .query(&mut conn)
            .map_err(|e| ShtairirError::Redis(format!("Failed to set stream TTL: {}", e)))?;
        
        Ok(())
    }
    
    /// Load events from Redis
    async fn load_events(&self, filter: &EventFilter) -> ShtairirResult<Vec<Event>> {
        let mut conn = self.redis_pool.get().await?;
        
        // Read events from Redis stream
        let events_json: Vec<String> = redis::cmd("XRANGE")
            .arg(&self.config.stream_name)
            .arg("-")
            .arg("+")
            .query(&mut conn)
            .map_err(|e| ShtairirError::Redis(format!("Failed to read events from stream: {}", e)))?;
        
        let mut events = Vec::new();
        
        for event_json in events_json {
            match serde_json::from_str::<Event>(&event_json) {
                Ok(event) => {
                    if self.event_matches_filter(&event, filter) {
                        events.push(event);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to deserialize event: {}", e);
                }
            }
        }
        
        // Apply limit
        if let Some(limit) = filter.limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    /// Check if an event matches the filter
    fn event_matches_filter(&self, event: &Event, filter: &EventFilter) -> bool {
        // Check event type
        if !filter.event_types.is_empty() && !filter.event_types.contains(&event.event_type) {
            return false;
        }
        
        // Check source
        if !filter.sources.is_empty() && !filter.sources.contains(&event.source) {
            return false;
        }
        
        // Check time range
        if let Some(start_time) = filter.start_time {
            if event.timestamp < start_time {
                return false;
            }
        }
        
        if let Some(end_time) = filter.end_time {
            if event.timestamp > end_time {
                return false;
            }
        }
        
        // Check data filter
        if let Some(data_filter) = &filter.data_filter {
            if let ShtairirValue::Object(event_data) = &event.data {
                for (key, expected_value) in data_filter {
                    if let Some(actual_value) = event_data.get(key) {
                        if actual_value != expected_value {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
        
        true
    }
    
    /// Deliver an event to handlers
    async fn deliver_to_handlers(&self, event: &Event) -> ShtairirResult<()> {
        let handlers = self.handlers.read().await;
        
        if let Some(event_handlers) = handlers.get(&event.event_type) {
            for handler in event_handlers {
                if let Err(e) = handler.handle(event).await {
                    tracing::error!("Event handler failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl EventSystem for RedisEventBus {
    async fn publish(&self, event: Event) -> ShtairirResult<()> {
        // Store event in Redis
        self.store_event(&event).await?;
        
        // Deliver to handlers
        self.deliver_to_handlers(&event).await?;
        
        Ok(())
    }
    
    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> ShtairirResult<()> {
        let mut handlers = self.handlers.write().await;
        
        if !handlers.contains_key(event_type) {
            handlers.insert(event_type.to_string(), Vec::new());
        }
        
        let event_handlers = handlers.get_mut(event_type).unwrap();
        
        // Check if handler is already subscribed
        for existing_handler in event_handlers.iter() {
            if existing_handler.handler_id() == handler.handler_id() {
                return Err(ShtairirError::EventBus(format!(
                    "Handler '{}' is already subscribed to event '{}'",
                    handler.handler_id(),
                    event_type
                )));
            }
        }
        
        event_handlers.push(handler);
        
        Ok(())
    }
    
    async fn unsubscribe(&self, event_type: &str, handler_id: &str) -> ShtairirResult<()> {
        let mut handlers = self.handlers.write().await;
        
        if let Some(event_handlers) = handlers.get_mut(event_type) {
            event_handlers.retain(|handler| handler.handler_id() != handler_id);
            
            // Remove empty event type entries
            if event_handlers.is_empty() {
                handlers.remove(event_type);
            }
        } else {
            return Err(ShtairirError::EventBus(format!(
                "No handlers subscribed to event '{}'", event_type
            )));
        }
        
        Ok(())
    }
    
    async fn get_event_history(&self, filter: EventFilter) -> ShtairirResult<Vec<Event>> {
        self.load_events(&filter).await
    }
}

/// In-memory event bus for testing and development
pub struct InMemoryEventBus {
    /// Event handlers by event type
    handlers: RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>,
    /// Event history
    events: RwLock<Vec<Event>>,
    /// Event bus configuration
    config: EventBusConfig,
    /// Event bus ID
    bus_id: Uuid,
}

impl InMemoryEventBus {
    /// Create a new in-memory event bus
    pub fn new(config: EventBusConfig) -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            events: RwLock::new(Vec::new()),
            config,
            bus_id: Uuid::new_v4(),
        }
    }
    
    /// Create a new in-memory event bus with default configuration
    pub fn default() -> Self {
        Self::new(EventBusConfig::default())
    }
    
    /// Get the event bus ID
    pub fn bus_id(&self) -> &Uuid {
        &self.bus_id
    }
    
    /// Get the event bus configuration
    pub fn config(&self) -> &EventBusConfig {
        &self.config
    }
    
    /// Deliver an event to handlers
    async fn deliver_to_handlers(&self, event: &Event) -> ShtairirResult<()> {
        let handlers = self.handlers.read().await;
        
        if let Some(event_handlers) = handlers.get(&event.event_type) {
            for handler in event_handlers {
                if let Err(e) = handler.handle(event).await {
                    tracing::error!("Event handler failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Add an event to the history
    async fn add_to_history(&self, event: &Event) -> ShtairirResult<()> {
        let mut events = self.events.write().await;
        
        // Add event to history
        events.push(event.clone());
        
        // Enforce maximum memory limit
        if events.len() > self.config.max_memory_events {
            events.remove(0);
        }
        
        Ok(())
    }
    
    /// Get events from history matching the filter
    async fn get_filtered_events(&self, filter: &EventFilter) -> Vec<Event> {
        let events = self.events.read().await;
        
        let mut filtered_events: Vec<Event> = events
            .iter()
            .filter(|event| self.event_matches_filter(event, filter))
            .cloned()
            .collect();
        
        // Apply limit
        if let Some(limit) = filter.limit {
            filtered_events.truncate(limit);
        }
        
        filtered_events
    }
    
    /// Check if an event matches the filter
    fn event_matches_filter(&self, event: &Event, filter: &EventFilter) -> bool {
        // Check event type
        if !filter.event_types.is_empty() && !filter.event_types.contains(&event.event_type) {
            return false;
        }
        
        // Check source
        if !filter.sources.is_empty() && !filter.sources.contains(&event.source) {
            return false;
        }
        
        // Check time range
        if let Some(start_time) = filter.start_time {
            if event.timestamp < start_time {
                return false;
            }
        }
        
        if let Some(end_time) = filter.end_time {
            if event.timestamp > end_time {
                return false;
            }
        }
        
        // Check data filter
        if let Some(data_filter) = &filter.data_filter {
            if let ShtairirValue::Object(event_data) = &event.data {
                for (key, expected_value) in data_filter {
                    if let Some(actual_value) = event_data.get(key) {
                        if actual_value != expected_value {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
        
        true
    }
}

#[async_trait]
impl EventSystem for InMemoryEventBus {
    async fn publish(&self, event: Event) -> ShtairirResult<()> {
        // Add to history
        self.add_to_history(&event).await?;
        
        // Deliver to handlers
        self.deliver_to_handlers(&event).await?;
        
        Ok(())
    }
    
    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> ShtairirResult<()> {
        let mut handlers = self.handlers.write().await;
        
        if !handlers.contains_key(event_type) {
            handlers.insert(event_type.to_string(), Vec::new());
        }
        
        let event_handlers = handlers.get_mut(event_type).unwrap();
        
        // Check if handler is already subscribed
        for existing_handler in event_handlers.iter() {
            if existing_handler.handler_id() == handler.handler_id() {
                return Err(ShtairirError::EventBus(format!(
                    "Handler '{}' is already subscribed to event '{}'",
                    handler.handler_id(),
                    event_type
                )));
            }
        }
        
        event_handlers.push(handler);
        
        Ok(())
    }
    
    async fn unsubscribe(&self, event_type: &str, handler_id: &str) -> ShtairirResult<()> {
        let mut handlers = self.handlers.write().await;
        
        if let Some(event_handlers) = handlers.get_mut(event_type) {
            event_handlers.retain(|handler| handler.handler_id() != handler_id);
            
            // Remove empty event type entries
            if event_handlers.is_empty() {
                handlers.remove(event_type);
            }
        } else {
            return Err(ShtairirError::EventBus(format!(
                "No handlers subscribed to event '{}'", event_type
            )));
        }
        
        Ok(())
    }
    
    async fn get_event_history(&self, filter: EventFilter) -> ShtairirResult<Vec<Event>> {
        Ok(self.get_filtered_events(&filter).await)
    }
}

/// Event bus factory
pub struct EventBusFactory;

impl EventBusFactory {
    /// Create a Redis event bus
    pub async fn create_redis_event_bus(config: EventBusConfig) -> ShtairirResult<Arc<dyn EventSystem>> {
        Ok(Arc::new(RedisEventBus::new(config).await?))
    }
    
    /// Create an in-memory event bus
    pub fn create_in_memory_event_bus(config: EventBusConfig) -> Arc<dyn EventSystem> {
        Arc::new(InMemoryEventBus::new(config))
    }
    
    /// Create an event bus based on configuration
    pub async fn create_event_bus(config: EventBusConfig) -> ShtairirResult<Arc<dyn EventSystem>> {
        if config.redis_url.is_empty() || config.redis_url == "memory" {
            Ok(Self::create_in_memory_event_bus(config))
        } else {
            Self::create_redis_event_bus(config).await
        }
    }
}

/// Example event handler implementation
pub struct ExampleEventHandler {
    handler_id: String,
}

impl ExampleEventHandler {
    /// Create a new example event handler
    pub fn new(id: String) -> Self {
        Self {
            handler_id: id,
        }
    }
}

#[async_trait]
impl EventHandler for ExampleEventHandler {
    async fn handle(&self, event: &Event) -> ShtairirResult<()> {
        println!("Handler {} received event: {} from {}", 
                 self.handler_id, event.event_type, event.source);
        Ok(())
    }
    
    fn handler_id(&self) -> &str {
        &self.handler_id
    }
}
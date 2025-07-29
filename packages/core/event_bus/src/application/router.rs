//! Event routing logic
//! 
//! This module handles the routing of events based on their domain and type.

use crate::domain::{
    event::DomainEvent,
    subscription::EventFilter,
};
use std::collections::HashMap;

/// Event router configuration
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Default routing rules
    pub default_routes: HashMap<String, Vec<String>>,
    
    /// Domain-specific routing rules
    pub domain_routes: HashMap<String, Vec<String>>,
}

impl Default for RouterConfig {
    fn default() -> Self {
        let mut default_routes = HashMap::new();
        default_routes.insert("health".to_string(), vec!["local".to_string(), "cloud".to_string()]);
        default_routes.insert("finance".to_string(), vec!["cloud".to_string()]);
        default_routes.insert("calendar".to_string(), vec!["local".to_string(), "cloud".to_string()]);
        
        Self {
            default_routes,
            domain_routes: HashMap::new(),
        }
    }
}

/// Event router that determines where events should be sent
pub struct EventRouter {
    config: RouterConfig,
}

impl EventRouter {
    /// Create a new event router
    pub fn new(config: RouterConfig) -> Self {
        Self { config }
    }
    
    /// Determine where an event should be routed
    pub fn route_event(&self, event: &DomainEvent) -> Vec<String> {
        // Check for domain-specific routes first
        if let Some(routes) = self.config.domain_routes.get(&event.domain) {
            return routes.clone();
        }
        
        // Fall back to default routes
        if let Some(routes) = self.config.default_routes.get(&event.domain) {
            return routes.clone();
        }
        
        // Default to local only
        vec!["local".to_string()]
    }
    
    /// Check if an event matches a filter
    pub fn event_matches_filter(&self, event: &DomainEvent, filter: &EventFilter) -> bool {
        // Check domain filter
        if let Some(ref domain) = filter.domain {
            if &event.domain != domain {
                return false;
            }
        }
        
        // Check event type filter
        if !filter.event_types.is_empty() {
            if !filter.event_types.contains(&event.event_type) {
                return false;
            }
        }
        
        true
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new(RouterConfig::default())
    }
}
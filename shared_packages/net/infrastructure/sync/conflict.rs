//! Conflict resolution strategies for sync operations
//!
//! This module provides default conflict resolution strategies for handling
//! conflicts between local and remote data during sync operations.

use crate::storage::{ResolutionResult, ResolutionPolicy};
use std::time::{SystemTime, UNIX_EPOCH};

/// Stored preference format (used for conflict resolution)
#[derive(Debug, Clone)]
pub struct StoredPreference {
    pub currency_code: String,
    pub synced: bool,
    pub timestamp: u64,
}

impl StoredPreference {
    /// Create a new StoredPreference
    pub fn new(currency_code: String, synced: bool) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        Self {
            currency_code,
            synced,
            timestamp,
        }
    }
}

/// Default conflict resolution using timestamp-based "last write wins"
pub struct TimestampConflictResolver;

impl TimestampConflictResolver {
    /// Create a new TimestampConflictResolver
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimestampConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl TimestampConflictResolver {
    /// Resolve conflicts between local and remote preferences
    pub fn resolve_conflict(&self, local: &StoredPreference, remote: &StoredPreference) -> ResolutionResult {
        if local.timestamp > remote.timestamp {
            ResolutionResult::UseLocal
        } else if remote.timestamp > local.timestamp {
            ResolutionResult::UseRemote
        } else {
            // Same timestamp, prefer local for user-facing actions
            ResolutionResult::UseLocal
        }
    }
    
    /// Get the resolution policy
    pub fn resolution_policy(&self) -> ResolutionPolicy {
        ResolutionPolicy::LastWriteWins
    }
}

/// Custom conflict resolver that always prefers local data
pub struct LocalConflictResolver;

impl LocalConflictResolver {
    /// Create a new LocalConflictResolver
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalConflictResolver {
    /// Always resolve conflicts by using local data
    pub fn resolve_conflict(&self, _local: &StoredPreference, _remote: &StoredPreference) -> ResolutionResult {
        ResolutionResult::UseLocal
    }
    
    /// Get the resolution policy
    pub fn resolution_policy(&self) -> ResolutionPolicy {
        ResolutionPolicy::Custom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_conflict_resolver_newer_local() {
        let resolver = TimestampConflictResolver::new();
        
        let local = StoredPreference::new("USD".to_string(), true);
        let remote = StoredPreference {
            currency_code: "EUR".to_string(),
            synced: true,
            timestamp: local.timestamp - 100, // Older timestamp
        };
        
        let result = resolver.resolve_conflict(&local, &remote);
        match result {
            ResolutionResult::UseLocal => (),
            _ => panic!("Expected UseLocal"),
        }
    }
    
    #[test]
    fn test_timestamp_conflict_resolver_newer_remote() {
        let resolver = TimestampConflictResolver::new();
        
        let local = StoredPreference::new("USD".to_string(), true);
        let remote = StoredPreference {
            currency_code: "EUR".to_string(),
            synced: true,
            timestamp: local.timestamp + 100, // Newer timestamp
        };
        
        let result = resolver.resolve_conflict(&local, &remote);
        match result {
            ResolutionResult::UseRemote => (),
            _ => panic!("Expected UseRemote"),
        }
    }
    
    #[test]
    fn test_local_conflict_resolver() {
        let resolver = LocalConflictResolver::new();
        
        let local = StoredPreference::new("USD".to_string(), true);
        let remote = StoredPreference::new("EUR".to_string(), true);
        
        let result = resolver.resolve_conflict(&local, &remote);
        match result {
            ResolutionResult::UseLocal => (),
            _ => panic!("Expected UseLocal"),
        }
    }
}
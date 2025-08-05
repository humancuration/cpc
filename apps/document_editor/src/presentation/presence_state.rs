//! Unified presence state management with LRU caching

use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};
use lru::LruCache;
use std::num::NonZeroUsize;

/// Rectangle representing a viewport or region
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

/// Unified structure for user presence state
#[derive(Debug, Clone)]
pub struct UserPresenceState {
    pub user: PresenceUser,
    pub cursor_position: Option<(usize, usize)>,
    pub viewport: Option<Rect>,
    pub resolution_level: f64,
    pub last_active: DateTime<Utc>,
    pub is_typing: bool,
}

impl UserPresenceState {
    /// Create a new UserPresenceState
    pub fn new(user: PresenceUser, last_active: DateTime<Utc>) -> Self {
        Self {
            user,
            cursor_position: None,
            viewport: None,
            resolution_level: 1.0, // Default resolution level
            last_active,
            is_typing: false,
        }
    }
    
    /// Update the cursor position
    pub fn update_cursor_position(&mut self, position: (usize, usize)) {
        self.cursor_position = Some(position);
        self.last_active = Utc::now();
    }
    
    /// Update the viewport
    pub fn update_viewport(&mut self, viewport: Rect) {
        self.viewport = Some(viewport);
        self.last_active = Utc::now();
    }
    
    /// Update the resolution level
    pub fn update_resolution_level(&mut self, resolution: f64) {
        self.resolution_level = resolution;
        self.last_active = Utc::now();
    }
    
    /// Update typing status
    pub fn set_typing(&mut self, is_typing: bool) {
        self.is_typing = is_typing;
        if is_typing {
            self.last_active = Utc::now();
        }
    }
    
    /// Update presence user information
    pub fn update_user(&mut self, user: PresenceUser) {
        self.user = user;
        self.last_active = Utc::now();
    }
}

/// LRU-based presence state manager
pub struct PresenceStateManager {
    /// LRU cache for presence states
    cache: LruCache<Uuid, UserPresenceState>,
    
    /// Maximum cache capacity
    capacity: usize,
}

impl PresenceStateManager {
    /// Create a new presence state manager with specified capacity
    pub fn new(capacity: usize) -> Self {
        let cache_capacity = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(100).unwrap());
        Self {
            cache: LruCache::new(cache_capacity),
            capacity,
        }
    }
    
    /// Get presence state for a user
    pub fn get(&mut self, user_id: &Uuid) -> Option<&UserPresenceState> {
        self.cache.get(user_id)
    }
    
    /// Get mutable presence state for a user
    pub fn get_mut(&mut self, user_id: &Uuid) -> Option<&mut UserPresenceState> {
        self.cache.get_mut(user_id)
    }
    
    /// Insert or update presence state for a user
    pub fn insert(&mut self, user_id: Uuid, state: UserPresenceState) {
        self.cache.put(user_id, state);
    }
    
    /// Remove presence state for a user
    pub fn remove(&mut self, user_id: &Uuid) -> Option<UserPresenceState> {
        self.cache.pop(user_id)
    }
    
    /// Check if a user is present
    pub fn contains(&mut self, user_id: &Uuid) -> bool {
        self.cache.contains(user_id)
    }
    
    /// Get all presence states
    pub fn iter(&mut self) -> impl Iterator<Item = (&Uuid, &UserPresenceState)> {
        self.cache.iter()
    }
    
    /// Get mutable iterator over all presence states
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Uuid, &mut UserPresenceState)> {
        self.cache.iter_mut()
    }
    
    /// Get the number of cached presence states
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
    
    /// Clear all presence states
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// Update user status based on inactivity
    pub fn update_user_status(&mut self, user_id: &Uuid, away_threshold: i64, offline_threshold: i64) {
        if let Some(state) = self.cache.get_mut(user_id) {
            let now = Utc::now();
            let inactive_duration = now.signed_duration_since(state.last_active);
            let inactive_secs = inactive_duration.num_seconds();
            
            if inactive_secs > offline_threshold {
                // User is offline, remove from cache
                self.cache.pop(user_id);
            } else if inactive_secs > away_threshold {
                // User is away
                state.user.status = PresenceStatus::Away;
            }
        }
    }
    
    /// Update all user statuses based on inactivity
    pub fn update_all_statuses(&mut self, away_threshold: i64, offline_threshold: i64) {
        let user_ids: Vec<Uuid> = self.cache.iter().map(|(id, _)| *id).collect();
        
        for user_id in user_ids {
            self.update_user_status(&user_id, away_threshold, offline_threshold);
        }
    }
}

impl Default for PresenceStateManager {
    fn default() -> Self {
        Self::new(1000) // Default capacity of 1000 users
    }
}

/// Helper functions for converting between presence state and signaling messages
pub mod converters {
    use super::*;
    use shared_packages::realtime_signaling::message::{PresenceUpdate, CursorPosition, ViewportUpdate};
    
    /// Convert PresenceUpdate to UserPresenceState
    pub fn presence_update_to_state(update: &PresenceUpdate) -> UserPresenceState {
        let status = if update.is_typing {
            PresenceStatus::Online
        } else {
            PresenceStatus::Online // Default to online, expiration logic will handle away status
        };
        
        let user = PresenceUser {
            avatar_url: update.avatar_url.clone(),
            color: update.color.clone(),
            status,
        };
        
        UserPresenceState {
            user,
            cursor_position: update.cursor.as_ref().map(|pos| (pos.line, pos.column)),
            viewport: None, // Viewport is handled separately via ViewportUpdate
            resolution_level: 1.0, // Default resolution level
            last_active: update.last_active,
            is_typing: update.is_typing,
        }
    }
    
    /// Convert ViewportUpdate to UserPresenceState update
    pub fn viewport_update_to_state(update: &ViewportUpdate, state: &mut UserPresenceState) {
        let viewport = Rect::new(
            update.viewport.x,
            update.viewport.y,
            update.viewport.width,
            update.viewport.height,
        );
        state.update_viewport(viewport);
        state.update_resolution_level(update.resolution);
    }
    
    /// Convert UserPresenceState to PresenceUser
    pub fn state_to_presence_user(state: &UserPresenceState) -> PresenceUser {
        state.user.clone()
    }
    
    /// Convert UserPresenceState to cursor position
    pub fn state_to_cursor_position(state: &UserPresenceState) -> Option<(usize, usize)> {
        state.cursor_position
    }
    
    /// Convert UserPresenceState to viewport
    pub fn state_to_viewport(state: &UserPresenceState) -> Option<Rect> {
        state.viewport.clone()
    }
}
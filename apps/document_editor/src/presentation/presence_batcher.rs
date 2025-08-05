//! Presence update batching for performance optimization

use std::collections::HashMap;
use std::time::{Instant, Duration};
use uuid::Uuid;
use shared_packages::realtime_signaling::message::{PresenceUpdate, CursorPosition};

/// A batcher for presence updates to reduce network traffic
pub struct PresenceUpdateBatcher {
    /// Buffer of pending presence updates
    presence_buffer: HashMap<Uuid, PresenceUpdate>,
    
    /// Buffer of pending cursor updates
    cursor_buffer: HashMap<Uuid, CursorPosition>,
    
    /// Timestamp of last flush
    last_flush: Instant,
    
    /// Flush interval (milliseconds)
    flush_interval: u64,
    
    /// Maximum batch size before forced flush
    max_batch_size: usize,
}

impl PresenceUpdateBatcher {
    /// Create a new presence update batcher
    pub fn new(flush_interval_ms: u64, max_batch_size: usize) -> Self {
        Self {
            presence_buffer: HashMap::new(),
            cursor_buffer: HashMap::new(),
            last_flush: Instant::now(),
            flush_interval: flush_interval_ms,
            max_batch_size,
        }
    }
    
    /// Add a presence update to the batch
    pub fn add_presence_update(&mut self, update: PresenceUpdate) {
        self.presence_buffer.insert(update.user_id, update);
    }
    
    /// Add a cursor update to the batch
    pub fn add_cursor_update(&mut self, cursor: CursorPosition) {
        self.cursor_buffer.insert(cursor.user_id, cursor);
    }
    
    /// Check if the batch should be flushed
    pub fn should_flush(&self) -> bool {
        // Flush if enough time has passed
        let elapsed = self.last_flush.elapsed().as_millis() as u64;
        if elapsed >= self.flush_interval {
            return true;
        }
        
        // Flush if batch is too large
        if self.presence_buffer.len() + self.cursor_buffer.len() >= self.max_batch_size {
            return true;
        }
        
        false
    }
    
    /// Flush the batch and return the updates
    pub fn flush(&mut self) -> (Vec<PresenceUpdate>, Vec<CursorPosition>) {
        self.last_flush = Instant::now();
        
        let presence_updates: Vec<PresenceUpdate> = 
            self.presence_buffer.drain().map(|(_, update)| update).collect();
        
        let cursor_updates: Vec<CursorPosition> = 
            self.cursor_buffer.drain().map(|(_, cursor)| cursor).collect();
        
        (presence_updates, cursor_updates)
    }
    
    /// Get the number of pending updates
    pub fn pending_count(&self) -> usize {
        self.presence_buffer.len() + self.cursor_buffer.len()
    }
}

impl Default for PresenceUpdateBatcher {
    fn default() -> Self {
        Self::new(100, 10) // 100ms flush interval, max 10 updates
    }
}

/// Virtualized cursor manager for efficient rendering
pub struct CursorVirtualizer {
    /// Viewport dimensions
    viewport: (f64, f64, f64, f64), // x, y, width, height
    
    /// Scroll offset
    scroll_offset: (f64, f64),
    
    /// Character dimensions for positioning
    char_dimensions: (f64, f64), // width, height
}

impl CursorVirtualizer {
    /// Create a new cursor virtualizer
    pub fn new(viewport_width: f64, viewport_height: f64) -> Self {
        Self {
            viewport: (0.0, 0.0, viewport_width, viewport_height),
            scroll_offset: (0.0, 0.0),
            char_dimensions: (8.0, 20.0), // default character width and height
        }
    }
    
    /// Update viewport dimensions
    pub fn set_viewport(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.viewport = (x, y, width, height);
    }
    
    /// Update scroll offset
    pub fn set_scroll_offset(&mut self, x: f64, y: f64) {
        self.scroll_offset = (x, y);
    }
    
    /// Update character dimensions
    pub fn set_char_dimensions(&mut self, width: f64, height: f64) {
        self.char_dimensions = (width, height);
    }
    
    /// Check if a cursor position is visible in the viewport
    pub fn is_cursor_visible(&self, line: usize, column: usize) -> bool {
        let (viewport_x, viewport_y, viewport_width, viewport_height) = self.viewport;
        let (scroll_x, scroll_y) = self.scroll_offset;
        let (char_width, char_height) = self.char_dimensions;
        
        // Calculate screen position
        let screen_x = (column as f64 * char_width) - scroll_x;
        let screen_y = (line as f64 * char_height) - scroll_y;
        
        // Check if position is within viewport
        screen_x >= viewport_x 
            && screen_x <= viewport_x + viewport_width 
            && screen_y >= viewport_y 
            && screen_y <= viewport_y + viewport_height
    }
    
    /// Filter cursor positions to only those that are visible
    pub fn filter_visible_cursors(&self, cursors: &HashMap<Uuid, (usize, usize)>) -> HashMap<Uuid, (usize, usize)> {
        cursors.iter()
            .filter(|(_, (line, column))| self.is_cursor_visible(**line, **column))
            .map(|(id, pos)| (*id, *pos))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_batcher_basic_functionality() {
        let mut batcher = PresenceUpdateBatcher::new(100, 5);
        
        // Add some updates
        let update = PresenceUpdate {
            document_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            cursor: None,
            selection: None,
            is_typing: false,
            avatar_url: None,
            color: "#ff0000".to_string(),
            last_active: Utc::now(),
            timestamp: Utc::now(),
        };
        
        batcher.add_presence_update(update.clone());
        assert_eq!(batcher.pending_count(), 1);
        
        // Flush should return the update
        let (presence_updates, cursor_updates) = batcher.flush();
        assert_eq!(presence_updates.len(), 1);
        assert_eq!(cursor_updates.len(), 0);
        assert_eq!(batcher.pending_count(), 0);
    }
    
    #[test]
    fn test_batcher_flush_conditions() {
        let mut batcher = PresenceUpdateBatcher::new(10, 3); // 10ms flush, max 3 updates
        
        // Add updates up to max batch size
        for _ in 0..3 {
            let update = PresenceUpdate {
                document_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                cursor: None,
                selection: None,
                is_typing: false,
                avatar_url: None,
                color: "#ff0000".to_string(),
                last_active: Utc::now(),
                timestamp: Utc::now(),
            };
            batcher.add_presence_update(update);
        }
        
        // Should flush due to max batch size
        assert!(batcher.should_flush());
    }
    
    #[test]
    fn test_cursor_virtualizer_visibility() {
        let mut virtualizer = CursorVirtualizer::new(800.0, 600.0); // 800x600 viewport
        virtualizer.set_char_dimensions(8.0, 20.0); // 8px wide, 20px tall characters
        
        // Cursor at position (5, 10) = 80px, 100px
        assert!(virtualizer.is_cursor_visible(5, 10));
        
        // Cursor outside viewport
        assert!(!virtualizer.is_cursor_visible(1000, 1000));
    }
}
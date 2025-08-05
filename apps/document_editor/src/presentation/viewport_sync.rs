//! Viewport synchronization for real-time collaborative cursors

use std::time::{Instant, Duration};
use uuid::Uuid;
use shared_packages::realtime_signaling::message::{ViewportUpdate, Rect as SignalRect};

/// Rectangle representing a viewport or region
#[derive(Debug, Clone, PartialEq)]
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
    
    /// Calculate the area of the rectangle
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    /// Calculate the intersection area with another rectangle
    pub fn intersection_area(&self, other: &Rect) -> f64 {
        let left = self.x.max(other.x);
        let right = (self.x + self.width).min(other.x + other.width);
        let top = self.y.max(other.y);
        let bottom = (self.y + self.height).min(other.y + other.height);
        
        if left < right && top < bottom {
            (right - left) * (bottom - top)
        } else {
            0.0
        }
    }
    
    /// Calculate the similarity ratio with another rectangle (0.0 to 1.0)
    pub fn similarity(&self, other: &Rect) -> f64 {
        let intersection = self.intersection_area(other);
        let union = self.area() + other.area() - intersection;
        
        if union > 0.0 {
            intersection / union
        } else {
            1.0 // Both rectangles are empty
        }
    }
}

/// Viewport synchronization manager
pub struct ViewportSyncManager {
    /// Last sent viewport update
    last_sent: Option<Rect>,
    
    /// Timestamp of last update
    last_update: Instant,
    
    /// Minimum time between updates (throttling)
    min_update_interval: Duration,
    
    /// Minimum change threshold to send update (0.0 to 1.0)
    min_change_threshold: f64,
    
    /// Resolution level for LOD
    resolution_level: f64,
}

impl ViewportSyncManager {
    /// Create a new viewport sync manager
    pub fn new() -> Self {
        Self {
            last_sent: None,
            last_update: Instant::now(),
            min_update_interval: Duration::from_millis(250), // 250ms throttling
            min_change_threshold: 0.1, // 10% change threshold
            resolution_level: 1.0, // Default resolution
        }
    }
    
    /// Create a new viewport sync manager with custom settings
    pub fn with_settings(min_update_interval_ms: u64, min_change_threshold: f64) -> Self {
        Self {
            last_sent: None,
            last_update: Instant::now(),
            min_update_interval: Duration::from_millis(min_update_interval_ms),
            min_change_threshold,
            resolution_level: 1.0,
        }
    }
    
    /// Check if an update should be sent based on throttling and change threshold
    pub fn should_send_update(&self, current: &Rect) -> bool {
        // Check throttling
        if self.last_update.elapsed() < self.min_update_interval {
            return false;
        }
        
        // Check if there's a significant change
        if let Some(last) = &self.last_sent {
            let similarity = last.similarity(current);
            // If similarity is high (close to 1.0), change is small, so don't send
            // If similarity is low (close to 0.0), change is large, so send
            (1.0 - similarity) >= self.min_change_threshold
        } else {
            // No previous update, so send this one
            true
        }
    }
    
    /// Prepare a viewport update message if needed
    pub fn prepare_update(&mut self, user_id: Uuid, document_id: Uuid, current: Rect) -> Option<ViewportUpdate> {
        if self.should_send_update(&current) {
            self.last_sent = Some(current.clone());
            self.last_update = Instant::now();
            
            Some(ViewportUpdate {
                user_id,
                document_id,
                viewport: SignalRect {
                    x: current.x,
                    y: current.y,
                    width: current.width,
                    height: current.height,
                },
                resolution: self.resolution_level,
                timestamp: chrono::Utc::now(),
            })
        } else {
            None
        }
    }
    
    /// Update the resolution level
    pub fn set_resolution_level(&mut self, resolution: f64) {
        self.resolution_level = resolution;
    }
    
    /// Get the current resolution level
    pub fn resolution_level(&self) -> f64 {
        self.resolution_level
    }
}

impl Default for ViewportSyncManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_rect_similarity() {
        // Identical rectangles
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(rect1.similarity(&rect2), 1.0);
        
        // Non-overlapping rectangles
        let rect3 = Rect::new(200.0, 200.0, 100.0, 100.0);
        assert_eq!(rect1.similarity(&rect3), 0.0);
        
        // Partially overlapping rectangles
        let rect4 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let similarity = rect1.similarity(&rect4);
        assert!(similarity > 0.0 && similarity < 1.0);
    }
    
    #[test]
    fn test_should_send_update_no_previous() {
        let manager = ViewportSyncManager::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(manager.should_send_update(&rect));
    }
    
    #[test]
    fn test_should_send_update_identical() {
        let mut manager = ViewportSyncManager::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        // First update should be sent
        assert!(manager.should_send_update(&rect));
        
        // Send the update
        manager.last_sent = Some(rect.clone());
        manager.last_update = Instant::now();
        
        // Identical update should not be sent
        assert!(!manager.should_send_update(&rect));
    }
    
    #[test]
    fn test_should_send_update_significant_change() {
        let mut manager = ViewportSyncManager::new();
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        // Send first update
        manager.last_sent = Some(rect1.clone());
        manager.last_update = Instant::now();
        
        // Significantly different rectangle should be sent
        let rect2 = Rect::new(200.0, 200.0, 100.0, 100.0);
        assert!(manager.should_send_update(&rect2));
    }
    
    #[test]
    fn test_throttling() {
        let mut manager = ViewportSyncManager::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        // First update should be sent
        assert!(manager.should_send_update(&rect));
        
        // Send the update
        manager.last_sent = Some(rect.clone());
        manager.last_update = Instant::now();
        
        // Second update immediately should be throttled
        assert!(!manager.should_send_update(&rect));
    }
    
    #[test]
    fn test_prepare_update() {
        let mut manager = ViewportSyncManager::new();
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        let update = manager.prepare_update(user_id, document_id, rect.clone());
        assert!(update.is_some());
        
        let update = update.unwrap();
        assert_eq!(update.user_id, user_id);
        assert_eq!(update.document_id, document_id);
        assert_eq!(update.viewport.x, rect.x);
        assert_eq!(update.viewport.y, rect.y);
        assert_eq!(update.viewport.width, rect.width);
        assert_eq!(update.viewport.height, rect.height);
        assert_eq!(update.resolution, manager.resolution_level());
    }
}
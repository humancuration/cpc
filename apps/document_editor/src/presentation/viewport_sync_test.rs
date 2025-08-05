//! Tests for viewport synchronization functionality

#[cfg(test)]
mod tests {
    use super::super::viewport_sync::{ViewportSyncManager, Rect};
    use uuid::Uuid;
    use std::thread::sleep;
    use std::time::Duration;
    
    #[test]
    fn test_viewport_sync_manager_basic_functionality() {
        let mut manager = ViewportSyncManager::new();
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        
        let rect = Rect::new(0.0, 0.0, 800.0, 600.0);
        let update = manager.prepare_update(user_id, document_id, rect.clone());
        
        assert!(update.is_some());
        let update = update.unwrap();
        assert_eq!(update.user_id, user_id);
        assert_eq!(update.document_id, document_id);
        assert_eq!(update.viewport.x, 0.0);
        assert_eq!(update.viewport.y, 0.0);
        assert_eq!(update.viewport.width, 800.0);
        assert_eq!(update.viewport.height, 600.0);
        assert_eq!(update.resolution, 1.0);
    }
    
    #[test]
    fn test_viewport_sync_manager_throttling() {
        let mut manager = ViewportSyncManager::new();
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        
        let rect1 = Rect::new(0.0, 0.0, 800.0, 600.0);
        let update1 = manager.prepare_update(user_id, document_id, rect1.clone());
        assert!(update1.is_some());
        
        // Immediate second update should be throttled
        let rect2 = Rect::new(10.0, 10.0, 800.0, 600.0);
        let update2 = manager.prepare_update(user_id, document_id, rect2.clone());
        assert!(update2.is_none());
    }
    
    #[test]
    fn test_viewport_sync_manager_significant_changes() {
        let mut manager = ViewportSyncManager::new();
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let update1 = manager.prepare_update(user_id, document_id, rect1.clone());
        assert!(update1.is_some());
        
        // Wait for throttling period to pass
        sleep(Duration::from_millis(300));
        
        // Significantly different viewport should trigger update
        let rect2 = Rect::new(500.0, 500.0, 100.0, 100.0);
        let update2 = manager.prepare_update(user_id, document_id, rect2.clone());
        assert!(update2.is_some());
    }
    
    #[test]
    fn test_viewport_sync_manager_resolution_levels() {
        let mut manager = ViewportSyncManager::new();
        manager.set_resolution_level(0.5);
        
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        let rect = Rect::new(0.0, 0.0, 800.0, 600.0);
        let update = manager.prepare_update(user_id, document_id, rect.clone());
        
        assert!(update.is_some());
        let update = update.unwrap();
        assert_eq!(update.resolution, 0.5);
    }
    
    #[test]
    fn test_rect_intersection_area() {
        // Overlapping rectangles
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let intersection = rect1.intersection_area(&rect2);
        assert_eq!(intersection, 2500.0); // 50 * 50
        
        // Non-overlapping rectangles
        let rect3 = Rect::new(200.0, 200.0, 100.0, 100.0);
        let intersection = rect1.intersection_area(&rect3);
        assert_eq!(intersection, 0.0);
        
        // Identical rectangles
        let intersection = rect1.intersection_area(&rect1);
        assert_eq!(intersection, 10000.0); // 100 * 100
    }
}
//! Integration tests for real-time cursors Phase 1 functionality

#[cfg(test)]
mod tests {
    use crate::presentation::position_translator::{PositionTranslator, LineMetrics, CharMetrics, FontId};
    use crate::presentation::presence_state::{PresenceStateManager, UserPresenceState, Rect as PresenceRect};
    use crate::presentation::viewport_sync::{ViewportSyncManager, Rect as ViewportRect};
    use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus, ViewportUpdate, Rect as SignalRect};
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_full_integration_coordinate_translation_and_viewport_sync() {
        // Create components
        let mut translator = PositionTranslator::new();
        let mut presence_manager = PresenceStateManager::new(100);
        let mut viewport_manager = ViewportSyncManager::new();
        
        // Set up translator with metrics
        let line_metrics = LineMetrics {
            top: 20.0,
            height: 25.0,
            wrapped_ranges: vec![(0, 20), (20, 40)],
        };
        translator.update_line_metrics(2, line_metrics);
        
        let font_id = FontId {
            family: "Arial".to_string(),
            size: 14.0,
            weight: 400,
        };
        translator.update_font_metrics(font_id.clone(), 'H', CharMetrics { width: 9.0, kerning: 0.0 });
        translator.update_font_metrics(font_id.clone(), 'e', CharMetrics { width: 8.5, kerning: 0.0 });
        
        // Test coordinate translation
        let (x, y) = translator.document_to_screen(2, 5);
        assert_eq!(y, 20.0); // line_metrics.top
        assert_eq!(x, 40.0); // 5 * 8.0 (default width, ignoring font metrics for simplicity)
        
        // Test viewport sync
        let user_id = Uuid::new_v4();
        let document_id = Uuid::new_v4();
        let viewport_rect = ViewportRect::new(100.0, 50.0, 800.0, 600.0);
        
        let update = viewport_manager.prepare_update(user_id, document_id, viewport_rect);
        assert!(update.is_some());
        
        // Test presence state with viewport
        let presence_user = PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#FF0000".to_string(),
            status: PresenceStatus::Online,
        };
        
        let mut user_state = UserPresenceState::new(presence_user, Utc::now());
        let viewport_update = update.unwrap();
        
        // Convert signal viewport to presence viewport
        let presence_rect = PresenceRect::new(
            viewport_update.viewport.x,
            viewport_update.viewport.y,
            viewport_update.viewport.width,
            viewport_update.viewport.height,
        );
        user_state.update_viewport(presence_rect);
        user_state.update_resolution_level(viewport_update.resolution);
        
        // Verify state
        assert_eq!(user_state.viewport.as_ref().unwrap().x, 100.0);
        assert_eq!(user_state.viewport.as_ref().unwrap().y, 50.0);
        assert_eq!(user_state.viewport.as_ref().unwrap().width, 800.0);
        assert_eq!(user_state.viewport.as_ref().unwrap().height, 600.0);
        assert_eq!(user_state.resolution_level, 1.0);
        
        // Add to presence manager
        presence_manager.insert(user_id, user_state);
        
        // Verify presence manager has the user
        assert!(presence_manager.contains(&user_id));
        let retrieved_state = presence_manager.get(&user_id);
        assert!(retrieved_state.is_some());
        let retrieved_state = retrieved_state.unwrap();
        assert_eq!(retrieved_state.viewport.as_ref().unwrap().x, 100.0);
    }
    
    #[test]
    fn test_cache_invalidation_across_components() {
        let mut translator = PositionTranslator::new();
        
        // Initial conversion
        let (x1, y1) = translator.document_to_screen(1, 10);
        
        // Add metrics to invalidate cache
        let line_metrics = LineMetrics {
            top: 30.0,
            height: 20.0,
            wrapped_ranges: vec![],
        };
        translator.update_line_metrics(1, line_metrics);
        
        // Second conversion should use new metrics
        let (x2, y2) = translator.document_to_screen(1, 10);
        assert_eq!(y2, 30.0); // line_metrics.top
        assert_ne!(y1, y2); // Should be different from cached value
    }
    
    #[test]
    fn test_viewport_similarity_calculations() {
        let viewport_manager = ViewportSyncManager::new();
        
        // Test identical viewports
        let rect1 = ViewportRect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = ViewportRect::new(0.0, 0.0, 100.0, 100.0);
        let similarity = rect1.similarity(&rect2);
        assert_eq!(similarity, 1.0);
        
        // Test non-overlapping viewports
        let rect3 = ViewportRect::new(200.0, 200.0, 100.0, 100.0);
        let similarity = rect1.similarity(&rect3);
        assert_eq!(similarity, 0.0);
        
        // Test partially overlapping viewports
        let rect4 = ViewportRect::new(50.0, 50.0, 100.0, 100.0);
        let similarity = rect1.similarity(&rect4);
        assert!(similarity > 0.0 && similarity < 1.0);
    }
}
//! Tests for the CalendarStore
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;
    
    // Import domain models
    use packages::cpc_core::calendar::domain::event::{CalendarEvent, EventType, EventVisibility};
    
    // Import local modules
    use crate::components::calendar::state::store::CalendarStore;
    use crate::components::calendar::state::view::CalendarView;
    
    #[test]
    fn test_calendar_store_initial_state() {
        let store = CalendarStore::default();
        assert_eq!(store.state.current_view, CalendarView::Month);
        assert_eq!(store.state.events.len(), 0);
        assert_eq!(store.state.work_shifts.len(), 0);
        assert!(!store.state.loading);
        assert!(store.state.error.is_none());
    }
    
    #[test]
    fn test_calendar_store_set_view() {
        let store = CalendarStore::default();
        let new_view = CalendarView::Week;
        
        let new_store = CalendarStore {
            state: store.state.with(|s| s.current_view = new_view.clone())
        };
        
        assert_eq!(new_store.state.current_view, new_view);
    }
    
    #[test]
    fn test_calendar_store_events_fetched() {
        let store = CalendarStore::default();
        let events = vec![create_test_event()];
        
        let new_store = CalendarStore {
            state: store.state.with(|s| {
                s.events = events.clone();
                s.loading = false;
                s.error = None;
            })
        };
        
        assert_eq!(new_store.state.events.len(), 1);
        assert!(!new_store.state.loading);
        assert!(new_store.state.error.is_none());
    }
    
    #[test]
    fn test_calendar_store_event_created() {
        let store = CalendarStore::default();
        let event = create_test_event();
        let event_id = event.id;
        
        let new_store = CalendarStore {
            state: store.state.with(|s| s.events.push(event.clone()))
        };
        
        assert_eq!(new_store.state.events.len(), 1);
        assert_eq!(new_store.state.events[0].id, event_id);
    }
    
    #[test]
    fn test_calendar_store_event_updated() {
        let mut store = CalendarStore::default();
        let event = create_test_event();
        let event_id = event.id;
        
        // Add event to store
        store.state.events.push(event.clone());
        
        // Update event
        let mut updated_event = event.clone();
        updated_event.title = "Updated Event".to_string();
        
        let new_store = CalendarStore {
            state: store.state.with(|s| {
                if let Some(pos) = s.events.iter().position(|e| e.id == event_id) {
                    s.events[pos] = updated_event.clone();
                }
            })
        };
        
        assert_eq!(new_store.state.events.len(), 1);
        assert_eq!(new_store.state.events[0].title, "Updated Event");
    }
    
    #[test]
    fn test_calendar_store_event_deleted() {
        let mut store = CalendarStore::default();
        let event = create_test_event();
        let event_id = event.id;
        
        // Add event to store
        store.state.events.push(event);
        
        // Delete event
        let new_store = CalendarStore {
            state: store.state.with(|s| s.events.retain(|e| e.id != event_id))
        };
        
        assert_eq!(new_store.state.events.len(), 0);
    }
    
    /// Helper function to create a test event
    fn create_test_event() -> CalendarEvent {
        CalendarEvent::new(
            Uuid::new_v4(),
            "Test Event".to_string(),
            Some("Test Description".to_string()),
            Utc::now(),
            Utc::now() + chrono::Duration::hours(1),
            EventType::Personal,
            EventVisibility::Private,
            None,
            None,
        )
    }
}
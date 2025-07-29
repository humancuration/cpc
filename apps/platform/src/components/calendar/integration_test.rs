//! Integration tests for the calendar components
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;
    
    // Import domain models
    use packages::cpc_core::calendar::domain::event::{CalendarEvent, EventType, EventVisibility};
    
    // Import local modules
    use crate::components::calendar::state::store::{CalendarStore, CalendarAction};
    use crate::components::calendar::state::view::CalendarView;
    use crate::components::calendar::utils::format_recurrence::format_recurrence;
    
    #[test]
    fn test_calendar_store_with_events() {
        // Create a calendar store
        let store = CalendarStore::default();
        
        // Create a test event
        let event = CalendarEvent::new(
            Uuid::new_v4(),
            "Test Event".to_string(),
            Some("Test Description".to_string()),
            Utc::now(),
            Utc::now() + chrono::Duration::hours(1),
            EventType::Personal,
            EventVisibility::Private,
            None,
            None,
        );
        
        // Add the event to the store
        let new_store = CalendarStore {
            state: store.state.with(|s| s.events.push(event.clone()))
        };
        
        // Verify the event was added
        assert_eq!(new_store.state.events.len(), 1);
        assert_eq!(new_store.state.events[0].id, event.id);
    }
    
    #[test]
    fn test_calendar_view_switching() {
        // Create a calendar store
        let store = CalendarStore::default();
        
        // Change the view to Week
        let new_store = CalendarStore {
            state: store.state.with(|s| s.current_view = CalendarView::Week)
        };
        
        // Verify the view was changed
        assert_eq!(new_store.state.current_view, CalendarView::Week);
    }
    
    #[test]
    fn test_format_recurrence_integration() {
        // Create a recurrence rule
        let rule = packages::cpc_core::calendar::domain::event::RecurrenceRule {
            frequency: packages::cpc_core::calendar::domain::event::RecurrenceFrequency::Weekly,
            interval: 1,
            by_day: None,
            by_month_day: None,
            until: None,
            count: Some(5),
        };
        
        // Format the recurrence
        let result = format_recurrence(&rule);
        
        // Verify the result
        assert_eq!(result, "Every week for 5 occurrences");
    }
}
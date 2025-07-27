//! Tests for the calendar event domain model

#[cfg(test)]
mod tests {
    use super::super::event::*;
    use chrono::{Utc, Duration};
    use uuid::Uuid;

    #[test]
    fn test_create_calendar_event() {
        let user_id = Uuid::new_v4();
        let start = Utc::now();
        let end = start + Duration::hours(1);
        
        let event = CalendarEvent::new(
            user_id,
            "Test Event".to_string(),
            Some("Test Description".to_string()),
            start,
            end,
            EventType::Personal,
            EventVisibility::Private,
            None,
            None,
        );
        
        assert_eq!(event.user_id, user_id);
        assert_eq!(event.title, "Test Event");
        assert_eq!(event.description, Some("Test Description".to_string()));
        assert_eq!(event.start, start);
        assert_eq!(event.end, end);
        assert_eq!(event.event_type, EventType::Personal);
        assert_eq!(event.visibility, EventVisibility::Private);
        assert!(event.recurrence.is_none());
        assert!(event.location.is_none());
        assert!(event.attachments.is_empty());
    }

    #[test]
    fn test_is_recurring() {
        let user_id = Uuid::new_v4();
        let start = Utc::now();
        let end = start + Duration::hours(1);
        
        let mut event = CalendarEvent::new(
            user_id,
            "Test Event".to_string(),
            None,
            start,
            end,
            EventType::Personal,
            EventVisibility::Private,
            None,
            None,
        );
        
        assert!(!event.is_recurring());
        
        event.recurrence = Some(RecurrenceRule {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            by_day: None,
            by_month_day: None,
            until: None,
            count: None,
        });
        
        assert!(event.is_recurring());
    }

    #[test]
    fn test_update_event() {
        let user_id = Uuid::new_v4();
        let start = Utc::now();
        let end = start + Duration::hours(1);
        
        let mut event = CalendarEvent::new(
            user_id,
            "Test Event".to_string(),
            None,
            start,
            end,
            EventType::Personal,
            EventVisibility::Private,
            None,
            None,
        );
        
        let updated_start = start + Duration::hours(2);
        let updates = EventUpdate::new()
            .title("Updated Event".to_string())
            .start(updated_start);
            
        event.update(updates);
        
        assert_eq!(event.title, "Updated Event");
        assert_eq!(event.start, updated_start);
        assert_eq!(event.end, end); // Should remain unchanged
    }

    #[test]
    fn test_event_update_builder() {
        let update = EventUpdate::new()
            .title("Test Event".to_string())
            .description("Test Description".to_string())
            .event_type(EventType::Business);
            
        assert_eq!(update.title, Some("Test Event".to_string()));
        assert_eq!(update.description, Some("Test Description".to_string()));
        assert_eq!(update.event_type, Some(EventType::Business));
    }
}
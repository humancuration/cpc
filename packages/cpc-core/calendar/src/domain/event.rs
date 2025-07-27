//! Calendar event domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub event_type: EventType,
    pub visibility: EventVisibility,
    pub recurrence: Option<RecurrenceRule>,
    pub location: Option<Location>,
    pub attachments: Vec<EventAttachment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CalendarEvent {
    /// Create a new calendar event
    pub fn new(
        user_id: Uuid,
        title: String,
        description: Option<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        event_type: EventType,
        visibility: EventVisibility,
        recurrence: Option<RecurrenceRule>,
        location: Option<Location>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            start,
            end,
            event_type,
            visibility,
            recurrence,
            location,
            attachments: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the event is recurring
    pub fn is_recurring(&self) -> bool {
        self.recurrence.is_some()
    }

    /// Update the event with new information
    pub fn update(&mut self, updates: EventUpdate) {
        if let Some(title) = updates.title {
            self.title = title;
        }
        if let Some(description) = updates.description {
            self.description = description;
        }
        if let Some(start) = updates.start {
            self.start = start;
        }
        if let Some(end) = updates.end {
            self.end = end;
        }
        if let Some(event_type) = updates.event_type {
            self.event_type = event_type;
        }
        if let Some(visibility) = updates.visibility {
            self.visibility = visibility;
        }
        if let Some(recurrence) = updates.recurrence {
            self.recurrence = recurrence;
        }
        if let Some(location) = updates.location {
            self.location = location;
        }
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

/// Types of events in the calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Personal,
    Business,
    Cooperative(String), // Cooperative ID for shared co-op events
    TaskDeadline(Uuid),  // References Task Manager module
    HealthAppointment,   // References Health module
}

/// Visibility settings for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventVisibility {
    Private,
    Shared(Vec<Uuid>),  // User IDs with access
    Public,
    CooperativeMembers(String), // Cooperative ID
}

/// Recurrence rule for repeating events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    pub frequency: RecurrenceFrequency,
    pub interval: u32,
    pub by_day: Option<Vec<Weekday>>,
    pub by_month_day: Option<Vec<u32>>,
    pub until: Option<DateTime<Utc>>,
    pub count: Option<u32>,
}

/// Frequency of recurrence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Day of the week
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Location information for an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>, // For geofencing
}

/// Attachment to an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttachment {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub mime_type: String,
    pub size: u64,
}

/// Structure for updating an event
#[derive(Debug, Default)]
pub struct EventUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub event_type: Option<EventType>,
    pub visibility: Option<EventVisibility>,
    pub recurrence: Option<Option<RecurrenceRule>>,
    pub location: Option<Option<Location>>,
}

impl EventUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn start(mut self, start: DateTime<Utc>) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: DateTime<Utc>) -> Self {
        self.end = Some(end);
        self
    }

    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn visibility(mut self, visibility: EventVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn recurrence(mut self, recurrence: Option<RecurrenceRule>) -> Self {
        self.recurrence = Some(recurrence);
        self
    }

    pub fn location(mut self, location: Option<Location>) -> Self {
        self.location = Some(location);
        self
    }
}
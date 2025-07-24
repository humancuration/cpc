use crate::graphql::client::GraphQLClient;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::lib::notifications::NotificationService;
use packages::cpc_core::auth::permissions::{Permission, check_permission};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<Uuid>, // Member IDs
    pub training_doc_id: Option<String>, // Reference to training document
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CalendarService {
    client: GraphQLClient,
    notification_service: NotificationService,
}

impl CalendarService {
    pub fn new(client: GraphQLClient, notification_service: NotificationService) -> Self {
        Self { client, notification_service }
    }

    pub async fn create_event(
        &self,
        title: String,
        description: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        location: Option<String>,
        attendees: Vec<Uuid>,
        training_doc_id: Option<String>,
        user_id: Uuid,
    ) -> Result<CalendarEvent, String> {
        // Check permissions
        if !check_permission(user_id, Permission::ManageTrainingSchedule).await {
            return Err("Insufficient permissions".to_string());
        }

        // Implementation will call GraphQL mutation
        let event = CalendarEvent {
            id: Uuid::new_v4(),
            title,
            description,
            start_time,
            end_time,
            location,
            attendees: attendees.clone(),
            training_doc_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Send notifications to attendees
        self.notification_service.send_notification(
            attendees,
            "New Training Session",
            &format!("You've been invited to: {}", event.title),
            Some("training_session"),
        ).await;

        Ok(event)
    }

    pub async fn update_event(
        &self,
        event_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        location: Option<Option<String>>,
        attendees: Option<Vec<Uuid>>,
        training_doc_id: Option<Option<String>>,
        user_id: Uuid,
    ) -> Result<CalendarEvent, String> {
        // Check permissions
        if !check_permission(user_id, Permission::ManageTrainingSchedule).await {
            return Err("Insufficient permissions".to_string());
        }

        // Implementation will call GraphQL mutation
        // Placeholder - in real implementation we'd fetch the event first
        let mut event = self.get_event(event_id).await?;
        
        if let Some(title) = title {
            event.title = title;
        }
        if let Some(description) = description {
            event.description = description;
        }
        if let Some(start_time) = start_time {
            event.start_time = start_time;
        }
        if let Some(end_time) = end_time {
            event.end_time = end_time;
        }
        if let Some(location) = location {
            event.location = location;
        }
        if let Some(attendees) = attendees {
            // Notify new attendees
            let new_attendees: Vec<Uuid> = attendees.iter()
                .filter(|id| !event.attendees.contains(id))
                .cloned()
                .collect();
            
            if !new_attendees.is_empty() {
                self.notification_service.send_notification(
                    new_attendees,
                    "Training Session Update",
                    &format!("You've been added to: {}", event.title),
                    Some("training_session"),
                ).await;
            }
            
            event.attendees = attendees;
        }
        if let Some(training_doc_id) = training_doc_id {
            event.training_doc_id = training_doc_id;
        }
        
        event.updated_at = Utc::now();

        Ok(event)
    }

    pub async fn get_event(&self, event_id: Uuid) -> Result<CalendarEvent, String> {
        // Implementation will call GraphQL query
        // Placeholder for demo
        Ok(CalendarEvent {
            id: event_id,
            title: "Sample Event".to_string(),
            description: "Sample description".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(1),
            location: Some("Online".to_string()),
            attendees: vec![],
            training_doc_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn list_events(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<CalendarEvent>, String> {
        // Implementation will call GraphQL query
        Ok(vec![])
    }
}
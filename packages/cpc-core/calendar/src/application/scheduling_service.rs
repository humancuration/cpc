//! Scheduling service for calendar events

use crate::domain::{
    CalendarEvent, EventUpdate, Participant, SchedulingConflict, TimeSlot, CalendarError,
    EventType, EventVisibility
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Repository trait for calendar events
#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn save(&self, event: &CalendarEvent) -> Result<(), CalendarError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CalendarEvent>, CalendarError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<CalendarEvent>, CalendarError>;
    async fn find_overlapping(&self, user_id: Uuid, slot: &TimeSlot) -> Result<Vec<CalendarEvent>, CalendarError>;
    async fn update(&self, event: &CalendarEvent) -> Result<(), CalendarError>;
    async fn delete(&self, id: Uuid) -> Result<(), CalendarError>;
}

/// Participant service trait
#[async_trait]
pub trait ParticipantService: Send + Sync {
    async fn check_availability(
        &self,
        participants: &[Participant],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<SchedulingConflict>, CalendarError>;
    
    async fn notify_participants(
        &self,
        event: &CalendarEvent,
        participants: &[Participant],
    ) -> Result<(), CalendarError>;
}

/// P2P manager trait for sharing events
#[async_trait]
pub trait P2PManager: Send + Sync {
    async fn share_event(
        &self,
        event: &CalendarEvent,
        participants: &[Participant],
    ) -> Result<(), CalendarError>;
}

/// Input for creating a new event
pub struct CreateEventInput {
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub event_type: EventType,
    pub visibility: EventVisibility,
    pub recurrence: Option<crate::domain::RecurrenceRule>,
    pub location: Option<crate::domain::Location>,
    pub participants: Vec<Participant>,
}

/// Scheduling service for managing calendar events
pub struct SchedulingService {
    repository: Arc<dyn EventRepository>,
    p2p_manager: Arc<dyn P2PManager>,
    participant_service: Arc<dyn ParticipantService>,
}

impl SchedulingService {
    /// Create a new scheduling service
    pub fn new(
        repository: Arc<dyn EventRepository>,
        p2p_manager: Arc<dyn P2PManager>,
        participant_service: Arc<dyn ParticipantService>,
    ) -> Self {
        Self {
            repository,
            p2p_manager,
            participant_service,
        }
    }

    /// Create a new calendar event
    pub async fn create_event(
        &self,
        user_id: Uuid,
        input: CreateEventInput,
    ) -> Result<CalendarEvent, CalendarError> {
        // Validate participant availability
        let conflicts = self.participant_service.check_availability(
            &input.participants,
            input.start,
            input.end
        ).await?;
        
        if !conflicts.is_empty() {
            return Err(CalendarError::SchedulingConflict(conflicts));
        }

        // Create event with proper visibility handling
        let event = CalendarEvent::new(
            user_id,
            input.title,
            input.description,
            input.start,
            input.end,
            input.event_type,
            input.visibility,
            input.recurrence,
            input.location,
        );

        // Share with participants through p2p network
        self.p2p_manager.share_event(&event, &input.participants).await?;
        self.repository.save(&event).await?;

        // Notify participants
        self.participant_service.notify_participants(&event, &input.participants).await?;

        Ok(event)
    }

    /// Update an existing calendar event
    pub async fn update_event(
        &self,
        user_id: Uuid,
        event_id: Uuid,
        updates: EventUpdate,
    ) -> Result<CalendarEvent, CalendarError> {
        let mut event = self.repository.find_by_id(event_id).await?
            .ok_or(CalendarError::EventNotFound)?;

        // Check authorization
        if event.user_id != user_id {
            return Err(CalendarError::Unauthorized);
        }

        // If time changed, check for conflicts
        if updates.start.is_some() || updates.end.is_some() {
            let start = updates.start.unwrap_or(event.start);
            let end = updates.end.unwrap_or(event.end);
            
            let slot = TimeSlot::new(start, end)?;
            let overlapping = self.repository.find_overlapping(user_id, &slot).await?;
            
            // Remove the current event from overlapping check
            let overlapping: Vec<_> = overlapping.into_iter()
                .filter(|e| e.id != event_id)
                .collect();
                
            if !overlapping.is_empty() {
                let conflicts = overlapping.into_iter()
                    .map(|e| SchedulingConflict::new(
                        e.id,
                        e.start.max(start),
                        e.end.min(end),
                        vec![user_id], // Simplified for now
                    ))
                    .collect();
                return Err(CalendarError::SchedulingConflict(conflicts));
            }
        }

        // Apply updates
        event.update(updates);

        // Save updated event
        self.repository.update(&event).await?;

        Ok(event)
    }

    /// Delete a calendar event
    pub async fn delete_event(
        &self,
        user_id: Uuid,
        event_id: Uuid,
    ) -> Result<(), CalendarError> {
        let event = self.repository.find_by_id(event_id).await?
            .ok_or(CalendarError::EventNotFound)?;

        // Check authorization
        if event.user_id != user_id {
            return Err(CalendarError::Unauthorized);
        }

        // Delete the event
        self.repository.delete(event_id).await?;

        Ok(())
    }

    /// Get a calendar event by ID
    pub async fn get_event(
        &self,
        user_id: Uuid,
        event_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        let event = self.repository.find_by_id(event_id).await?
            .ok_or(CalendarError::EventNotFound)?;

        // Check if user has access to this event
        if !self.user_has_access_to_event(user_id, &event) {
            return Err(CalendarError::Unauthorized);
        }

        Ok(event)
    }

    /// List events for a user within a date range
    pub async fn list_events(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        event_type: Option<EventType>,
    ) -> Result<Vec<CalendarEvent>, CalendarError> {
        let events = self.repository.find_by_user_id(user_id).await?;

        let filtered_events = events.into_iter()
            .filter(|event| {
                // Filter by date range if provided
                let in_date_range = match (start_date, end_date) {
                    (Some(start), Some(end)) => event.start <= end && event.end >= start,
                    (Some(start), None) => event.end >= start,
                    (None, Some(end)) => event.start <= end,
                    (None, None) => true,
                };

                // Filter by event type if provided
                let correct_type = if let Some(ref filter_type) = event_type {
                    std::mem::discriminant(filter_type) == std::mem::discriminant(&event.event_type)
                } else {
                    true
                };

                // Check access permissions
                let has_access = self.user_has_access_to_event(user_id, event);

                in_date_range && correct_type && has_access
            })
            .collect();

        Ok(filtered_events)
    }

    /// Check if a user has access to an event
    fn user_has_access_to_event(&self, user_id: Uuid, event: &CalendarEvent) -> bool {
        match &event.visibility {
            EventVisibility::Private => event.user_id == user_id,
            EventVisibility::Shared(user_ids) => {
                event.user_id == user_id || user_ids.contains(&user_id)
            },
            EventVisibility::Public => true,
            EventVisibility::CooperativeMembers(coop_id) => {
                // In a real implementation, we would check if the user is a member of the cooperative
                // For now, we'll just check if it's the owner
                event.user_id == user_id
            },
        }
    }
}
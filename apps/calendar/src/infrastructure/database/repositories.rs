//! Repository implementations for the calendar module

use crate::domain::{
    CalendarEvent, WorkShift, EventReminder, CalendarError, TimeSlot, EventType, EventVisibility,
    RecurrenceRule, Location, EventAttachment, CoverageStatus, StaffMember, AvailabilitySlot,
    AvailabilityType, Participant, ParticipantRole, ParticipationStatus
};
use crate::application::{
    EventRepository, ShiftRepository, ReminderRepository, ParticipantService, P2PManager
};
use crate::infrastructure::database::models::*;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime, Duration, Weekday as ChronoWeekday};
use serde_json;

/// Implementation of EventRepository using PostgreSQL
pub struct EventRepositoryImpl {
    pool: PgPool,
}

impl EventRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for EventRepositoryImpl {
    async fn save(&self, event: &CalendarEvent) -> Result<(), CalendarError> {
        // Serialize complex fields
        let event_type = serde_json::to_string(&event.event_type)
            .map_err(|_| CalendarError::InvalidEventDuration)?; // Using this error for simplicity
        let visibility = serde_json::to_string(&event.visibility)
            .map_err(|_| CalendarError::InvalidEventDuration)?;
        let recurrence = event.recurrence.as_ref()
            .map(|r| serde_json::to_string(r))
            .transpose()
            .map_err(|_| CalendarError::InvalidRecurrence("Failed to serialize".to_string))?;
        let location = event.location.as_ref()
            .map(|l| serde_json::to_string(l))
            .transpose()
            .map_err(|_| CalendarError::InvalidLocation)?;
        let attachments = if !event.attachments.is_empty() {
            Some(serde_json::to_string(&event.attachments)
                .map_err(|_| CalendarError::InvalidEventDuration)?)
        } else {
            None
        };

        sqlx::query!(
            r#"
            INSERT INTO calendar_events (
                id, user_id, title, description, start, "end", event_type, visibility,
                recurrence, location, attachments, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                title = $3, description = $4, start = $5, "end" = $6, event_type = $7,
                visibility = $8, recurrence = $9, location = $10, attachments = $11,
                updated_at = $13
            "#,
            event.id,
            event.user_id,
            event.title,
            event.description,
            event.start,
            event.end,
            event_type,
            visibility,
            recurrence,
            location,
            attachments,
            event.created_at,
            event.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CalendarError::InvalidEventDuration)?; // Simplified error handling

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CalendarEvent>, CalendarError> {
        let row = sqlx::query_as!(CalendarEventModel, r#"SELECT * FROM calendar_events WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        match row {
            Some(model) => {
                // Deserialize complex fields
                let event_type: EventType = serde_json::from_str(&model.event_type)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;
                let visibility: EventVisibility = serde_json::from_str(&model.visibility)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;
                let recurrence: Option<RecurrenceRule> = model.recurrence
                    .map(|r| serde_json::from_str(&r))
                    .transpose()
                    .map_err(|_| CalendarError::InvalidRecurrence("Failed to deserialize".to_string))?;
                let location: Option<Location> = model.location
                    .map(|l| serde_json::from_str(&l))
                    .transpose()
                    .map_err(|_| CalendarError::InvalidLocation)?;
                let attachments: Vec<EventAttachment> = model.attachments
                    .map(|a| serde_json::from_str(&a))
                    .transpose()
                    .map_err(|_| CalendarError::InvalidEventDuration)?
                    .unwrap_or_default();

                Ok(Some(CalendarEvent {
                    id: model.id,
                    user_id: model.user_id,
                    title: model.title,
                    description: model.description,
                    start: model.start,
                    end: model.end,
                    event_type,
                    visibility,
                    recurrence,
                    location,
                    attachments,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<CalendarEvent>, CalendarError> {
        let rows = sqlx::query_as!(CalendarEventModel, r#"SELECT * FROM calendar_events WHERE user_id = $1"#, user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        let mut events = Vec::new();
        for model in rows {
            // Deserialize complex fields
            let event_type: EventType = serde_json::from_str(&model.event_type)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let visibility: EventVisibility = serde_json::from_str(&model.visibility)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let recurrence: Option<RecurrenceRule> = model.recurrence
                .map(|r| serde_json::from_str(&r))
                .transpose()
                .map_err(|_| CalendarError::InvalidRecurrence("Failed to deserialize".to_string))?;
            let location: Option<Location> = model.location
                .map(|l| serde_json::from_str(&l))
                .transpose()
                .map_err(|_| CalendarError::InvalidLocation)?;
            let attachments: Vec<EventAttachment> = model.attachments
                .map(|a| serde_json::from_str(&a))
                .transpose()
                .map_err(|_| CalendarError::InvalidEventDuration)?
                .unwrap_or_default();

            events.push(CalendarEvent {
                id: model.id,
                user_id: model.user_id,
                title: model.title,
                description: model.description,
                start: model.start,
                end: model.end,
                event_type,
                visibility,
                recurrence,
                location,
                attachments,
                created_at: model.created_at,
                updated_at: model.updated_at,
            });
        }

        Ok(events)
    }

    async fn find_overlapping(&self, user_id: Uuid, slot: &TimeSlot) -> Result<Vec<CalendarEvent>, CalendarError> {
        let rows = sqlx::query_as!(CalendarEventModel, 
            r#"SELECT * FROM calendar_events 
               WHERE user_id = $1 
               AND start < $2 
               AND "end" > $3"#, 
            user_id, slot.end, slot.start)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        let mut events = Vec::new();
        for model in rows {
            // Deserialize complex fields
            let event_type: EventType = serde_json::from_str(&model.event_type)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let visibility: EventVisibility = serde_json::from_str(&model.visibility)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let recurrence: Option<RecurrenceRule> = model.recurrence
                .map(|r| serde_json::from_str(&r))
                .transpose()
                .map_err(|_| CalendarError::InvalidRecurrence("Failed to deserialize".to_string))?;
            let location: Option<Location> = model.location
                .map(|l| serde_json::from_str(&l))
                .transpose()
                .map_err(|_| CalendarError::InvalidLocation)?;
            let attachments: Vec<EventAttachment> = model.attachments
                .map(|a| serde_json::from_str(&a))
                .transpose()
                .map_err(|_| CalendarError::InvalidEventDuration)?
                .unwrap_or_default();

            events.push(CalendarEvent {
                id: model.id,
                user_id: model.user_id,
                title: model.title,
                description: model.description,
                start: model.start,
                end: model.end,
                event_type,
                visibility,
                recurrence,
                location,
                attachments,
                created_at: model.created_at,
                updated_at: model.updated_at,
            });
        }

        Ok(events)
    }

    async fn update(&self, event: &CalendarEvent) -> Result<(), CalendarError> {
        self.save(event).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), CalendarError> {
        sqlx::query!(r#"DELETE FROM calendar_events WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        Ok(())
    }
}

/// Implementation of ShiftRepository using PostgreSQL
pub struct ShiftRepositoryImpl {
    pool: PgPool,
}

impl ShiftRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ShiftRepository for ShiftRepositoryImpl {
    async fn save(&self, shift: &WorkShift) -> Result<(), CalendarError> {
        // Serialize complex fields
        let schedule = serde_json::to_string(&shift.schedule)
            .map_err(|_| CalendarError::InvalidEventDuration)?;
        let coverage = serde_json::to_string(&shift.coverage)
            .map_err(|_| CalendarError::InvalidEventDuration)?;

        sqlx::query!(
            r#"
            INSERT INTO work_shifts (
                id, cooperative_id, position, schedule, coverage, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                cooperative_id = $2, position = $3, schedule = $4, coverage = $5
            "#,
            shift.id,
            shift.cooperative_id,
            shift.position,
            schedule,
            coverage,
            shift.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CalendarError::InvalidEventDuration)?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkShift>, CalendarError> {
        let row = sqlx::query_as!(WorkShiftModel, r#"SELECT * FROM work_shifts WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        match row {
            Some(model) => {
                // Deserialize complex fields
                let schedule: crate::domain::ShiftSchedule = serde_json::from_str(&model.schedule)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;
                let coverage: crate::domain::ShiftCoverage = serde_json::from_str(&model.coverage)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;

                Ok(Some(WorkShift {
                    id: model.id,
                    cooperative_id: model.cooperative_id,
                    position: model.position,
                    schedule,
                    coverage,
                    created_at: model.created_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_cooperative_id(&self, cooperative_id: Uuid) -> Result<Vec<WorkShift>, CalendarError> {
        let rows = sqlx::query_as!(WorkShiftModel, r#"SELECT * FROM work_shifts WHERE cooperative_id = $1"#, cooperative_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        let mut shifts = Vec::new();
        for model in rows {
            // Deserialize complex fields
            let schedule: crate::domain::ShiftSchedule = serde_json::from_str(&model.schedule)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let coverage: crate::domain::ShiftCoverage = serde_json::from_str(&model.coverage)
                .map_err(|_| CalendarError::InvalidEventDuration)?;

            shifts.push(WorkShift {
                id: model.id,
                cooperative_id: model.cooperative_id,
                position: model.position,
                schedule,
                coverage,
                created_at: model.created_at,
            });
        }

        Ok(shifts)
    }

    async fn find_by_date_range(&self, cooperative_id: Uuid, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<WorkShift>, CalendarError> {
        // This is a simplified implementation that just finds shifts by cooperative_id
        // A more sophisticated implementation would filter by actual shift dates
        self.find_by_cooperative_id(cooperative_id).await
    }

    async fn update(&self, shift: &WorkShift) -> Result<(), CalendarError> {
        self.save(shift).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), CalendarError> {
        sqlx::query!(r#"DELETE FROM work_shifts WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        Ok(())
    }
}

/// Implementation of ReminderRepository using PostgreSQL
pub struct ReminderRepositoryImpl {
    pool: PgPool,
}

impl ReminderRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReminderRepository for ReminderRepositoryImpl {
    async fn save(&self, reminder: &EventReminder) -> Result<(), CalendarError> {
        // Serialize complex fields
        let method = serde_json::to_string(&reminder.method)
            .map_err(|_| CalendarError::InvalidEventDuration)?;
        let status = serde_json::to_string(&reminder.status)
            .map_err(|_| CalendarError::InvalidEventDuration)?;

        sqlx::query!(
            r#"
            INSERT INTO event_reminders (
                id, event_id, user_id, trigger_time, method, escalation_level, status, created_at, message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                event_id = $2, user_id = $3, trigger_time = $4, method = $5,
                escalation_level = $6, status = $7, message = $9
            "#,
            reminder.id,
            reminder.event_id,
            reminder.user_id,
            reminder.trigger_time,
            method,
            reminder.escalation_level as i32,
            status,
            reminder.created_at,
            reminder.message,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CalendarError::InvalidEventDuration)?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<EventReminder>, CalendarError> {
        let row = sqlx::query_as!(EventReminderModel, r#"SELECT * FROM event_reminders WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        match row {
            Some(model) => {
                // Deserialize complex fields
                let method: crate::domain::ReminderMethod = serde_json::from_str(&model.method)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;
                let status: crate::domain::ReminderStatus = serde_json::from_str(&model.status)
                    .map_err(|_| CalendarError::InvalidEventDuration)?;

                Ok(Some(EventReminder {
                    id: model.id,
                    event_id: model.event_id,
                    user_id: model.user_id,
                    trigger_time: model.trigger_time,
                    method,
                    escalation_level: model.escalation_level as u8,
                    status,
                    created_at: model.created_at,
                    message: model.message,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_due_reminders(&self, current_time: DateTime<Utc>) -> Result<Vec<EventReminder>, CalendarError> {
        let rows = sqlx::query_as!(EventReminderModel, 
            r#"SELECT * FROM event_reminders 
               WHERE trigger_time <= $1 
               AND status = '"Pending"'"#, 
            current_time)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        let mut reminders = Vec::new();
        for model in rows {
            // Deserialize complex fields
            let method: crate::domain::ReminderMethod = serde_json::from_str(&model.method)
                .map_err(|_| CalendarError::InvalidEventDuration)?;
            let status: crate::domain::ReminderStatus = serde_json::from_str(&model.status)
                .map_err(|_| CalendarError::InvalidEventDuration)?;

            reminders.push(EventReminder {
                id: model.id,
                event_id: model.event_id,
                user_id: model.user_id,
                trigger_time: model.trigger_time,
                method,
                escalation_level: model.escalation_level as u8,
                status,
                created_at: model.created_at,
                message: model.message,
            });
        }

        Ok(reminders)
    }

    async fn update_status(&self, id: Uuid, status: crate::domain::ReminderStatus) -> Result<(), CalendarError> {
        let status_str = serde_json::to_string(&status)
            .map_err(|_| CalendarError::InvalidEventDuration)?;

        sqlx::query!(r#"UPDATE event_reminders SET status = $1 WHERE id = $2"#, status_str, id)
            .execute(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), CalendarError> {
        sqlx::query!(r#"DELETE FROM event_reminders WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

        Ok(())
    }
}

/// Implementation of ParticipantService
pub struct ParticipantServiceImpl {
    pool: PgPool,
}

impl ParticipantServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ParticipantService for ParticipantServiceImpl {
    async fn check_availability(
        &self,
        participants: &[Participant],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<crate::domain::SchedulingConflict>, CalendarError> {
        let mut conflicts = Vec::new();
        
        for participant in participants {
            // Check for overlapping events for each participant
            let overlapping_events = sqlx::query!(
                r#"SELECT id, start, "end" FROM calendar_events 
                   WHERE user_id = $1 
                   AND start < $2 
                   AND "end" > $3"#,
                participant.user_id,
                end,
                start
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CalendarError::EventNotFound)?;

            for row in overlapping_events {
                conflicts.push(crate::domain::SchedulingConflict::new(
                    row.id,
                    row.start.max(start),
                    row.end.min(end),
                    vec![participant.user_id],
                ));
            }
        }
        
        Ok(conflicts)
    }
    
    async fn notify_participants(
        &self,
        event: &CalendarEvent,
        participants: &[Participant],
    ) -> Result<(), CalendarError> {
        // In a real implementation, we would send notifications to participants
        // For now, we'll just return Ok
        Ok(())
    }
}

/// Implementation of P2PManager
pub struct P2PSyncManagerImpl {
    // In a real implementation, this would contain p2panda client
}

impl P2PSyncManagerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl P2PManager for P2PSyncManagerImpl {
    async fn share_event(
        &self,
        event: &CalendarEvent,
        participants: &[Participant],
    ) -> Result<(), CalendarError> {
        // In a real implementation, we would share the event through p2panda network
        // For now, we'll just return Ok
        Ok(())
    }
}
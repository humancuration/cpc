//! Integration tests for the calendar module

#[cfg(test)]
mod tests {
    use cpc_core_calendar::{
        domain::{CalendarEvent, EventType, EventVisibility, CalendarError},
        application::{SchedulingService, CreateEventInput},
        domain::participant::{Participant, ParticipantRole},
    };
    use chrono::{Utc, Duration};
    use uuid::Uuid;

    // Mock implementations for testing
    use async_trait::async_trait;
    use std::sync::Arc;

    struct MockEventRepository;
    struct MockP2PManager;
    struct MockParticipantService;

    #[async_trait]
    impl cpc_core_calendar::application::EventRepository for MockEventRepository {
        async fn save(&self, _event: &CalendarEvent) -> Result<(), CalendarError> {
            Ok(())
        }
        
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<CalendarEvent>, CalendarError> {
            Ok(None)
        }
        
        async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<CalendarEvent>, CalendarError> {
            Ok(Vec::new())
        }
        
        async fn find_overlapping(&self, _user_id: Uuid, _slot: &cpc_core_calendar::domain::TimeSlot) -> Result<Vec<CalendarEvent>, CalendarError> {
            Ok(Vec::new())
        }
        
        async fn update(&self, _event: &CalendarEvent) -> Result<(), CalendarError> {
            Ok(())
        }
        
        async fn delete(&self, _id: Uuid) -> Result<(), CalendarError> {
            Ok(())
        }
    }

    #[async_trait]
    impl cpc_core_calendar::application::P2PManager for MockP2PManager {
        async fn share_event(
            &self,
            _event: &CalendarEvent,
            _participants: &[Participant],
        ) -> Result<(), CalendarError> {
            Ok(())
        }
    }

    #[async_trait]
    impl cpc_core_calendar::application::ParticipantService for MockParticipantService {
        async fn check_availability(
            &self,
            _participants: &[Participant],
            _start: chrono::DateTime<Utc>,
            _end: chrono::DateTime<Utc>,
        ) -> Result<Vec<cpc_core_calendar::domain::SchedulingConflict>, CalendarError> {
            Ok(Vec::new())
        }
        
        async fn notify_participants(
            &self,
            _event: &CalendarEvent,
            _participants: &[Participant],
        ) -> Result<(), CalendarError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_event() {
        let repository = Arc::new(MockEventRepository);
        let p2p_manager = Arc::new(MockP2PManager);
        let participant_service = Arc::new(MockParticipantService);
        
        let scheduling_service = SchedulingService::new(
            repository,
            p2p_manager,
            participant_service,
        );
        
        let user_id = Uuid::new_v4();
        let start = Utc::now();
        let end = start + Duration::hours(1);
        
        let input = CreateEventInput {
            title: "Test Event".to_string(),
            description: Some("Test Description".to_string()),
            start,
            end,
            event_type: EventType::Personal,
            visibility: EventVisibility::Private,
            recurrence: None,
            location: None,
            participants: vec![],
        };
        
        let result = scheduling_service.create_event(user_id, input).await;
        assert!(result.is_ok());
        
        let event = result.unwrap();
        assert_eq!(event.user_id, user_id);
        assert_eq!(event.title, "Test Event");
        assert_eq!(event.description, Some("Test Description".to_string()));
    }
}
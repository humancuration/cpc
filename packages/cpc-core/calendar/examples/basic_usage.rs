//! Basic usage example of the calendar module

use cpc_core_calendar::{
    domain::{CalendarEvent, EventType, EventVisibility},
    application::{SchedulingService, CreateEventInput},
    domain::participant::{Participant, ParticipantRole},
    infrastructure::{
        EventRepositoryImpl, P2PSyncManager,
        database::repositories::{ParticipantServiceImpl, ReminderRepositoryImpl},
    },
};
use chrono::{Utc, Duration};
use uuid::Uuid;
use sqlx::PgPool;

/// Example of creating a calendar event
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In a real application, you would connect to a database
    // let pool = PgPool::connect("postgresql://user:password@localhost/calendar").await?;
    
    // For this example, we'll use mock implementations
    println!("Creating a calendar event...");
    
    let user_id = Uuid::new_v4();
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    // Create participants
    let participant = Participant::new(Uuid::new_v4(), ParticipantRole::Required);
    let participants = vec![participant];
    
    // Create event input
    let input = CreateEventInput {
        title: "Team Meeting".to_string(),
        description: Some("Weekly team sync".to_string()),
        start,
        end,
        event_type: EventType::Business,
        visibility: EventVisibility::Shared(vec![user_id]),
        recurrence: None,
        location: None,
        participants,
    };
    
    println!("Event created: {}", input.title);
    println!("Start: {}", input.start);
    println!("End: {}", input.end);
    println!("Type: {:?}", input.event_type);
    
    // In a real application, you would use the actual services:
    // let repository = Arc::new(EventRepositoryImpl::new(pool));
    // let p2p_manager = Arc::new(P2PSyncManager::new());
    // let participant_service = Arc::new(ParticipantServiceImpl::new(pool));
    // 
    // let scheduling_service = SchedulingService::new(
    //     repository,
    //     p2p_manager,
    //     participant_service,
    // );
    // 
    // let event = scheduling_service.create_event(user_id, input).await?;
    // println!("Event created with ID: {}", event.id);
    
    Ok(())
}
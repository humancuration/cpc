use crate::KarmaEvent;
use rust_decimal::Decimal;
use uuid::Uuid;

pub fn create_karma_event(
    user_id: Uuid,
    event_type: String,
    points: Decimal,
    description: String,
) -> KarmaEvent {
    KarmaEvent {
        id: Uuid::new_v4(),
        user_id,
        event_type,
        points,
        description,
        created_at: chrono::Utc::now(),
    }
}

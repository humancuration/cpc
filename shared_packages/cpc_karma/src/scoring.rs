use rust_decimal::Decimal;

// Karma scoring constants
pub const HELPFUL_COMMENT_POINTS: Decimal = Decimal::from_parts(5, 0, 0, false, 0);
pub const QUALITY_POST_POINTS: Decimal = Decimal::from_parts(10, 0, 0, false, 0);
pub const VOLUNTEER_ACTIVITY_POINTS: Decimal = Decimal::from_parts(20, 0, 0, false, 0);
pub const NEGATIVE_BEHAVIOR_PENALTY: Decimal = Decimal::from_parts(5, 0, 0, true, 0);

pub fn calculate_karma_for_event(event_type: &str) -> Decimal {
    match event_type {
        "helpful_comment" => HELPFUL_COMMENT_POINTS,
        "quality_post" => QUALITY_POST_POINTS,
        "volunteer_activity" => VOLUNTEER_ACTIVITY_POINTS,
        "negative_behavior" => NEGATIVE_BEHAVIOR_PENALTY,
        _ => Decimal::ZERO,
    }
}

//! Calendar view types
use serde::{Deserialize, Serialize};

/// Calendar view types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalendarView {
    Month,
    Week,
    Day,
    Shift,
}
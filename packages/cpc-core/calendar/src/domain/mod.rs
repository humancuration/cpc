//! Domain models for the calendar module
//!
//! This module contains all the core business logic and entities for the calendar system.

pub mod event;
pub mod participant;
pub mod shift;
pub mod reminder;
pub mod primitives;

pub use event::{CalendarEvent, EventType, EventVisibility};
pub use shift::{WorkShift, ShiftSchedule, RotationPattern, ShiftCoverage};
pub use reminder::{EventReminder, ReminderMethod, ReminderEscalation, EscalationStep};
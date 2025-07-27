//! Calendar module for the CPC platform
//!
//! This module provides comprehensive calendar functionality including:
//! - Personal and business event management
//! - Work shift scheduling for cooperatives
//! - Smart reminder system with escalation
//! - p2p synchronization
//! - Integration with task manager and other modules

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for convenience
pub use domain::{CalendarEvent, WorkShift, EventReminder, CalendarError};
pub use application::{SchedulingService, ShiftManagementService, ReminderService};
pub use infrastructure::{EventRepositoryImpl, ShiftRepositoryImpl, ReminderRepositoryImpl, P2PSyncManager};
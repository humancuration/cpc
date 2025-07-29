//! Application services for the calendar module

pub mod scheduling_service;
pub mod shift_management;
pub mod reminder_service;
pub mod calendar_integration;
pub mod consent;

pub use scheduling_service::SchedulingService;
pub use shift_management::ShiftManagementService;
pub use reminder_service::ReminderService;
pub use calendar_integration::CalendarIntegrationService;
pub use consent::ConsentService;
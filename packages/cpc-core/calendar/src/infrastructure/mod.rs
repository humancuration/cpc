//! Infrastructure implementations for the calendar module

pub mod database;
pub mod sync;

pub use database::repositories::{EventRepositoryImpl, ShiftRepositoryImpl, ReminderRepositoryImpl};
pub use sync::p2p_sync::P2PSyncManager;
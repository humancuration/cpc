//! Notification infrastructure for the invoicing module
//!
//! This module contains the implementations for sending payment reminders through various channels.

pub mod email;
pub mod sms;
pub mod p2p;
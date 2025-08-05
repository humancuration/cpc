//! Real-time signaling service for collaborative applications
//!
//! This crate provides WebSocket-based real-time signaling for collaborative applications,
//! including presence tracking, cursor positioning, and selection sharing.

pub mod signaling;
pub mod client;
pub mod server;
pub mod message;
pub mod redis_signaling;

pub use signaling::SignalingService;
pub use client::SignalingClient;
pub use server::SignalingServer;
pub use redis_signaling::RedisSignalingService;
pub use message::{SignalingMessage, PresenceUpdate, PresenceSummary, PresenceUser, PresenceStatus, CursorPosition, SelectionRange, Position};
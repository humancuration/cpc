//! Presentation layer for the finance module

#[cfg(feature = "visualization")]
pub mod bevy;

#[cfg(feature = "web")]
pub mod yew;
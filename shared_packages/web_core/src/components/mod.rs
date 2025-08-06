//! UI components for CPC web applications
//!
//! This module provides reusable UI components that can be used
//! across all web applications in the CPC ecosystem.

pub mod base;
pub mod button;
pub mod checkbox;
pub mod error_boundary;
pub mod form;
pub mod modal;
pub mod radio_button;
pub mod select;
pub mod text_area;
pub mod text_input;

// Re-export components for convenience
pub use base::{BaseComponent, CommonProps, ComponentSize, ComponentTheme};
pub use button::Button;
pub use checkbox::Checkbox;
pub use error_boundary::ErrorBoundary;
pub use form::{Form, FormSubmitEvent, FormData};
pub use modal::Modal;
pub use radio_button::RadioButton;
pub use select::{Select, SelectOption};
pub use text_area::{TextArea, TextAreaResize};
pub use text_input::TextInput;
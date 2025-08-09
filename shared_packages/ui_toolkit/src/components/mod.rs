//! UI components for the UI toolkit
//!
//! This module provides reusable UI components that can be used
//! across all applications in the CPC ecosystem.

pub mod base;
pub mod button;
pub mod card;
pub mod container;
pub mod input;
pub mod theme_provider;

// Re-export components for convenience
pub use base::{BaseComponent, CommonProps, ComponentSize, ComponentTheme};
pub use button::{Button, ButtonProps, ButtonVariant, ButtonSize};
pub use card::{Card, CardProps, CardSize};
pub use container::{Container, ContainerProps, ContainerMaxWidth, ContainerPadding};
pub use input::{Input, InputProps, InputType};
pub use theme_provider::{ThemeProvider, ThemeProviderProps, ThemeContext};
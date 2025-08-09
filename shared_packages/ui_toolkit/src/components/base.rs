//! Base component trait for UI toolkit components
//!
//! This module defines the base component trait that all UI components
//! in the UI toolkit should implement.

use yew::prelude::*;

/// Base trait for all UI toolkit components
///
/// This trait defines the common interface that all UI components
/// should implement to ensure consistency across the toolkit.
pub trait BaseComponent: Sized {
    /// Properties for the component
    type Properties: Properties;

    /// Create a new instance of the component
    fn create(props: &Self::Properties) -> Self;

    /// Update the component state based on new properties
    fn update_props(&mut self, props: Self::Properties);

    /// Render the component as HTML
    fn view(&self) -> Html;
}

/// Common properties that all components should support
#[derive(Properties, PartialEq, Clone)]
pub struct CommonProps {
    /// Additional CSS classes to apply
    #[prop_or_default]
    pub class: Option<String>,

    /// Unique identifier for the component
    #[prop_or_default]
    pub id: Option<String>,

    /// Whether the component is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Custom styling
    #[prop_or_default]
    pub style: Option<String>,
}

impl Default for CommonProps {
    fn default() -> Self {
        Self {
            class: None,
            id: None,
            disabled: false,
            style: None,
        }
    }
}

/// Size variants for components
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentSize {
    /// Small size variant
    Small,
    
    /// Medium size variant
    Medium,
    
    /// Large size variant
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Theme variants for components
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentTheme {
    /// Primary theme variant
    Primary,
    
    /// Secondary theme variant
    Secondary,
    
    /// Success theme variant
    Success,
    
    /// Warning theme variant
    Warning,
    
    /// Danger theme variant
    Danger,
    
    /// Info theme variant
    Info,
    
    /// Light theme variant
    Light,
    
    /// Dark theme variant
    Dark,
    
    /// Text theme variant (no background)
    Text,
}

impl Default for ComponentTheme {
    fn default() -> Self {
        Self::Primary
    }
}
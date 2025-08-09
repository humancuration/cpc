//! Reusable button component with theming
//!
//! This module provides a flexible button component that can be
//! styled and used throughout applications.

use crate::components::base::{BaseComponent, CommonProps, ComponentTheme};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Button component
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The text to display on the button
    #[prop_or_default]
    pub children: Children,
    
    /// Callback when the button is clicked
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    
    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,
    
    /// The variant of the button
    #[prop_or_default]
    pub variant: ButtonVariant,
    
    /// The size of the button
    #[prop_or_default]
    pub size: ButtonSize,
}

/// Button variant styling options
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonVariant {
    /// Filled button with background color
    Contained,
    /// Outlined button with border
    Outlined,
    /// Text-only button
    Text,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Contained
    }
}

/// Button size options
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonSize {
    /// Small button
    Small,
    /// Medium button (default)
    Medium,
    /// Large button
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// A reusable button component with theming
#[styled_component(Button)]
pub struct Button {
    props: ButtonProps,
}

impl BaseComponent for Button {
    type Properties = ButtonProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let variant_style = match (&self.props.variant, &self.props.size) {
            (ButtonVariant::Contained, ButtonSize::Small) => style!(
                r#"
                background-color: var(--cpc-primary);
                color: var(--cpc-white);
                border: 1px solid var(--cpc-primary);
                padding: var(--cpc-spacing-sm) var(--cpc-spacing-md);
                font-size: var(--cpc-font-size-sm);
            "#
            ),
            (ButtonVariant::Contained, ButtonSize::Medium) => style!(
                r#"
                background-color: var(--cpc-primary);
                color: var(--cpc-white);
                border: 1px solid var(--cpc-primary);
                padding: calc(var(--cpc-spacing-sm) * 1.5) var(--cpc-spacing-lg);
                font-size: var(--cpc-font-size-md);
            "#
            ),
            (ButtonVariant::Contained, ButtonSize::Large) => style!(
                r#"
                background-color: var(--cpc-primary);
                color: var(--cpc-white);
                border: 1px solid var(--cpc-primary);
                padding: var(--cpc-spacing-md) var(--cpc-spacing-xl);
                font-size: var(--cpc-font-size-lg);
            "#
            ),
            (ButtonVariant::Outlined, ButtonSize::Small) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: 1px solid var(--cpc-primary);
                padding: var(--cpc-spacing-sm) var(--cpc-spacing-md);
                font-size: var(--cpc-font-size-sm);
            "#
            ),
            (ButtonVariant::Outlined, ButtonSize::Medium) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: 1px solid var(--cpc-primary);
                padding: calc(var(--cpc-spacing-sm) * 1.5) var(--cpc-spacing-lg);
                font-size: var(--cpc-font-size-md);
            "#
            ),
            (ButtonVariant::Outlined, ButtonSize::Large) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: 1px solid var(--cpc-primary);
                padding: var(--cpc-spacing-md) var(--cpc-spacing-xl);
                font-size: var(--cpc-font-size-lg);
            "#
            ),
            (ButtonVariant::Text, ButtonSize::Small) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: none;
                padding: var(--cpc-spacing-sm) var(--cpc-spacing-md);
                font-size: var(--cpc-font-size-sm);
            "#
            ),
            (ButtonVariant::Text, ButtonSize::Medium) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: none;
                padding: calc(var(--cpc-spacing-sm) * 1.5) var(--cpc-spacing-lg);
                font-size: var(--cpc-font-size-md);
            "#
            ),
            (ButtonVariant::Text, ButtonSize::Large) => style!(
                r#"
                background-color: transparent;
                color: var(--cpc-primary);
                border: none;
                padding: var(--cpc-spacing-md) var(--cpc-spacing-xl);
                font-size: var(--cpc-font-size-lg);
            "#
            ),
        };
        
        let base_style = style!(
            r#"
            display: inline-block;
            font-weight: var(--cpc-font-weight-medium);
            text-align: center;
            white-space: nowrap;
            vertical-align: middle;
            user-select: none;
            line-height: 1.5;
            border-radius: var(--cpc-border-radius-md);
            transition: all 0.15s ease-in-out;
            cursor: pointer;
            text-decoration: none;
            
            &:disabled {
                opacity: 0.65;
                cursor: not-allowed;
            }
            
            &:hover:not(:disabled) {
                opacity: 0.8;
                transform: translateY(-1px);
            }
            
            &:focus {
                outline: 2px solid var(--cpc-primary);
                outline-offset: 2px;
            }
        "#
        );
        
        let classes = classes!(
            base_style.get_class_name(),
            variant_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        html! {
            <button
                id={self.props.common.id.clone()}
                class={classes}
                onclick={self.props.onclick.clone()}
                disabled={self.props.disabled}
                style={self.props.common.style.clone()}
            >
                { for self.props.children.iter() }
            </button>
        }
    }
}

impl Button {
    /// Create a new button component
    pub fn new(props: ButtonProps) -> Self {
        Self::create(&props)
    }
}
//! Reusable button component with theming
//!
//! This module provides a flexible button component that can be
//! styled and used throughout CPC web applications.

use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Button component
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
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
    
    /// Additional CSS classes to apply
    #[prop_or_default]
    pub class: Option<String>,
}

/// Button variant styling options
#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Text,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// A reusable button component with theming
#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_style = match props.variant {
        ButtonVariant::Primary => style!(
            r#"
            background-color: #007bff;
            color: white;
            border: 1px solid #007bff;
        "#
        ),
        ButtonVariant::Secondary => style!(
            r#"
            background-color: #6c757d;
            color: white;
            border: 1px solid #6c757d;
        "#
        ),
        ButtonVariant::Danger => style!(
            r#"
            background-color: #dc3545;
            color: white;
            border: 1px solid #dc3545;
        "#
        ),
        ButtonVariant::Text => style!(
            r#"
            background-color: transparent;
            color: #007bff;
            border: none;
        "#
        ),
    };
    
    let base_style = style!(
        r#"
        display: inline-block;
        font-weight: 400;
        text-align: center;
        white-space: nowrap;
        vertical-align: middle;
        user-select: none;
        padding: 0.375rem 0.75rem;
        font-size: 1rem;
        line-height: 1.5;
        border-radius: 0.25rem;
        transition: color 0.15s ease-in-out, background-color 0.15s ease-in-out, border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
        cursor: pointer;
        
        &:disabled {
            opacity: 0.65;
            cursor: not-allowed;
        }
        
        &:hover:not(:disabled) {
            opacity: 0.8;
        }
    "#
    );
    
    let classes = classes!(
        base_style.get_class_name(),
        variant_style.get_class_name(),
        props.class.clone()
    );
    
    html! {
        <button
            class={classes}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </button>
    }
}
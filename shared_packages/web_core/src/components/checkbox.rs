//! Reusable checkbox component
//!
//! This module provides a flexible checkbox component that can be
//! styled and used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Checkbox component
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Whether the checkbox is checked
    pub checked: bool,
    
    /// Callback when the checkbox state changes
    #[prop_or_default]
    pub onchange: Callback<bool>,
    
    /// Label text for the checkbox
    #[prop_or_default]
    pub label: String,
    
    /// Whether the checkbox is disabled
    #[prop_or_default]
    pub disabled: bool,
    
    /// Whether the checkbox is required
    #[prop_or_default]
    pub required: bool,
}

/// A reusable checkbox component
#[styled_component(Checkbox)]
pub struct Checkbox {
    props: CheckboxProps,
}

impl BaseComponent for Checkbox {
    type Properties = CheckboxProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let container_style = style!(
            r#"
            display: flex;
            align-items: center;
            margin-bottom: 0.5rem;
        "#
        );
        
        let checkbox_style = style!(
            r#"
            width: 1rem;
            height: 1rem;
            margin-right: 0.5rem;
            accent-color: #007bff;
            
            &:disabled {
                opacity: 0.65;
            }
        "#
        );
        
        let label_style = style!(
            r#"
            margin-bottom: 0;
            cursor: pointer;
            
            &:disabled {
                cursor: not-allowed;
            }
        "#
        );
        
        let container_classes = classes!(
            container_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        let on_change = {
            let onchange = self.props.onchange.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                onchange.emit(input.checked());
            })
        };
        
        html! {
            <div class={container_classes}>
                <input
                    type="checkbox"
                    class={checkbox_style.get_class_name()}
                    checked={self.props.checked}
                    disabled={self.props.disabled}
                    required={self.props.required}
                    onchange={on_change}
                />
                if !self.props.label.is_empty() {
                    <label 
                        class={label_style.get_class_name()}
                        disabled={self.props.disabled}
                    >
                        { &self.props.label }
                    </label>
                }
            </div>
        }
    }
}

impl Checkbox {
    /// Create a new checkbox component
    pub fn new(props: CheckboxProps) -> Self {
        Self::create(&props)
    }
    
    /// Check if the checkbox is currently checked
    pub fn is_checked(&self) -> bool {
        self.props.checked
    }
}
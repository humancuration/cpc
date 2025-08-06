//! Reusable radio button component
//!
//! This module provides a flexible radio button component that can be
//! styled and used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the RadioButton component
#[derive(Properties, PartialEq, Clone)]
pub struct RadioButtonProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Whether the radio button is selected
    #[prop_or_default]
    pub checked: bool,
    
    /// Callback when the radio button state changes
    #[prop_or_default]
    pub onchange: Callback<bool>,
    
    /// Value of the radio button
    pub value: String,
    
    /// Name of the radio group
    pub name: String,
    
    /// Label text for the radio button
    #[prop_or_default]
    pub label: String,
    
    /// Whether the radio button is disabled
    #[prop_or_default]
    pub disabled: bool,
}

/// A reusable radio button component
#[styled_component(RadioButton)]
pub struct RadioButton {
    props: RadioButtonProps,
}

impl BaseComponent for RadioButton {
    type Properties = RadioButtonProps;
    
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
        
        let radio_style = style!(
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
                    type="radio"
                    class={radio_style.get_class_name()}
                    checked={self.props.checked}
                    disabled={self.props.disabled}
                    name={self.props.name.clone()}
                    value={self.props.value.clone()}
                    onchange={on_change}
                    aria-checked={self.props.checked.to_string()}
                    aria-disabled={self.props.disabled.to_string()}
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

impl RadioButton {
    /// Create a new radio button component
    pub fn new(props: RadioButtonProps) -> Self {
        Self::create(&props)
    }
    
    /// Check if the radio button is currently checked
    pub fn is_checked(&self) -> bool {
        self.props.checked
    }
}
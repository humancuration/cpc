//! Reusable input component with theming
//!
//! This module provides a flexible input component that can be
//! styled and used throughout applications.

use crate::components::base::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Input component
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The current value of the input
    #[prop_or_default]
    pub value: String,
    
    /// Callback when the input value changes
    #[prop_or_default]
    pub onchange: Callback<String>,
    
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: String,
    
    /// Input type (text, password, email, etc.)
    #[prop_or_default]
    pub input_type: InputType,
    
    /// Whether the input is disabled
    #[prop_or_default]
    pub disabled: bool,
    
    /// Whether the input is required
    #[prop_or_default]
    pub required: bool,
    
    /// Maximum length of the input
    #[prop_or_default]
    pub max_length: Option<u32>,
    
    /// Minimum length of the input
    #[prop_or_default]
    pub min_length: Option<u32>,
    
    /// Name attribute for the input
    #[prop_or_default]
    pub name: Option<String>,
}

/// Input type variants
#[derive(PartialEq, Clone, Debug)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Search,
    Url,
    Tel,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Text => "text".to_string(),
            InputType::Password => "password".to_string(),
            InputType::Email => "email".to_string(),
            InputType::Number => "number".to_string(),
            InputType::Search => "search".to_string(),
            InputType::Url => "url".to_string(),
            InputType::Tel => "tel".to_string(),
        }
    }
}

/// A reusable input component
#[styled_component(Input)]
pub struct Input {
    props: InputProps,
}

impl BaseComponent for Input {
    type Properties = InputProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let base_style = style!(
            r#"
            display: block;
            width: 100%;
            padding: calc(var(--cpc-spacing-sm) * 1.5) var(--cpc-spacing-lg);
            font-size: var(--cpc-font-size-md);
            line-height: 1.5;
            color: var(--cpc-text);
            background-color: var(--cpc-white);
            background-clip: padding-box;
            border: 1px solid var(--cpc-gray-300);
            border-radius: var(--cpc-border-radius-md);
            transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
            
            &:focus {
                color: var(--cpc-text);
                background-color: var(--cpc-white);
                border-color: var(--cpc-primary);
                outline: 0;
                box-shadow: 0 0 0 0.2rem rgba(67, 97, 238, 0.25);
            }
            
            &:disabled {
                background-color: var(--cpc-gray-100);
                opacity: 1;
            }
            
            &::placeholder {
                color: var(--cpc-gray-500);
                opacity: 1;
            }
        "#
        );
        
        let classes = classes!(
            base_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        let on_change = {
            let onchange = self.props.onchange.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                onchange.emit(input.value());
            })
        };
        
        html! {
            <input
                id={self.props.common.id.clone()}
                type={self.props.input_type.to_string()}
                class={classes}
                value={self.props.value.clone()}
                placeholder={self.props.placeholder.clone()}
                disabled={self.props.disabled}
                required={self.props.required}
                maxlength={self.props.max_length.map(|l| l as i32)}
                minlength={self.props.min_length.map(|l| l as i32)}
                name={self.props.name.clone()}
                onchange={on_change}
                style={self.props.common.style.clone()}
            />
        }
    }
}

impl Input {
    /// Create a new input component
    pub fn new(props: InputProps) -> Self {
        Self::create(&props)
    }
    
    /// Get the current value of the input
    pub fn value(&self) -> &str {
        &self.props.value
    }
}
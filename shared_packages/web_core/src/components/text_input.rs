//! Reusable text input component
//!
//! This module provides a flexible text input component that can be
//! styled and used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the TextInput component
#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps {
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

/// A reusable text input component
#[styled_component(TextInput)]
pub struct TextInput {
    props: TextInputProps,
}

impl BaseComponent for TextInput {
    type Properties = TextInputProps;
    
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
            padding: 0.375rem 0.75rem;
            font-size: 1rem;
            line-height: 1.5;
            color: #495057;
            background-color: #fff;
            background-clip: padding-box;
            border: 1px solid #ced4da;
            border-radius: 0.25rem;
            transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
            
            &:focus {
                color: #495057;
                background-color: #fff;
                border-color: #80bdff;
                outline: 0;
                box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
            }
            
            &:disabled {
                background-color: #e9ecef;
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
                type={self.props.input_type.to_string()}
                class={classes}
                value={self.props.value.clone()}
                placeholder={self.props.placeholder.clone()}
                disabled={self.props.disabled}
                required={self.props.required}
                maxlength={self.props.max_length.map(|l| l as i32)}
                onchange={on_change}
            />
        }
    }
}

impl TextInput {
    /// Create a new text input component
    pub fn new(props: TextInputProps) -> Self {
        Self::create(&props)
    }
    
    /// Get the current value of the input
    pub fn value(&self) -> &str {
        &self.props.value
    }
}
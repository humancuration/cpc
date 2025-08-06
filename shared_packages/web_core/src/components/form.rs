//! Reusable form component
//!
//! This module provides a flexible form component that can be
//! styled and used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};
use web_sys::MouseEvent;

/// Form submission event
#[derive(Debug, Clone)]
pub struct FormSubmitEvent {
    /// The form data
    pub data: FormData,
    
    /// The original event
    pub event: web_sys::Event,
}

/// Form data
#[derive(Debug, Clone, Default)]
pub struct FormData {
    /// Key-value pairs of form data
    pub fields: std::collections::HashMap<String, String>,
}

/// Properties for the Form component
#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The form content
    #[prop_or_default]
    pub children: Children,
    
    /// Callback when the form is submitted
    #[prop_or_default]
    pub onsubmit: Callback<FormSubmitEvent>,
    
    /// Callback when the form is reset
    #[prop_or_default]
    pub onreset: Callback<MouseEvent>,
    
    /// Whether to skip validation
    #[prop_or_default]
    pub novalidate: bool,
}

/// Context for form state management
#[derive(Clone, Debug, PartialEq)]
pub struct FormContext {
    /// Callback to register form fields
    pub register_field: Callback<(String, String)>,
    
    /// Callback to unregister form fields
    pub unregister_field: Callback<String>,
    
    /// Current form data
    pub data: std::collections::HashMap<String, String>,
}

/// A reusable form component
#[styled_component(Form)]
pub struct Form {
    props: FormProps,
    form_data: std::collections::HashMap<String, String>,
}

impl BaseComponent for Form {
    type Properties = FormProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { 
            props: props.clone(),
            form_data: std::collections::HashMap::new(),
        }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let base_style = style!(
            r#"
            display: block;
            margin: 0;
            padding: 0;
        "#
        );
        
        let classes = classes!(
            base_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        let on_submit = {
            let onsubmit = self.props.onsubmit.clone();
            let form_data = self.form_data.clone();
            Callback::from(move |e: web_sys::Event| {
                e.prevent_default();
                let event = FormSubmitEvent {
                    data: FormData {
                        fields: form_data.clone(),
                    },
                    event: e,
                };
                onsubmit.emit(event);
            })
        };
        
        let on_reset = {
            let onreset = self.props.onreset.clone();
            Callback::from(move |e: MouseEvent| {
                onreset.emit(e);
            })
        };
        
        // Create context callbacks
        // Note: In a real implementation, we would need a more sophisticated approach
        // to handle form field registration and state management
        
        html! {
            <form
                class={classes}
                onsubmit={on_submit}
                onreset={on_reset}
                novalidate={self.props.novalidate}
            >
                { for self.props.children.iter() }
            </form>
        }
    }
}

impl Form {
    /// Create a new form component
    pub fn new(props: FormProps) -> Self {
        Self::create(&props)
    }
    
    /// Get the current form data
    pub fn data(&self) -> &std::collections::HashMap<String, String> {
        &self.form_data
    }
}
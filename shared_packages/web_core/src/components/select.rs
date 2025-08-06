//! Reusable select component
//!
//! This module provides a flexible select component that can be
//! styled and used throughout CPC web applications.
//!
//! ## Examples
//!
//! ```
//! use web_core::components::{Select, SelectOption};
//!
//! let options = vec![
//!     SelectOption { value: "option1".to_string(), label: "Option 1".to_string(), disabled: false },
//!     SelectOption { value: "option2".to_string(), label: "Option 2".to_string(), disabled: false },
//! ];
//!
//! html! {
//!     <Select
//!         value="option1"
//!         options={options}
//!         onchange={|value: String| println!("Selected: {}", value)}
//!     />
//! }
//! ```
//!
//! ## Related Modules
//!
//! - [SelectProps]
//! - [SelectOption]

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Select component
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The current value of the select
    #[prop_or_default]
    pub value: String,
    
    /// Available options
    #[prop_or_default]
    pub options: Vec<SelectOption>,
    
    /// Callback when the select value changes
    #[prop_or_default]
    pub onchange: Callback<String>,
    
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<String>,
    
    /// Whether the select is disabled
    #[prop_or_default]
    pub disabled: bool,
    
    /// Whether multiple selection is allowed
    #[prop_or_default]
    pub multiple: bool,
}

/// Option for the Select component
#[derive(PartialEq, Clone, Debug)]
pub struct SelectOption {
    /// The value of the option
    pub value: String,
    
    /// The label to display for the option
    pub label: String,
    
    /// Whether the option is disabled
    #[prop_or_default]
    pub disabled: bool,
}

/// A reusable select component
#[styled_component(Select)]
pub struct Select {
    props: SelectProps,
}

impl BaseComponent for Select {
    type Properties = SelectProps;
    
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
            padding: 0.375rem 2.25rem 0.375rem 0.75rem;
            font-size: 1rem;
            line-height: 1.5;
            color: #495057;
            background-color: #fff;
            background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3e%3cpath fill='none' stroke='%23343a40' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='m2 5 6 6 6-6'/%3e%3c/svg%3e");
            background-repeat: no-repeat;
            background-position: right 0.75rem center;
            background-size: 16px 12px;
            border: 1px solid #ced4da;
            border-radius: 0.25rem;
            transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
            appearance: none;
            
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
            
            &[multiple] {
                height: auto;
                background-image: none;
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
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                onchange.emit(select.value());
            })
        };
        
        html! {
            <select
                class={classes}
                value={self.props.value.clone()}
                disabled={self.props.disabled}
                multiple={self.props.multiple}
                onchange={on_change}
                aria-disabled={self.props.disabled.to_string()}
            >
                if let Some(placeholder) = &self.props.placeholder {
                    <option value="" disabled=true selected={self.props.value.is_empty()}>
                        {placeholder}
                    </option>
                }
                { for self.props.options.iter().map(|option| {
                    html! {
                        <option 
                            value={option.value.clone()} 
                            disabled={option.disabled}
                            selected={option.value == self.props.value}
                        >
                            {&option.label}
                        </option>
                    }
                })}
            </select>
        }
    }
}

impl Select {
    /// Create a new select component
    pub fn new(props: SelectProps) -> Self {
        Self::create(&props)
    }
    
    /// Get the current value of the select
    pub fn value(&self) -> &str {
        &self.props.value
    }
}
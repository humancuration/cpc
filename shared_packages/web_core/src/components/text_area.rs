//! Reusable text area component
//!
//! This module provides a flexible text area component that can be
//! styled and used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Resize behavior for the TextArea component
#[derive(PartialEq, Clone, Debug)]
pub enum TextAreaResize {
    /// No resizing allowed
    None,
    
    /// Both horizontal and vertical resizing allowed
    Both,
    
    /// Only horizontal resizing allowed
    Horizontal,
    
    /// Only vertical resizing allowed
    Vertical,
}

impl Default for TextAreaResize {
    fn default() -> Self {
        Self::Vertical
    }
}

impl ToString for TextAreaResize {
    fn to_string(&self) -> String {
        match self {
            TextAreaResize::None => "none".to_string(),
            TextAreaResize::Both => "both".to_string(),
            TextAreaResize::Horizontal => "horizontal".to_string(),
            TextAreaResize::Vertical => "vertical".to_string(),
        }
    }
}

/// Properties for the TextArea component
#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The current value of the text area
    #[prop_or_default]
    pub value: String,
    
    /// Callback when the text area value changes
    #[prop_or_default]
    pub onchange: Callback<String>,
    
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: String,
    
    /// Whether the text area is disabled
    #[prop_or_default]
    pub disabled: bool,
    
    /// Whether the text area is read-only
    #[prop_or_default]
    pub readonly: bool,
    
    /// Number of visible text lines
    #[prop_or_default]
    pub rows: Option<u32>,
    
    /// Number of visible columns
    #[prop_or_default]
    pub cols: Option<u32>,
    
    /// Maximum number of characters
    #[prop_or_default]
    pub maxlength: Option<u32>,
    
    /// Resize behavior
    #[prop_or_default]
    pub resize: TextAreaResize,
}

/// A reusable text area component
#[styled_component(TextArea)]
pub struct TextArea {
    props: TextAreaProps,
}

impl BaseComponent for TextArea {
    type Properties = TextAreaProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let resize_value = self.props.resize.to_string();
        
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
            resize: ${resize_value};
            
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
            
            &[readonly] {
                background-color: #e9ecef;
            }
        "#,
            resize_value = resize_value
        );
        
        let classes = classes!(
            base_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        let on_change = {
            let onchange = self.props.onchange.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                onchange.emit(input.value());
            })
        };
        
        html! {
            <textarea
                class={classes}
                value={self.props.value.clone()}
                placeholder={self.props.placeholder.clone()}
                disabled={self.props.disabled}
                readonly={self.props.readonly}
                rows={self.props.rows.map(|r| r as i32)}
                cols={self.props.cols.map(|c| c as i32)}
                maxlength={self.props.maxlength.map(|l| l as i32)}
                onchange={on_change}
                aria-disabled={self.props.disabled.to_string()}
                aria-readonly={self.props.readonly.to_string()}
            />
        }
    }
}

impl TextArea {
    /// Create a new text area component
    pub fn new(props: TextAreaProps) -> Self {
        Self::create(&props)
    }
    
    /// Get the current value of the text area
    pub fn value(&self) -> &str {
        &self.props.value
    }
}
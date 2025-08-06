//! Reusable modal component
//!
//! This module provides a flexible modal component that can be
//! used throughout CPC web applications.

use crate::components::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Modal component
#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    /// Common properties
    ///
    /// These are common properties that all components support.
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Whether the modal is open
    pub open: bool,
    
    /// Callback when the modal is closed
    #[prop_or_default]
    pub onclose: Callback<()>,
    
    /// The title of the modal
    #[prop_or_default]
    pub title: String,
    
    /// The content to display in the modal
    #[prop_or_default]
    pub children: Children,
    
    /// Whether to show the close button
    #[prop_or_default]
    pub show_close_button: bool,
}

impl Default for ModalProps {
    fn default() -> Self {
        Self {
            common: CommonProps::default(),
            open: false,
            onclose: Callback::default(),
            title: String::default(),
            children: Children::default(),
            show_close_button: false,
        }
    }
}

/// A reusable modal component
#[styled_component(Modal)]
pub struct Modal {
    props: ModalProps,
}

impl BaseComponent for Modal {
    type Properties = ModalProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    fn view(&self) -> Html {
        let modal_style = style!(
            r#"
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.5);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 1000;
        "#
        );
        
        let modal_content_style = style!(
            r#"
            background-color: white;
            border-radius: 0.5rem;
            box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
            max-width: 500px;
            width: 90%;
            max-height: 90vh;
            overflow-y: auto;
        "#
        );
        
        let modal_header_style = style!(
            r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem 1.5rem;
            border-bottom: 1px solid #e9ecef;
        "#
        );
        
        let modal_title_style = style!(
            r#"
            margin: 0;
            font-size: 1.25rem;
            font-weight: 500;
        "#
        );
        
        let modal_close_button_style = style!(
            r#"
            background: none;
            border: none;
            font-size: 1.5rem;
            cursor: pointer;
            color: #6c757d;
            
            &:hover {
                color: #000;
            }
        "#
        );
        
        let modal_body_style = style!(
            r#"
            padding: 1.5rem;
        "#
        );
        
        let on_close = {
            let onclose = self.props.onclose.clone();
            Callback::from(move |_| onclose.emit(()))
        };
        
        if !self.props.open {
            return html! {};
        }
        
        html! {
            <div class={modal_style.get_class_name()}>
                <div class={modal_content_style.get_class_name()}>
                    <div class={modal_header_style.get_class_name()}>
                        <h2 class={modal_title_style.get_class_name()}>{ &self.props.title }</h2>
                        if self.props.show_close_button {
                            <button
                                class={modal_close_button_style.get_class_name()}
                                onclick={on_close.clone()}
                            >
                                {"×"}
                            </button>
                        }
                    </div>
                    <div class={modal_body_style.get_class_name()}>
                        { for self.props.children.iter() }
                    </div>
                </div>
            </div>
        }
    }
}

impl Modal {
    /// Create a new modal component
    pub fn new(props: ModalProps) -> Self {
        Self::create(&props)
    }
}
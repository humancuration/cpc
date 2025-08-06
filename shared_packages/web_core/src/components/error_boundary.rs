//! Error boundary component
//!
//! This module provides an error boundary component that can catch
//! and handle errors in child components.
//!
//! The ErrorBoundary component implements both the Yew Component trait
//! for full error boundary functionality and the BaseComponent trait
//! for consistency with other components in the library.
//!
//! ## Usage
//!
//! For full error boundary functionality, use the Yew Component implementation:
//! ```rust
//! use yew::prelude::*;
//! use web_core::components::error_boundary::ErrorBoundary;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ErrorBoundary>
//!             // Child components that might throw errors
//!         </ErrorBoundary>
//!     }
//! }
//! ```
//!
//! For use with the BaseComponent interface (without error boundary functionality):
//! ```rust
//! use web_core::components::{BaseComponent, error_boundary::ErrorBoundary};
//! use yew::prelude::*;
//!
//! let props = ErrorBoundary::Properties::default();
//! let component = ErrorBoundary::create(&props);
//! let html = component.view();
//! ```

use yew::prelude::*;
use crate::utils::error_handling::WebError;

/// Properties for the ErrorBoundary component
#[derive(Properties, PartialEq, Clone)]
pub struct ErrorBoundaryProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// The content to display when there's no error
    #[prop_or_default]
    pub children: Children,
    
    /// Callback when an error occurs
    #[prop_or_default]
    pub on_error: Callback<WebError>,
    
    /// Custom fallback UI to display when an error occurs
    #[prop_or_default]
    pub fallback: Option<Html>,
}

impl Default for ErrorBoundaryProps {
    fn default() -> Self {
        Self {
            common: CommonProps::default(),
            children: Children::default(),
            on_error: Callback::default(),
            fallback: None,
        }
    }
}

/// State for the ErrorBoundary component
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorBoundaryState {
    /// Whether an error has occurred
    has_error: bool,
    
    /// The error message
    error_message: Option<String>,
}

/// Error boundary component that catches and handles errors in child components
pub struct ErrorBoundary {
    /// Component properties
    props: ErrorBoundaryProps,
    
    /// Component state
    state: ErrorBoundaryState,
}

pub enum Msg {
    /// An error has occurred
    ErrorOccurred(String),
    
    /// Reset the error state
    ResetError,
}

impl Component for ErrorBoundary {
    type Message = Msg;
    type Properties = ErrorBoundaryProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            state: ErrorBoundaryState {
                has_error: false,
                error_message: None,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ErrorOccurred(message) => {
                self.state.has_error = true;
                self.state.error_message = Some(message.clone());
                self.props = ctx.props().clone();
                
                // Call the error callback
                self.props.on_error.emit(WebError::ComponentError(message));
                
                true
            }
            Msg::ResetError => {
                self.state.has_error = false;
                self.state.error_message = None;
                self.props = ctx.props().clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.state.has_error {
            // If a custom fallback UI is provided, use it
            if let Some(fallback) = &self.props.fallback {
                return fallback.clone();
            }
            
            // Otherwise, use a default error UI
            html! {
                <div class={classes!("error-boundary", self.props.common.class.clone())}>
                    <h2>{"Something went wrong."}</h2>
                    if let Some(message) = &self.state.error_message {
                        <p>{message}</p>
                    }
                    <button onclick={ctx.link().callback(|_| Msg::ResetError)}>
                        {"Try again"}
                    </button>
                </div>
            }
        }

        html! {
            <div class={self.props.common.class.clone()}>
                { for self.props.children.iter() }
            </div>
        }
    }
}

impl ErrorBoundary {
    /// Create a new error boundary
    pub fn new(props: ErrorBoundaryProps) -> Self {
        Self {
            props,
            state: ErrorBoundaryState {
                has_error: false,
                error_message: None,
            },
        }
    }
    
    /// Check if the component has caught an error
    pub fn has_error(&self) -> bool {
        self.state.has_error
    }
    
    /// Get the error message if an error has occurred
    pub fn error_message(&self) -> Option<&String> {
        self.state.error_message.as_ref()
    }
}

impl Default for ErrorBoundary {
    fn default() -> Self {
        Self::new(ErrorBoundaryProps {
            common: CommonProps::default(),
            children: Children::default(),
            on_error: Callback::default(),
            fallback: None,
        })
    }
}

/// Trait for components that can be wrapped in an error boundary
pub trait ErrorBoundaryWrapper {
    /// Wrap the component in an error boundary
    fn with_error_boundary(self) -> Html;
}

impl BaseComponent for ErrorBoundary {
    type Properties = ErrorBoundaryProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self::new(props.clone())
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    /// Render the component as HTML with common props applied.
    /// Note: This BaseComponent interface does not provide error boundary functionality.
    /// For full error boundary functionality, use the Yew Component implementation which
    /// implements Component trait directly.
    fn view(&self) -> Html {
        // For BaseComponent, we simply render the children without error boundary functionality
        // The error boundary functionality is only available through the Yew Component implementation
        let mut classes = Classes::from("error-boundary");
        if let Some(class) = &self.props.common.class {
            classes.push(class);
        }
        
        html! {
            <div class={classes} id={self.props.common.id.clone()} style={self.props.common.style.clone()}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
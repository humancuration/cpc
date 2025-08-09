//! ThemeProvider component for the UI toolkit
//!
//! This module provides a Yew context-based ThemeProvider component that manages
//! the theme state and provides it to child components.

use yew::prelude::*;
use crate::themes::{ThemeProvider as ThemeManager, ColorScheme};
use stylist::{style, yew::styled_component};

/// Properties for the ThemeProvider component
#[derive(Properties, PartialEq, Clone)]
pub struct ThemeProviderProps {
    /// Child components that will have access to the theme context
    #[prop_or_default]
    pub children: Children,
    
    /// Initial color scheme
    #[prop_or_default]
    pub initial_scheme: Option<ColorScheme>,
}

/// Theme context that will be provided to child components
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
    /// The theme manager
    pub theme_manager: ThemeManager,
    /// Callback to update the theme
    pub set_theme: Callback<ColorScheme>,
    /// Callback to toggle between light and dark mode
    pub toggle_theme: Callback<()>,
}

/// A context provider for the theme system
#[styled_component(ThemeProvider)]
pub struct ThemeProvider {
    props: ThemeProviderProps,
    theme_manager: UseStateHandle<ThemeManager>,
}

impl Component for ThemeProvider {
    type Message = ();
    type Properties = ThemeProviderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut theme_manager = ThemeManager::default();
        
        // If an initial scheme is provided, set it
        if let Some(scheme) = &ctx.props().initial_scheme {
            theme_manager.set_color_scheme(scheme.clone());
        }
        
        Self {
            props: ctx.props().clone(),
            theme_manager: use_state(|| theme_manager),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme_manager = self.theme_manager.clone();
        
        // Create callbacks for updating the theme
        let set_theme = {
            let theme_manager = theme_manager.clone();
            Callback::from(move |scheme: ColorScheme| {
                let mut new_manager = (*theme_manager).clone();
                new_manager.set_color_scheme(scheme);
                theme_manager.set(new_manager);
            })
        };
        
        let toggle_theme = {
            let theme_manager = theme_manager.clone();
            Callback::from(move |_| {
                let mut new_manager = (*theme_manager).clone();
                new_manager.toggle_theme();
                theme_manager.set(new_manager);
            })
        };
        
        // Create the theme context
        let theme_context = ThemeContext {
            theme_manager: (*theme_manager).clone(),
            set_theme,
            toggle_theme,
        };
        
        // Apply the theme CSS to the document
        let theme_css = theme_manager.get_theme_css();
        let style_element = style!(
            r#"{}"#,
            theme_css
        ).expect("Failed to create theme style");
        
        // Set the data-theme attribute on the body element
        let effective_scheme = theme_manager.design_system.get_effective_color_scheme();
        let theme_attr = match effective_scheme {
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
            ColorScheme::System => "system",
        };
        
        // Update the data-theme attribute on the body
        use_effect_with(theme_attr.to_string(), move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        body.set_attribute("data-theme", theme_attr).ok();
                    }
                }
            }
        });
        
        html! {
            <ContextProvider<ThemeContext> context={theme_context}>
                <div class={style_element.get_class_name()}>
                    { for ctx.props().children.iter() }
                </div>
            </ContextProvider<ThemeContext>>
        }
    }
}

impl ThemeProvider {
    /// Create a new theme provider component
    pub fn new(props: ThemeProviderProps) -> Self {
        Self {
            props,
            theme_manager: use_state(|| ThemeManager::default()),
        }
    }
}
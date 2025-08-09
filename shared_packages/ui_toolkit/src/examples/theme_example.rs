//! Theme example component
//!
//! This module provides an example of how to use the ThemeProvider
//! and theme-related hooks in a Yew application.

use yew::prelude::*;
use crate::components::theme_provider::ThemeProvider;
use crate::hooks::use_theme::use_theme;
use crate::themes::ColorScheme;
use crate::components::{Button, ButtonProps, ButtonVariant};
use crate::components::base::CommonProps;

/// Example component demonstrating theme usage
#[function_component(ThemeExample)]
pub fn theme_example() -> Html {
    let theme_context = use_theme();
    
    let on_toggle_theme = {
        let toggle_theme = theme_context.toggle_theme.clone();
        Callback::from(move |_| toggle_theme.emit(()))
    };
    
    let on_set_light = {
        let set_theme = theme_context.set_theme.clone();
        Callback::from(move |_| set_theme.emit(ColorScheme::Light))
    };
    
    let on_set_dark = {
        let set_theme = theme_context.set_theme.clone();
        Callback::from(move |_| set_theme.emit(ColorScheme::Dark))
    };
    
    let on_set_system = {
        let set_theme = theme_context.set_theme.clone();
        Callback::from(move |_| set_theme.emit(ColorScheme::System))
    };
    
    let current_scheme = theme_context.theme_manager.design_system.get_effective_color_scheme();
    let scheme_text = match current_scheme {
        ColorScheme::Light => "Light",
        ColorScheme::Dark => "Dark",
        ColorScheme::System => "System",
    };
    
    html! {
        <div style="padding: 2rem;">
            <h1>{ "Theme Example" }</h1>
            <p>{ format!("Current theme: {}", scheme_text) }</p>
            
            <div style="margin: 1rem 0;">
                <Button 
                    onclick={on_toggle_theme}
                    common={CommonProps {
                        id: Some("toggle-theme".to_string()),
                        ..Default::default()
                    }}
                >
                    { "Toggle Theme" }
                </Button>
            </div>
            
            <div style="margin: 1rem 0;">
                <Button 
                    onclick={on_set_light}
                    variant={ButtonVariant::Outlined}
                    common={CommonProps {
                        id: Some("set-light".to_string()),
                        style: Some("margin-right: 0.5rem".to_string()),
                        ..Default::default()
                    }}
                >
                    { "Set Light" }
                </Button>
                
                <Button 
                    onclick={on_set_dark}
                    variant={ButtonVariant::Outlined}
                    common={CommonProps {
                        id: Some("set-dark".to_string()),
                        style: Some("margin-right: 0.5rem".to_string()),
                        ..Default::default()
                    }}
                >
                    { "Set Dark" }
                </Button>
                
                <Button 
                    onclick={on_set_system}
                    variant={ButtonVariant::Outlined}
                    common={CommonProps {
                        id: Some("set-system".to_string()),
                        ..Default::default()
                    }}
                >
                    { "Set System" }
                </Button>
            </div>
            
            <div style="margin-top: 2rem; padding: 1rem; border: 1px solid var(--cpc-gray-300); border-radius: var(--cpc-border-radius-md);">
                <h2>{ "Theme Colors" }</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 1rem;">
                    <ColorBox name="Primary" color="var(--cpc-primary)" />
                    <ColorBox name="Secondary" color="var(--cpc-secondary)" />
                    <ColorBox name="Success" color="var(--cpc-success)" />
                    <ColorBox name="Warning" color="var(--cpc-warning)" />
                    <ColorBox name="Danger" color="var(--cpc-danger)" />
                    <ColorBox name="Info" color="var(--cpc-info)" />
                    <ColorBox name="Light" color="var(--cpc-light)" />
                    <ColorBox name="Dark" color="var(--cpc-dark)" />
                    <ColorBox name="Text" color="var(--cpc-text)" />
                    <ColorBox name="Background" color="var(--cpc-background)" />
                    <ColorBox name="Surface" color="var(--cpc-surface)" />
                </div>
            </div>
        </div>
    }
}

/// A simple component to display a color box
#[derive(Properties, PartialEq)]
struct ColorBoxProps {
    name: String,
    color: String,
}

#[function_component(ColorBox)]
fn color_box(props: &ColorBoxProps) -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <div 
                style={format!(
                    "width: 60px; height: 60px; border-radius: var(--cpc-border-radius-md); background-color: {}; border: 1px solid var(--cpc-border);",
                    props.color
                )}
            />
            <span style="margin-top: 0.5rem; font-size: 0.875rem;">{ &props.name }</span>
        </div>
    }
}
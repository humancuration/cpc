//! Consent indicator component for the SCM module
//!
//! This module provides a visual indicator for consent settings using the new
//! centralized Consent Manager.

use yew::prelude::*;
use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
    },
    application::service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

/// Size variants for the consent indicator
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorSize {
    Small,
    Medium,
    Large,
}

/// Properties for the consent indicator component
#[derive(Properties, PartialEq)]
pub struct ConsentIndicatorProps {
    /// The user ID to display consent for
    pub user_id: Uuid,
    /// The size of the indicator
    pub size: IndicatorSize,
    /// The consent service to use for fetching consent levels
    pub consent_service: Arc<ConsentService>,
}

/// Consent indicator component
///
/// This component displays a visual representation of consent settings using color-coded rings:
/// - Green = Full sharing
/// - Yellow = Partial sharing
/// - Red = No sharing
#[function_component(ConsentIndicator)]
pub fn consent_indicator(props: &ConsentIndicatorProps) -> Html {
    let consent_level = use_state(|| DataSharingLevel::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    
    let user_id = props.user_id;
    let consent_service = props.consent_service.clone();
    
    // Fetch consent level when component mounts
    use_effect_with_deps(
        move |_| {
            let consent_level = consent_level.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let user_id_str = user_id.to_string();
                match consent_service.get_consent_level(&user_id_str, Domain::ScmData).await {
                    Ok(level) => {
                        consent_level.set(level);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load consent: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        },
        (),
    );
    
    let size_class = match props.size {
        IndicatorSize::Small => "consent-indicator-small",
        IndicatorSize::Medium => "consent-indicator-medium",
        IndicatorSize::Large => "consent-indicator-large",
    };
    
    let sharing_level_class = get_sharing_level_class(&*consent_level);
    let tooltip = get_sharing_tooltip("SCM Data", &*consent_level);
    
    if *loading {
        return html! {
            <div class={classes!("consent-indicator", size_class, "loading")}>
                <div class="spinner"></div>
            </div>
        };
    }
    
    if let Some(err) = &*error {
        return html! {
            <div class={classes!("consent-indicator", size_class, "error")} title={err.clone()}>
                <span>{"⚠️"}</span>
            </div>
        };
    }
    
    html! {
        <div class={classes!("consent-indicator", size_class)} title={tooltip}>
            <div class={classes!("consent-ring", sharing_level_class)}>
            </div>
        </div>
    }
}

/// Get the CSS class for a sharing level
fn get_sharing_level_class(level: &DataSharingLevel) -> &'static str {
    match level {
        DataSharingLevel::None => "consent-level-none",
        DataSharingLevel::Minimal => "consent-level-partial",
        DataSharingLevel::Standard => "consent-level-full",
        DataSharingLevel::Full => "consent-level-full",
    }
}

/// Get the tooltip text for a sharing level
fn get_sharing_tooltip(category: &str, level: &DataSharingLevel) -> String {
    let level_text = match level {
        DataSharingLevel::None => "None",
        DataSharingLevel::Minimal => "Minimal",
        DataSharingLevel::Standard => "Standard",
        DataSharingLevel::Full => "Full",
    };
    format!("{} sharing: {}", category, level_text)
}
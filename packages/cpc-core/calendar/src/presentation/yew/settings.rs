//! Settings page for the Calendar module
//!
//! This module provides a UI for managing calendar settings, including
//! the consent dashboard for data sharing preferences.

use yew::prelude::*;
use crate::application::consent::ConsentService;
use consent_manager::presentation::yew::consent_dashboard::ConsentDashboard;
use consent_manager::domain::consent::Domain;
use std::sync::Arc;
use uuid::Uuid;

/// Properties for the settings component
#[derive(Properties, PartialEq)]
pub struct SettingsProps {
    /// The current user ID
    pub user_id: Uuid,
    /// The consent service for managing data sharing preferences
    pub consent_service: Arc<ConsentService>,
}

/// Settings component for the Calendar module
#[function_component(CalendarSettings)]
pub fn calendar_settings(props: &SettingsProps) -> Html {
    let user_id = props.user_id;
    let consent_service = props.consent_service.clone();
    
    html! {
        <div class="calendar-settings">
            <h1>{"Calendar Settings"}</h1>
            
            <div class="settings-section">
                <h2>{"Data Sharing Preferences"}</h2>
                <p>{"Manage your data sharing preferences for the Calendar module."}</p>
                
                <ConsentDashboard 
                    user_id={user_id.to_string()}
                    domain={Domain::CalendarData}
                    consent_service={consent_service.consent_service.clone()}
                />
            </div>
            
            <div class="settings-section">
                <h2>{"Integration Settings"}</h2>
                <p>{"Configure integrations with other modules."}</p>
                // Add integration settings here
            </div>
            
            <div class="settings-section">
                <h2>{"Notification Settings"}</h2>
                <p>{"Configure notification preferences."}</p>
                // Add notification settings here
            </div>
        </div>
    }
}
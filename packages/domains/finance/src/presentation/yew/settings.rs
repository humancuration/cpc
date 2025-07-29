//! Settings page for the Finance module
//!
//! This module provides a UI for managing finance settings, including
//! the consent dashboard for data sharing preferences.

use yew::prelude::*;
use consent_manager::presentation::yew::consent_dashboard::ConsentDashboard;
use consent_manager::domain::consent::Domain;
use consent_manager::application::service::ConsentService;
use std::sync::Arc;

/// Properties for the settings component
#[derive(Properties, PartialEq)]
pub struct SettingsProps {
    /// The current user ID
    pub user_id: String,
    /// The consent service for managing data sharing preferences
    pub consent_service: Arc<ConsentService>,
}

/// Settings component for the Finance module
#[function_component(FinanceSettings)]
pub fn finance_settings(props: &SettingsProps) -> Html {
    let user_id = props.user_id.clone();
    let consent_service = props.consent_service.clone();
    
    html! {
        <div class="finance-settings">
            <h1>{"Finance Settings"}</h1>
            
            <div class="settings-section">
                <h2>{"Data Sharing Preferences"}</h2>
                <p>{"Manage your data sharing preferences for the Finance module."}</p>
                
                <ConsentDashboard 
                    user_id={user_id}
                    domain={Domain::FinancialData}
                    consent_service={consent_service}
                />
            </div>
            
            <div class="settings-section">
                <h2>{"Budget Settings"}</h2>
                <p>{"Configure your budgeting preferences."}</p>
                // Add budget settings here
            </div>
            
            <div class="settings-section">
                <h2>{"Savings Goals"}</h2>
                <p>{"Manage your savings goals and preferences."}</p>
                // Add savings goals settings here
            </div>
        </div>
    }
}
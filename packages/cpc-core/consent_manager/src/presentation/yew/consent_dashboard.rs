//! Main consent management dashboard component.

use yew::prelude::*;
use crate::{
    domain::consent::{Domain, DataSharingLevel},
    presentation::yew::{permission_toggle::PermissionToggle, audit_log_viewer::AuditLogViewer},
};

/// Properties for the consent dashboard
#[derive(Properties, PartialEq)]
pub struct ConsentDashboardProps {
    /// User ID for which to manage consent
    pub user_id: String,
}

/// State for the consent dashboard
#[derive(Clone, PartialEq)]
pub struct DashboardState {
    /// Financial data consent level
    pub financial_level: DataSharingLevel,
    /// Health data consent level
    pub health_level: DataSharingLevel,
    /// Calendar data consent level
    pub calendar_level: DataSharingLevel,
    /// CRM data consent level
    pub crm_level: DataSharingLevel,
    /// SCM data consent level
    pub scm_level: DataSharingLevel,
    /// Loading state
    pub loading: bool,
    /// Error message if any
    pub error: Option<String>,
}

/// Main consent management dashboard component
#[function_component(ConsentDashboard)]
pub fn consent_dashboard(props: &ConsentDashboardProps) -> Html {
    let state = use_state(|| DashboardState {
        financial_level: DataSharingLevel::None,
        health_level: DataSharingLevel::None,
        calendar_level: DataSharingLevel::None,
        crm_level: DataSharingLevel::None,
        scm_level: DataSharingLevel::None,
        loading: true,
        error: None,
    });

    // In a real implementation, this would fetch current consent levels from the service
    // For now, we'll just simulate loading
    use_effect_with_deps(
        |_| {
            // This would be an async call to fetch consent levels
            // For now, we'll just set loading to false
            || ()
        },
        (),
    );

    let on_level_change = {
        let state = state.clone();
        Callback::from(move |(domain, level): (Domain, DataSharingLevel)| {
            let mut new_state = (*state).clone();
            match domain {
                Domain::FinancialData => new_state.financial_level = level,
                Domain::HealthData => new_state.health_level = level,
                Domain::CalendarData => new_state.calendar_level = level,
                Domain::CrmData => new_state.crm_level = level,
                Domain::ScmData => new_state.scm_level = level,
                _ => {} // Handle other domains as needed
            }
            state.set(new_state);
        })
    };

    html! {
        <div class="consent-dashboard">
            <h1>{"Consent Management"}</h1>
            
            if state.loading {
                <div class="loading">{"Loading..."}</div>
            } else {
                <div class="domain-permissions">
                    <PermissionToggle
                        domain={Domain::FinancialData}
                        current_level={state.financial_level.clone()}
                        on_change={on_level_change.clone()}
                    />
                    
                    <PermissionToggle
                        domain={Domain::HealthData}
                        current_level={state.health_level.clone()}
                        on_change={on_level_change.clone()}
                    />
                    
                    <PermissionToggle
                        domain={Domain::CalendarData}
                        current_level={state.calendar_level.clone()}
                        on_change={on_level_change.clone()}
                    />
                    
                    <PermissionToggle
                        domain={Domain::CrmData}
                        current_level={state.crm_level.clone()}
                        on_change={on_level_change.clone()}
                    />
                    
                    <PermissionToggle
                        domain={Domain::ScmData}
                        current_level={state.scm_level.clone()}
                        on_change={on_level_change.clone()}
                    />
                </div>
                
                <AuditLogViewer user_id={props.user_id.clone()} />
            }
        </div>
    }
}
//! Audit log viewer component.

use yew::prelude::*;
use crate::domain::{
    consent::{Domain, DataSharingLevel},
    audit::{AuditEvent, Actor, ConsentAction},
};

/// Properties for the audit log viewer
#[derive(Properties, PartialEq)]
pub struct AuditLogViewerProps {
    /// User ID for which to show audit log
    pub user_id: String,
}

/// State for the audit log viewer
#[derive(Clone, PartialEq)]
pub struct AuditLogState {
    /// Audit events
    pub events: Vec<AuditEvent>,
    /// Loading state
    pub loading: bool,
    /// Error message if any
    pub error: Option<String>,
}

/// Format domain name for display
fn format_domain(domain: &Domain) -> String {
    match domain {
        Domain::FinancialData => "Financial Data".to_string(),
        Domain::HealthData => "Health Data".to_string(),
        Domain::CalendarData => "Calendar Data".to_string(),
        Domain::CrmData => "CRM Data".to_string(),
        Domain::ScmData => "SCM Data".to_string(),
        Domain::DocumentData => "Document Data".to_string(),
        Domain::WebsiteData => "Website Data".to_string(),
        Domain::RecruitmentData => "Recruitment Data".to_string(),
        Domain::DataLakehouse => "Data Lakehouse".to_string(),
        Domain::ForecastingData => "Forecasting Data".to_string(),
    }
}

/// Format action for display
fn format_action(action: &ConsentAction) -> String {
    match action {
        ConsentAction::Granted => "Granted".to_string(),
        ConsentAction::Revoked => "Revoked".to_string(),
        ConsentAction::Modified => "Modified".to_string(),
    }
}

/// Format level for display
fn format_level(level: &DataSharingLevel) -> String {
    match level {
        DataSharingLevel::None => "None".to_string(),
        DataSharingLevel::Minimal => "Minimal".to_string(),
        DataSharingLevel::Standard => "Standard".to_string(),
        DataSharingLevel::Full => "Full".to_string(),
    }
}

/// Format actor for display
fn format_actor(actor: &Actor) -> String {
    match actor {
        Actor::User(id) => format!("User: {}", id),
        Actor::Service(name) => format!("Service: {}", name),
        Actor::Admin(id) => format!("Admin: {}", id),
    }
}

/// Audit log viewer component
#[function_component(AuditLogViewer)]
pub fn audit_log_viewer(props: &AuditLogViewerProps) -> Html {
    let state = use_state(|| AuditLogState {
        events: vec![], // In a real implementation, this would be fetched from the service
        loading: true,
        error: None,
    });

    // In a real implementation, this would fetch audit events from the service
    // For now, we'll just simulate loading
    use_effect_with_deps(
        |_| {
            // This would be an async call to fetch audit events
            // For now, we'll just set loading to false
            || ()
        },
        (),
    );

    html! {
        <div class="audit-log-viewer">
            <h2>{"Consent History"}</h2>
            
            if state.loading {
                <div class="loading">{"Loading audit log..."}</div>
            } else if let Some(error) = &state.error {
                <div class="error">{error}</div>
            } else {
                <div class="audit-events">
                    if state.events.is_empty() {
                        <div class="no-events">{"No consent events found."}</div>
                    } else {
                        {for state.events.iter().map(|event| {
                            let domain_name = format_domain(&event.domain);
                            let action_name = format_action(&event.action);
                            let new_level_name = format_level(&event.new_level);
                            let actor_name = format_actor(&event.actor);
                            let timestamp = event.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
                            
                            html! {
                                <div class="audit-event">
                                    <div class="event-header">
                                        <span class="domain">{domain_name}</span>
                                        <span class="action">{action_name}</span>
                                        <span class="timestamp">{timestamp}</span>
                                    </div>
                                    <div class="event-details">
                                        <span class="level">{"New level: "}{new_level_name}</span>
                                        <span class="actor">{actor_name}</span>
                                        if let Some(prev_level) = &event.previous_level {
                                            <span class="previous">{"Previous: "}{format_level(prev_level)}</span>
                                        }
                                    </div>
                                </div>
                            }
                        })}
                    }
                </div>
            }
        </div>
    }
}
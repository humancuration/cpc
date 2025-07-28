//! Interactive domain permission toggle component.

use yew::prelude::*;
use crate::domain::consent::{Domain, DataSharingLevel};

/// Properties for the permission toggle
#[derive(Properties, PartialEq)]
pub struct PermissionToggleProps {
    /// The domain for which to toggle permissions
    pub domain: Domain,
    /// Current consent level
    pub current_level: DataSharingLevel,
    /// Callback when level changes
    pub on_change: Callback<(Domain, DataSharingLevel)>,
}

/// Get domain name as string
fn domain_name(domain: &Domain) -> &'static str {
    match domain {
        Domain::FinancialData => "Financial Data",
        Domain::HealthData => "Health Data",
        Domain::CalendarData => "Calendar Data",
        Domain::CrmData => "CRM Data",
        Domain::ScmData => "SCM Data",
        Domain::DocumentData => "Document Data",
        Domain::WebsiteData => "Website Data",
        Domain::RecruitmentData => "Recruitment Data",
        Domain::DataLakehouse => "Data Lakehouse",
        Domain::ForecastingData => "Forecasting Data",
    }
}

/// Get level description
fn level_description(level: &DataSharingLevel) -> &'static str {
    match level {
        DataSharingLevel::None => "No data sharing",
        DataSharingLevel::Minimal => "Minimal data sharing",
        DataSharingLevel::Standard => "Standard data sharing",
        DataSharingLevel::Full => "Full data sharing",
    }
}

/// Interactive domain permission toggle component
#[function_component(PermissionToggle)]
pub fn permission_toggle(props: &PermissionToggleProps) -> Html {
    let domain_name = domain_name(&props.domain);
    let current_description = level_description(&props.current_level);
    
    let on_select = {
        let on_change = props.on_change.clone();
        let domain = props.domain.clone();
        Callback::from(move |level: DataSharingLevel| {
            on_change.emit((domain.clone(), level));
        })
    };

    html! {
        <div class="permission-toggle">
            <div class="domain-header">
                <h3>{domain_name}</h3>
                <span class="current-level">{current_description}</span>
            </div>
            
            <div class="level-selector">
                <button
                    class={if props.current_level == DataSharingLevel::None { "selected" } else { "" }}
                    onclick={on_select.reform(|_| DataSharingLevel::None)}
                >
                    {"None"}
                </button>
                
                <button
                    class={if props.current_level == DataSharingLevel::Minimal { "selected" } else { "" }}
                    onclick={on_select.reform(|_| DataSharingLevel::Minimal)}
                >
                    {"Minimal"}
                </button>
                
                <button
                    class={if props.current_level == DataSharingLevel::Standard { "selected" } else { "" }}
                    onclick={on_select.reform(|_| DataSharingLevel::Standard)}
                >
                    {"Standard"}
                </button>
                
                <button
                    class={if props.current_level == DataSharingLevel::Full { "selected" } else { "" }}
                    onclick={on_select.reform(|_| DataSharingLevel::Full)}
                >
                    {"Full"}
                </button>
            </div>
        </div>
    }
}
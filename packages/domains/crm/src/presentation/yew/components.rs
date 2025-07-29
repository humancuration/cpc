//! Yew components for CRM functionality
//!
//! This module contains the main UI components for the CRM module.

use yew::prelude::*;
use crate::domain::contact::Contact;
use crate::domain::interaction::Interaction;
use crate::domain::pipeline::Pipeline;
use crate::domain::deal::Deal;
use crate::presentation::yew::consent_indicator::{ConsentIndicator, ConsentIndicatorProps, IndicatorSize};

/// Properties for the contact list component
#[derive(Properties, PartialEq)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
    pub on_contact_selected: Callback<Contact>,
}

/// Contact list component
#[function_component(ContactList)]
pub fn contact_list(props: &ContactListProps) -> Html {
    let on_contact_selected = props.on_contact_selected.clone();
    
    html! {
        <div class="contact-list">
            <h2>{"Contacts"}</h2>
            <ul>
                {for props.contacts.iter().map(|contact| {
                    let contact = contact.clone();
                    let on_click = on_contact_selected.reform(move |_| contact.clone());
                    
                    html! {
                        <li onclick={on_click}>
                            <span class="contact-name">{&contact.name}</span>
                            if let Some(company) = &contact.company {
                                <span class="contact-company">{" - "}{company}</span>
                            }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

/// Properties for the contact detail component
#[derive(Properties, PartialEq)]
pub struct ContactDetailProps {
    pub contact: Option<Contact>,
}

/// Contact detail component
#[function_component(ContactDetail)]
pub fn contact_detail(props: &ContactDetailProps) -> Html {
    Some(contact) => {
        html! {
            <div class="contact-detail">
                <h2>{&contact.name}</h2>
                if let Some(email) = &contact.primary_email {
                    <p>{"Email: "}{email}</p>
                }
                if let Some(phone) = &contact.primary_phone {
                    <p>{"Phone: "}{phone}</p>
                }
                if let Some(company) = &contact.company {
                    <p>{"Company: "}{company}</p>
                }
                <p>{"Contact Type: "}{if contact.is_platform_native() { "Platform Native" } else { "External" }}</p>
                if contact.is_platform_native() {
                    if let crate::domain::contact::ContactType::PlatformNative(_, consent_settings) = &contact.contact_type {
                        <div class="consent-indicator-container">
                            <h3>{"Consent Settings"}</h3>
                            <ConsentIndicator settings={consent_settings.clone()} size={IndicatorSize::Medium} />
                        </div>
                    }
                }
                if !contact.tags.is_empty() {
                    <p>{"Tags: "}{contact.tags.join(", ")}</p>
                }
            </div>
        }
    }
    None => {
        html! {
            <div class="contact-detail">
                <p>{"Select a contact to view details"}</p>
            </div>
        }
    }
}
}
}

/// Properties for the interaction timeline component
#[derive(Properties, PartialEq)]
pub struct InteractionTimelineProps {
    pub interactions: Vec<Interaction>,
}

/// Interaction timeline component
#[function_component(InteractionTimeline)]
pub fn interaction_timeline(props: &InteractionTimelineProps) -> Html {
    html! {
        <div class="interaction-timeline">
            <h2>{"Interaction Timeline"}</h2>
            <ul>
                {for props.interactions.iter().map(|interaction| {
                    html! {
                        <li class="interaction-item">
                            <span class="interaction-type">{format!("{:?}", interaction.interaction_type)}</span>
                            <span class="interaction-summary">{&interaction.summary}</span>
                            <span class="interaction-date">{interaction.timestamp.format("%Y-%m-%d %H:%M").to_string()}</span>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

/// Properties for the pipeline board component
#[derive(Properties, PartialEq)]
pub struct PipelineBoardProps {
    pub pipeline: Pipeline,
    pub deals: Vec<Deal>,
}

/// Pipeline board component
#[function_component(PipelineBoard)]
pub fn pipeline_board(props: &PipelineBoardProps) -> Html {
    html! {
        <div class="pipeline-board">
            <h2>{&props.pipeline.name}</h2>
            <div class="pipeline-stages">
                {for props.pipeline.stages.iter().map(|stage| {
                    let stage_deals: Vec<&Deal> = props.deals.iter()
                        .filter(|deal| deal.current_stage == stage.id)
                        .collect();
                    
                    html! {
                        <div class="pipeline-stage">
                            <h3>{&stage.name} {" ("}{stage_deals.len()}{")"}</h3>
                            <div class="stage-deals">
                                {for stage_deals.iter().map(|deal| {
                                    html! {
                                        <div class="deal-card">
                                            <h4>{&deal.title}</h4>
                                            <p>{format!("${:.2}", deal.value.amount)}</p>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Properties for the consent management component
#[derive(Properties, PartialEq)]
pub struct ConsentManagementProps {
    pub contact: Contact,
    pub on_consent_updated: Callback<Contact>,
}

/// Consent management component
#[function_component(ConsentManagement)]
pub fn consent_management(props: &ConsentManagementProps) -> Html {
    if !props.contact.is_platform_native() {
        return html! {
            <div class="consent-management">
                <p>{"Consent management is only available for platform-native contacts."}</p>
            </div>
        };
    }
    
    html! {
        <div class="consent-management">
            <h2>{"Consent Management"}</h2>
            <p>{"Manage data sharing permissions for this contact."}</p>
            // In a real implementation, this would include controls for managing consent settings
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::platform::spawn_local;
    
    #[test]
    fn test_contact_list_component() {
        // Component tests would typically be done with wasm-bindgen-test
        // This is just a placeholder to show the structure
    }
}
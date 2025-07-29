//! Yew UI components for email campaign management
//!
//! This module contains the Yew components for managing email campaigns.

use yew::prelude::*;
use crate::domain::email_provider::{EmailCampaign, CampaignStatus, TargetSegment, CampaignMetrics};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Properties for the EmailCampaignList component
#[derive(Properties, PartialEq)]
pub struct EmailCampaignListProps {
    pub campaigns: Vec<EmailCampaign>,
    pub on_create_campaign: Callback<()>,
    pub on_edit_campaign: Callback<Uuid>,
    pub on_delete_campaign: Callback<Uuid>,
}

/// Component to display a list of email campaigns
#[function_component(EmailCampaignList)]
pub fn email_campaign_list(props: &EmailCampaignListProps) -> Html {
    let on_create = {
        let on_create_campaign = props.on_create_campaign.clone();
        Callback::from(move |_| on_create_campaign.emit(()))
    };

    html! {
        <div class="email-campaign-list">
            <div class="header">
                <h2>{"Email Campaigns"}</h2>
                <button class="btn btn-primary" onclick={on_create}>
                    {"Create Campaign"}
                </button>
            </div>
            <div class="campaigns">
                {for props.campaigns.iter().map(|campaign| {
                    html! {
                        <EmailCampaignItem 
                            campaign={campaign.clone()} 
                            on_edit={props.on_edit_campaign.clone()}
                            on_delete={props.on_delete_campaign.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}

/// Properties for the EmailCampaignItem component
#[derive(Properties, PartialEq)]
pub struct EmailCampaignItemProps {
    pub campaign: EmailCampaign,
    pub on_edit: Callback<Uuid>,
    pub on_delete: Callback<Uuid>,
}

/// Component to display a single email campaign
#[function_component(EmailCampaignItem)]
pub fn email_campaign_item(props: &EmailCampaignItemProps) -> Html {
    let campaign = &props.campaign;
    
    let on_edit = {
        let on_edit = props.on_edit.clone();
        let campaign_id = campaign.id;
        Callback::from(move |_| on_edit.emit(campaign_id))
    };
    
    let on_delete = {
        let on_delete = props.on_delete.clone();
        let campaign_id = campaign.id;
        Callback::from(move |_| on_delete.emit(campaign_id))
    };

    html! {
        <div class="campaign-item">
            <div class="campaign-header">
                <h3>{&campaign.name}</h3>
                <div class="campaign-status">
                    <span class={status_class(&campaign.status)}>{status_text(&campaign.status)}</span>
                </div>
            </div>
            <div class="campaign-details">
                <p><strong>{"Subject:"}</strong> {&campaign.subject}</p>
                <p><strong>{"Target Segment:"}</strong> {&campaign.target_segment.name}</p>
                <p><strong>{"Created:"}</strong> {campaign.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                if let Some(scheduled) = campaign.scheduled_time {
                    <p><strong>{"Scheduled:"}</strong> {scheduled.format("%Y-%m-%d %H:%M").to_string()}</p>
                }
            </div>
            <div class="campaign-metrics">
                <MetricsDisplay metrics={campaign.metrics.clone()} />
            </div>
            <div class="campaign-actions">
                <button class="btn btn-secondary" onclick={on_edit}>{"Edit"}</button>
                <button class="btn btn-danger" onclick={on_delete}>{"Delete"}</button>
            </div>
        </div>
    }
}

/// Properties for the MetricsDisplay component
#[derive(Properties, PartialEq)]
pub struct MetricsDisplayProps {
    pub metrics: CampaignMetrics,
}

/// Component to display campaign metrics
#[function_component(MetricsDisplay)]
pub fn metrics_display(props: &MetricsDisplayProps) -> Html {
    let metrics = &props.metrics;
    
    html! {
        <div class="metrics-display">
            <div class="metric">
                <span class="metric-label">{"Sent"}</span>
                <span class="metric-value">{metrics.sent_count}</span>
            </div>
            <div class="metric">
                <span class="metric-label">{"Open Rate"}</span>
                <span class="metric-value">{format!("{:.1}%", metrics.open_rate * 100.0)}</span>
            </div>
            <div class="metric">
                <span class="metric-label">{"Click Rate"}</span>
                <span class="metric-value">{format!("{:.1}%", metrics.click_rate * 100.0)}</span>
            </div>
            <div class="metric">
                <span class="metric-label">{"Bounces"}</span>
                <span class="metric-value">{metrics.bounce_count}</span>
            </div>
            <div class="metric">
                <span class="metric-label">{"Unsubscribes"}</span>
                <span class="metric-value">{metrics.unsubscribe_count}</span>
            </div>
        </div>
    }
}

/// Properties for the EmailCampaignForm component
#[derive(Properties, PartialEq)]
pub struct EmailCampaignFormProps {
    pub campaign: Option<EmailCampaign>,
    pub on_save: Callback<EmailCampaign>,
    pub on_cancel: Callback<()>,
}

/// State for the EmailCampaignForm component
#[derive(Clone)]
struct CampaignFormState {
    name: String,
    subject: String,
    content: String,
    target_segment: String,
}

/// Component to create or edit an email campaign
#[function_component(EmailCampaignForm)]
pub fn email_campaign_form(props: &EmailCampaignFormProps) -> Html {
    let state = use_state(|| {
        if let Some(campaign) = &props.campaign {
            CampaignFormState {
                name: campaign.name.clone(),
                subject: campaign.subject.clone(),
                content: campaign.content.clone(),
                target_segment: campaign.target_segment.name.clone(),
            }
        } else {
            CampaignFormState {
                name: String::new(),
                subject: String::new(),
                content: String::new(),
                target_segment: String::new(),
            }
        }
    });

    let on_name_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.name = input.value();
            state.set(new_state);
        })
    };

    let on_subject_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.subject = input.value();
            state.set(new_state);
        })
    };

    let on_content_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.content = input.value();
            state.set(new_state);
        })
    };

    let on_target_segment_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.target_segment = input.value();
            state.set(new_state);
        })
    };

    let on_save = {
        let state = state.clone();
        let on_save = props.on_save.clone();
        let campaign_id = props.campaign.as_ref().map(|c| c.id);
        let created_at = props.campaign.as_ref().map(|c| c.created_at).unwrap_or_else(Utc::now);
        let updated_at = Utc::now();
        
        Callback::from(move |_| {
            let new_state = (*state).clone();
            let target_segment = TargetSegment {
                name: new_state.target_segment.clone(),
                criteria: serde_json::Value::Null, // In a real implementation, this would be more complex
            };
            
            let campaign = EmailCampaign {
                id: campaign_id.unwrap_or_else(Uuid::new_v4),
                name: new_state.name.clone(),
                subject: new_state.subject.clone(),
                content: new_state.content.clone(),
                status: CampaignStatus::Draft,
                scheduled_time: None,
                target_segment,
                metrics: CampaignMetrics::default(),
                created_at,
                updated_at,
            };
            
            on_save.emit(campaign);
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="email-campaign-form">
            <h2>
                {if props.campaign.is_some() { "Edit Campaign" } else { "Create Campaign" }}
            </h2>
            <form>
                <div class="form-group">
                    <label for="name">{"Campaign Name"}</label>
                    <input
                        type="text"
                        id="name"
                        value={state.name.clone()}
                        oninput={on_name_input}
                        placeholder="Enter campaign name"
                    />
                </div>
                
                <div class="form-group">
                    <label for="subject">{"Email Subject"}</label>
                    <input
                        type="text"
                        id="subject"
                        value={state.subject.clone()}
                        oninput={on_subject_input}
                        placeholder="Enter email subject"
                    />
                </div>
                
                <div class="form-group">
                    <label for="target_segment">{"Target Segment"}</label>
                    <input
                        type="text"
                        id="target_segment"
                        value={state.target_segment.clone()}
                        oninput={on_target_segment_input}
                        placeholder="Enter target segment"
                    />
                </div>
                
                <div class="form-group">
                    <label for="content">{"Email Content"}</label>
                    <textarea
                        id="content"
                        value={state.content.clone()}
                        oninput={on_content_input}
                        placeholder="Enter email content"
                        rows="10"
                    />
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-primary" onclick={on_save}>
                        {"Save"}
                    </button>
                    <button type="button" class="btn btn-secondary" onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                </div>
            </form>
        </div>
    }
}

/// Helper function to get CSS class for campaign status
fn status_class(status: &CampaignStatus) -> &'static str {
    match status {
        CampaignStatus::Draft => "status-draft",
        CampaignStatus::Scheduled => "status-scheduled",
        CampaignStatus::Sending => "status-sending",
        CampaignStatus::Completed => "status-completed",
        CampaignStatus::Cancelled => "status-cancelled",
    }
}

/// Helper function to get text for campaign status
fn status_text(status: &CampaignStatus) -> &'static str {
    match status {
        CampaignStatus::Draft => "Draft",
        CampaignStatus::Scheduled => "Scheduled",
        CampaignStatus::Sending => "Sending",
        CampaignStatus::Completed => "Completed",
        CampaignStatus::Cancelled => "Cancelled",
    }
}
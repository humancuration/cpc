//! Email campaign UI components for the advanced CRM module
//!
//! This module contains Yew components for creating and managing email campaigns.

use yew::prelude::*;

/// Properties for the email campaign list component
#[derive(Properties, PartialEq)]
pub struct EmailCampaignListProps {
    pub campaigns: Vec<crate::domain::email_campaign::EmailCampaign>,
}

/// Email campaign list component
#[function_component(EmailCampaignList)]
pub fn email_campaign_list(props: &EmailCampaignListProps) -> Html {
    html! {
        <div class="email-campaign-list">
            <h2>{"Email Campaigns"}</h2>
            <div class="campaign-list">
                { for props.campaigns.iter().map(|campaign| html! {
                    <div class="campaign-item">
                        <h3>{ &campaign.name }</h3>
                        <p>{ &campaign.subject }</p>
                        <span class="campaign-status">{ format!("{:?}", campaign.status) }</span>
                    </div>
                }) }
            </div>
        </div>
    }
}
//! Lead scoring UI components for the advanced CRM module
//!
//! This module contains Yew components for displaying and managing lead scoring.

use yew::prelude::*;

/// Properties for the lead scoring dashboard component
#[derive(Properties, PartialEq)]
pub struct LeadScoringDashboardProps {
    pub leads: Vec<crate::domain::lead_scoring::LeadScore>,
}

/// Lead scoring dashboard component
#[function_component(LeadScoringDashboard)]
pub fn lead_scoring_dashboard(props: &LeadScoringDashboardProps) -> Html {
    html! {
        <div class="lead-scoring-dashboard">
            <h2>{"Lead Scoring Dashboard"}</h2>
            <div class="lead-list">
                { for props.leads.iter().map(|lead| html! {
                    <div class="lead-item">
                        <span class="lead-score">{ lead.total_score }</span>
                        <span class="lead-id">{ format!("{}", lead.lead_id) }</span>
                    </div>
                }) }
            </div>
        </div>
    }
}
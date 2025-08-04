// Application Card Component
// ADR 0008: Volunteer Coordination - This component is part of volunteer coordination UI.
// TODO(GraphQL): Wire to api_server schema mutation: reviewApplication (Approve/Reject)

use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use yew::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq)]
pub enum ReviewDecision {
    Approve,
    Reject,
}

#[derive(Clone, PartialEq)]
pub struct ApplicationView {
    pub id: Uuid,
    pub opportunity_id: Uuid,
    pub applicant_id: Uuid,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Properties, PartialEq)]
pub struct ApplicationCardProps {
    pub application: ApplicationView,
    #[prop_or(false)]
    pub can_review: bool,
    pub on_review: Callback<(Uuid, ReviewDecision)>,
}

#[styled_component(ApplicationCard)]
pub fn application_card(props: &ApplicationCardProps) -> Html {
    let css = Style::new(r#"
        .card {
            border: 1px solid #e5e7eb;
            background: #fff;
            border-radius: 10px;
            padding: 12px;
            display: flex; 
            flex-direction: column;
            gap: 8px;
            box-shadow: 0 1px 2px rgba(0,0,0,0.04);
        }
        .row { display: flex; gap: 8px; align-items: center; }
        .muted { color: #6b7280; font-size: 12px; }
        .status {
            display: inline-block; padding: 2px 8px; border-radius: 9999px; 
            font-size: 12px; background: #eef2ff; color: #3730a3;
        }
        .actions { display: flex; gap: 8px; margin-top: 4px; }
        button {
            padding: 6px 10px; border: none; border-radius: 6px; cursor: pointer;
            font-weight: 600; color: white;
        }
        .approve { background: #16a34a; }
        .reject { background: #dc2626; }
        button:disabled { opacity: .6; cursor: not-allowed; }
    "#).expect("style");

    let approving = use_state(|| false);
    let rejecting = use_state(|| false);

    let on_approve = {
        let approving = approving.clone();
        let rejecting = rejecting.clone();
        let on_review = props.on_review.clone();
        let id = props.application.id;
        Callback::from(move |_| {
            if *rejecting { return; }
            approving.set(true);
            on_review.emit((id, ReviewDecision::Approve));
            approving.set(false); // demo only; parent should control in real flow
        })
    };

    let on_reject = {
        let approving = approving.clone();
        let rejecting = rejecting.clone();
        let on_review = props.on_review.clone();
        let id = props.application.id;
        Callback::from(move |_| {
            if *approving { return; }
            rejecting.set(true);
            on_review.emit((id, ReviewDecision::Reject));
            rejecting.set(false); // demo only
        })
    };

    let app = &props.application;

    html! {
        <div class={css}>
            <div class="card">
                <div class="row">
                    <span class="status">{ app.status.clone() }</span>
                    <span class="muted">{ format!("Created: {}", app.created_at) }</span>
                </div>
                <div class="row">
                    <strong>{ "Applicant ID:" }</strong>
                    <span>{ app.applicant_id.to_string() }</span>
                </div>
                if let Some(msg) = &app.message {
                    <div class="row">
                        <strong>{ "Message:" }</strong>
                        <span>{ msg.clone() }</span>
                    </div>
                }
                if let Some(reviewer) = app.reviewed_by {
                    <div class="row">
                        <strong>{ "Reviewed by:" }</strong>
                        <span>{ reviewer.to_string() }</span>
                        if let Some(at) = app.reviewed_at {
                            <span class="muted">{ format!("at {}", at) }</span>
                        }
                    </div>
                }
                if props.can_review {
                    <div class="actions">
                        <button class="approve" onclick={on_approve} disabled={*approving || *rejecting}>{ "Approve" }</button>
                        <button class="reject" onclick={on_reject} disabled={*approving || *rejecting}>{ "Reject" }</button>
                    </div>
                }
            </div>
        </div>
    }
}

// TODO: Add tests if Yew test patterns exist in this crate.
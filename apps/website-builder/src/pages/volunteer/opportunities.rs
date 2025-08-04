// Volunteer Opportunities Demo Page
// ADR 0008: Volunteer Coordination - Demo UI for opportunities, applications, contributions.
// This page showcases the three components and stubs callbacks. Networking intentionally decoupled.

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use chrono::{Utc};

use crate::web::types; // keep available for future GraphQL typing if needed

// Components
use crate::components::volunteer::opportunity_form::{OpportunityForm, CreateOpportunityFormData};
use crate::components::volunteer::application_card::{ApplicationCard, ApplicationView, ReviewDecision};
use crate::components::volunteer::contribution_tracker::{ContributionTracker, ContributionView, ContributionKind, LogContributionFormData};

#[styled_component(VolunteerOpportunitiesPage)]
pub fn volunteer_opportunities_page() -> Html {
    let css = Style::new(r#"
        .page { display:flex; flex-direction:column; gap: 20px; padding: 16px; }
        .grid { display:grid; grid-template-columns: 1fr; gap: 16px; }
        @media (min-width: 960px) { .grid { grid-template-columns: 1fr 1fr; } }
        .section { background:#f8fafc; border: 1px solid #e5e7eb; border-radius: 10px; padding: 12px; }
        h2 { margin: 0 0 8px 0; }
        ul { margin: 0; padding-left: 18px; }
        .muted { color:#6b7280; font-size: 13px; }
    "#).expect("style");

    // Placeholder opportunities state
    #[derive(Clone, PartialEq)]
    struct Opp {
        id: Uuid,
        title: String,
        description: String,
        skills: Vec<String>,
        location: String,
    }
    let opportunities = use_state(|| vec![
        Opp {
            id: Uuid::new_v4(),
            title: "Community Garden Helper".into(),
            description: "Help with planting and watering at the local community garden.".into(),
            skills: vec!["gardening".into(), "composting".into()],
            location: "Oakland, CA".into(),
        },
        Opp {
            id: Uuid::new_v4(),
            title: "Mutual Aid Delivery".into(),
            description: "Deliver groceries and essentials to neighbors in need.".into(),
            skills: vec!["driving".into(), "logistics".into()],
            location: "Remote / Local".into(),
        }
    ]);

    // Handle new opportunity submission (demo: push into list)
    let on_submit = {
        let opportunities = opportunities.clone();
        Callback::from(move |data: CreateOpportunityFormData| {
            web_sys::console::log_1(&format!("[demo] create_opportunity: {:?}", data.title).into());
            let mut next = (*opportunities).clone();
            next.push(Opp {
                id: Uuid::new_v4(),
                title: data.title,
                description: data.description,
                skills: data.skills_needed,
                location: data.location,
            });
            opportunities.set(next);
            // TODO(GraphQL): call createOpportunity mutation (see ADR 0008) via api_server schema.
        })
    };

    // ApplicationCard demo data
    let app_demo = ApplicationView {
        id: Uuid::new_v4(),
        opportunity_id: opportunities.get(0).map(|o| o.id).unwrap_or_else(Uuid::new_v4),
        applicant_id: Uuid::new_v4(),
        message: Some("I'd love to help on weekends.".into()),
        status: "SUBMITTED".into(),
        created_at: Utc::now(),
        reviewed_by: None,
        reviewed_at: None,
    };

    let on_review = Callback::from(|(app_id, decision): (Uuid, ReviewDecision)| {
        web_sys::console::log_1(&format!("[demo] review_application: {} -> {:?}", app_id, decision).into());
        // TODO(GraphQL): call reviewApplication mutation (Approve/Reject)
    });

    // ContributionTracker demo data
    let contribs = vec![
        ContributionView {
            id: Uuid::new_v4(),
            kind: ContributionKind::Hours,
            amount: 2.5,
            notes: Some("Watered beds 2 and 3".into()),
            occurred_at: Utc::now(),
            verified: true,
            verified_by: Some(Uuid::new_v4()),
            verified_at: Some(Utc::now()),
        },
        ContributionView {
            id: Uuid::new_v4(),
            kind: ContributionKind::Deliverable,
            amount: 1.0,
            notes: Some("Delivered compost bags".into()),
            occurred_at: Utc::now(),
            verified: false,
            verified_by: None,
            verified_at: None,
        }
    ];
    let on_log = Callback::from(|payload: LogContributionFormData| {
        web_sys::console::log_1(&format!("[demo] log_contribution: opp={} kind={:?} amount={}", payload.opportunity_id, payload.kind, payload.amount).into());
        // TODO(GraphQL): call logContribution; organizer flow then verifyContribution mutation.
    });

    let contributor_id = Uuid::new_v4();
    let opp_for_contrib = opportunities.get(0).map(|o| o.id).unwrap_or_else(Uuid::new_v4);

    html! {
        <div class={css}>
            <div class="page">
                <div class="section">
                    <h2>{ "Create Opportunity" }</h2>
                    <p class="muted">{ "Fill in details to add a new volunteer opportunity. Demo-only; no networking yet." }</p>
                    <OpportunityForm on_submit={on_submit} />
                </div>

                <div class="section">
                    <h2>{ "Opportunities" }</h2>
                    <ul>
                        { for opportunities.iter().map(|o| html!{
                            <li>
                                <strong>{ &o.title }</strong>
                                { format!(" — {} — skills: {}", o.location, o.skills.join(", ")) }
                                <div class="muted">{ &o.description }</div>
                            </li>
                        }) }
                    </ul>
                </div>

                <div class="grid">
                    <div class="section">
                        <h2>{ "Application Card (Demo)" }</h2>
                        <ApplicationCard application={app_demo} can_review={true} on_review={on_review} />
                    </div>

                    <div class="section">
                        <h2>{ "Contribution Tracker (Demo)" }</h2>
                        <ContributionTracker
                            opportunity_id={opp_for_contrib}
                            contributor_id={contributor_id}
                            contributions={contribs}
                            on_log={on_log}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
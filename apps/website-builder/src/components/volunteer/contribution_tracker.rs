// Contribution Tracker Component
// ADR 0008: Volunteer Coordination - This component is part of volunteer coordination UI.
// TODO(GraphQL): Wire to api_server schema mutations: logContribution, verifyContribution

use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use yew::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq)]
pub enum ContributionKind {
    Hours,
    Deliverable,
    Donation,
    Other,
}

#[derive(Clone, PartialEq)]
pub struct ContributionView {
    pub id: Uuid,
    pub kind: ContributionKind,
    pub amount: f32, // for Hours, number of hours; for others, quantity/points
    pub notes: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq)]
pub struct LogContributionFormData {
    pub opportunity_id: Uuid,
    pub contributor_id: Uuid,
    pub kind: ContributionKind,
    pub amount: f32,
    pub notes: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct ContributionTrackerProps {
    pub opportunity_id: Uuid,
    pub contributor_id: Uuid,
    pub contributions: Vec<ContributionView>,
    pub on_log: Callback<LogContributionFormData>,
}

#[styled_component(ContributionTracker)]
pub fn contribution_tracker(props: &ContributionTrackerProps) -> Html {
    let css = Style::new(r#"
        .wrap { display: flex; flex-direction: column; gap: 12px; }
        .form {
            border: 1px solid #e5e7eb; background: #fafafa; border-radius: 8px; padding: 12px;
            display: grid; grid-template-columns: 1fr 1fr; gap: 12px;
        }
        .row { display: flex; flex-direction: column; gap: 6px; }
        label { font-weight: 600; }
        select, input[type="number"], textarea {
            padding: 8px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px;
        }
        .actions { grid-column: 1 / -1; display: flex; gap: 8px; }
        button { padding: 8px 12px; border: none; border-radius: 6px; background: #2563eb; color: white; font-weight: 600; cursor: pointer; }
        button[disabled] { opacity: .6; cursor: not-allowed; }
        .list { display: flex; flex-direction: column; gap: 8px; }
        .item { border: 1px solid #e5e7eb; border-radius: 8px; padding: 10px; background: #fff; display:flex; flex-direction:column; gap:6px; }
        .muted { color: #6b7280; font-size: 12px; }
        .badge { display:inline-block; padding:2px 8px; border-radius:9999px; font-size: 12px; }
        .verified { background:#ecfdf5; color:#065f46; }
        .unverified { background:#fff7ed; color:#9a3412; }
    "#).expect("valid style");

    let kind = use_state(|| ContributionKind::Hours);
    let hours = use_state(|| 1.0_f32);
    let notes = use_state(|| String::new());
    let submitting = use_state(|| false);

    // Derived: whether show hours input
    let is_hours = matches!(*kind, ContributionKind::Hours);

    let on_kind_change = {
        let kind = kind.clone();
        Callback::from(move |e: Event| {
            if let Some(sel) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let v = sel.value();
                let k = match v.as_str() {
                    "Hours" => ContributionKind::Hours,
                    "Deliverable" => ContributionKind::Deliverable,
                    "Donation" => ContributionKind::Donation,
                    _ => ContributionKind::Other,
                };
                kind.set(k);
            }
        })
    };

    let on_hours_change = {
        let hours = hours.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(inp) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val = inp.value().parse::<f32>().unwrap_or(0.0);
                hours.set(val);
            }
        })
    };

    let on_notes_change = {
        let notes = notes.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(el) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                notes.set(el.value());
            }
        })
    };

    let on_submit = {
        let submitting = submitting.clone();
        let kind = kind.clone();
        let hours = hours.clone();
        let notes = notes.clone();
        let on_log = props.on_log.clone();
        let opportunity_id = props.opportunity_id;
        let contributor_id = props.contributor_id;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *submitting { return; }
            submitting.set(true);

            let payload = LogContributionFormData {
                opportunity_id,
                contributor_id,
                kind: (*kind).clone(),
                amount: *hours,
                notes: if notes.is_empty() { None } else { Some((*notes).clone()) },
            };

            on_log.emit(payload);

            submitting.set(false); // demo reset
        })
    };

    html! {
        <div class={css}>
            <div class="wrap">
                <form class="form">
                    <div class="row">
                        <label for="kind">{ "Kind" }</label>
                        <select id="kind" onchange={on_kind_change}>
                            <option value="Hours" selected={matches!(*kind, ContributionKind::Hours)}>{ "Hours" }</option>
                            <option value="Deliverable" selected={matches!(*kind, ContributionKind::Deliverable)}>{ "Deliverable" }</option>
                            <option value="Donation" selected={matches!(*kind, ContributionKind::Donation)}>{ "Donation" }</option>
                            <option value="Other" selected={matches!(*kind, ContributionKind::Other)}>{ "Other" }</option>
                        </select>
                    </div>

                    if is_hours {
                        <div class="row">
                            <label for="hours">{ "Hours" }</label>
                            <input id="hours" type="number" min="0" step="0.25" value={hours.to_string()} oninput={on_hours_change} />
                        </div>
                    }

                    <div class="row" style="grid-column: 1 / -1;">
                        <label for="notes">{ "Notes" }</label>
                        <textarea id="notes" rows={3} value={(*notes).clone()} oninput={on_notes_change} placeholder="Optional notes about the contribution..." />
                    </div>

                    <div class="actions">
                        <button onclick={on_submit} disabled={*submitting}>{ "Log Contribution" }</button>
                    </div>
                </form>

                <div class="list">
                    { for props.contributions.iter().map(render_item) }
                </div>
            </div>
        </div>
    }
}

fn render_item(c: &ContributionView) -> Html {
    let badge_class = if c.verified { "badge verified" } else { "badge unverified" };
    html! {
        <div class="item">
            <div>
                <strong>{ format!("{:?}", c.kind) }</strong>
                { " - " }
                <span>{ format!("Amount: {}", c.amount) }</span>
                { " · " }
                <span class="muted">{ format!("Occurred: {}", c.occurred_at) }</span>
            </div>
            if let Some(n) = &c.notes {
                <div class="muted">{ n.clone() }</div>
            }
            <div>
                <span class={badge_class}>
                    { if c.verified { "Verified" } else { "Unverified" } }
                </span>
                if let Some(by) = c.verified_by {
                    { " " }
                    <span class="muted">{ format!("by {}", by) }</span>
                }
                if let Some(at) = c.verified_at {
                    { " " }
                    <span class="muted">{ format!("at {}", at) }</span>
                }
            </div>
        </div>
    }
}

// TODO: Add tests for simple render and Hours validation when test utilities exist.
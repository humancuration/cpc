// Volunteer Opportunity Form Component
// ADR 0008: Volunteer Coordination - This component is part of the volunteer coordination UI.
// TODO(GraphQL): Wire to api_server schema mutation: createOpportunity (see ADR 0008)

use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CreateOpportunityFormData {
    pub title: String,
    pub description: String,
    pub skills_needed: Vec<String>,
    pub location: String,
}

#[derive(Properties, PartialEq)]
pub struct OpportunityFormProps {
    pub on_submit: Callback<CreateOpportunityFormData>,
    #[prop_or_default]
    pub error_text: Option<String>,
}

#[styled_component(OpportunityForm)]
pub fn opportunity_form(props: &OpportunityFormProps) -> Html {
    let css = Style::new(
        r#"
        .form {
            display: flex;
            flex-direction: column;
            gap: 12px;
            padding: 12px;
            border: 1px solid #ddd;
            border-radius: 8px;
            background: #fafafa;
        }
        .row { display: flex; flex-direction: column; gap: 6px; }
        label { font-weight: 600; }
        input[type="text"], textarea {
            padding: 8px; border: 1px solid #ccc; border-radius: 6px;
            font-size: 14px;
        }
        .error { color: #b00020; font-size: 13px; }
        .actions { display: flex; gap: 8px; }
        button {
            padding: 8px 12px; border: none; border-radius: 6px; cursor: pointer;
            background: #2563eb; color: white; font-weight: 600;
        }
        button[disabled] { opacity: 0.6; cursor: not-allowed; }
    "#,
    )
    .expect("valid style");

    let title = use_state(|| String::new());
    let description = use_state(|| String::new());
    let skills_csv = use_state(|| String::new());
    let location = use_state(|| String::new());
    let submitting = use_state(|| false);
    let local_error = use_state(|| Option::<String>::None);

    // basic validation
    let validate = {
        let title = title.clone();
        let description = description.clone();
        Callback::from(move |_| {
            if title.trim().is_empty() {
                return Err("Title is required".to_string());
            }
            if description.trim().is_empty() {
                return Err("Description is required".to_string());
            }
            Ok(())
        })
    };

    let on_submit_click = {
        let title = title.clone();
        let description = description.clone();
        let skills_csv = skills_csv.clone();
        let location = location.clone();
        let submitting = submitting.clone();
        let local_error = local_error.clone();
        let on_submit = props.on_submit.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *submitting {
                return;
            }
            local_error.set(None);
            if let Err(msg) = validate.emit(()) {
                local_error.set(Some(msg));
                return;
            }
            submitting.set(true);

            let skills: Vec<String> = skills_csv
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let data = CreateOpportunityFormData {
                title: (*title).clone(),
                description: (*description).clone(),
                skills_needed: skills,
                location: (*location).clone(),
            };

            // Networking is decoupled; parent handles it. Here we just invoke callback.
            on_submit.emit(data);

            // Re-enable immediately for demo. In real use, parent should control submitting state.
            submitting.set(false);
        })
    };

    html! {
        <div class={css}>
            <form class="form">
                if let Some(err) = &*local_error {
                    <div class="error">{ err }</div>
                }
                if let Some(err) = &props.error_text {
                    <div class="error">{ err }</div>
                }
                <div class="row">
                    <label for="title">{ "Title" }</label>
                    <input
                        id="title"
                        type="text"
                        value={(*title).clone()}
                        oninput={{
                            let title = title.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    title.set(input.value());
                                }
                            })
                        }}
                        placeholder="e.g. Community Garden Helper"
                    />
                </div>

                <div class="row">
                    <label for="description">{ "Description" }</label>
                    <textarea
                        id="description"
                        value={(*description).clone()}
                        oninput={{
                            let description = description.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(el) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                                    description.set(el.value());
                                }
                            })
                        }}
                        placeholder="Describe the opportunity and expected duties..."
                        rows={6}
                    />
                </div>

                <div class="row">
                    <label for="skills">{ "Skills Needed (CSV)" }</label>
                    <input
                        id="skills"
                        type="text"
                        value={(*skills_csv).clone()}
                        oninput={{
                            let skills_csv = skills_csv.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    skills_csv.set(input.value());
                                }
                            })
                        }}
                        placeholder="e.g. gardening, composting, irrigation"
                    />
                </div>

                <div class="row">
                    <label for="location">{ "Location" }</label>
                    <input
                        id="location"
                        type="text"
                        value={(*location).clone()}
                        oninput={{
                            let location = location.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    location.set(input.value());
                                }
                            })
                        }}
                        placeholder="City, Region, or Remote"
                    />
                </div>

                <div class="actions">
                    <button onclick={on_submit_click} disabled={*submitting}>{ "Create Opportunity" }</button>
                </div>
            </form>
        </div>
    }
}

// Basic unit tests pattern: if the project has Yew tests, add them; otherwise TODOs.
// TODO: Add unit tests for validation: empty title/description produce errors.
use yew::prelude::*;
use crate::domain::models::Candidate;
use crate::application::candidate_service::CandidateService;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct CandidateProfileProps {
    pub candidate_service: CandidateService,
    pub candidate_id: Option<String>,
}

#[function_component(CandidateProfile)]
pub fn candidate_profile(props: &CandidateProfileProps) -> Html {
    let candidate = use_state(|| None);
    let is_editing = use_state(|| false);
    let headline = use_state(|| String::new());
    let summary = use_state(|| String::new());
    let location = use_state(|| String::new());
    let is_open_to_work = use_state(|| true);
    
    // Load candidate data if ID is provided
    {
        let candidate = candidate.clone();
        let candidate_service = Rc::new(RefCell::new(props.candidate_service.clone()));
        let candidate_id = props.candidate_id.clone();
        
        use_effect_with_deps(
            move |_| {
                // In a real implementation, this would load the candidate data
                // For now, we'll just set up the state
                || ()
            },
            (),
        );
    }
    
    let on_save = {
        let candidate_service = Rc::new(RefCell::new(props.candidate_service.clone()));
        let headline = headline.clone();
        let summary = summary.clone();
        let location = location.clone();
        let is_open_to_work = is_open_to_work.clone();
        
        Callback::from(move |_| {
            // In a real implementation, this would save the candidate profile
            // For now, we'll just show a placeholder
        })
    };
    
    let on_toggle_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(!*is_editing);
        })
    };
    
    html! {
        <div class="candidate-profile">
            <div class="profile-header">
                <h1>{"Candidate Profile"}</h1>
                <button onclick={on_toggle_edit}>
                    {if *is_editing { "Cancel" } else { "Edit" }}
                </button>
            </div>
            
            if *is_editing {
                <form onsubmit={on_save}>
                    <div class="form-group">
                        <label for="headline">{"Headline"}</label>
                        <input
                            type="text"
                            id="headline"
                            value={(*headline).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                headline.set(input.value());
                            })}
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="summary">{"Summary"}</label>
                        <textarea
                            id="summary"
                            value={(*summary).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                summary.set(input.value());
                            })}
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="location">{"Location"}</label>
                        <input
                            type="text"
                            id="location"
                            value={(*location).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                location.set(input.value());
                            })}
                        />
                    </div>
                    
                    <div class="form-group">
                        <label>
                            <input
                                type="checkbox"
                                checked={*is_open_to_work}
                                onchange={Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    is_open_to_work.set(input.checked());
                                })}
                            />
                            {"Open to work"}
                        </label>
                    </div>
                    
                    <button type="submit">{"Save Profile"}</button>
                </form>
            } else {
                if let Some(candidate) = candidate.as_ref() {
                    <div class="profile-view">
                        <h2>{candidate.headline.clone().unwrap_or_default()}</h2>
                        <p class="location">{candidate.location.clone().unwrap_or_default()}</p>
                        <div class="summary">
                            <h3>{"Summary"}</h3>
                            <p>{candidate.summary.clone().unwrap_or_default()}</p>
                        </div>
                        <div class="availability">
                            <strong>{"Availability: "}</strong>
                            {if candidate.is_open_to_work { "Open to work" } else { "Not available" }}
                        </div>
                    </div>
                } else {
                    <p>{"Loading profile..."}</p>
                }
            }
        </div>
    }
}
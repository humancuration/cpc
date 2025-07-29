use yew::prelude::*;
use crate::application::interview_service::InterviewService;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct InterviewSchedulerProps {
    pub interview_service: InterviewService,
    pub application_id: String,
}

#[function_component(InterviewScheduler)]
pub fn interview_scheduler(props: &InterviewSchedulerProps) -> Html {
    let interviews = use_state(|| vec![]);
    let show_schedule_form = use_state(|| false);
    let selected_date = use_state(|| String::new());
    let selected_time = use_state(|| String::new());
    let location = use_state(|| String::new());
    let notes = use_state(|| String::new());
    
    // Load interviews for application
    {
        let interviews = interviews.clone();
        let interview_service = Rc::new(RefCell::new(props.interview_service.clone()));
        let application_id = props.application_id.clone();
        
        use_effect_with_deps(
            move |_| {
                // In a real implementation, this would load the interviews
                // For now, we'll just set up the state
                || ()
            },
            (),
        );
    }
    
    let on_schedule_interview = {
        let interview_service = Rc::new(RefCell::new(props.interview_service.clone()));
        let show_schedule_form = show_schedule_form.clone();
        let selected_date = selected_date.clone();
        let selected_time = selected_time.clone();
        let location = location.clone();
        let notes = notes.clone();
        
        Callback::from(move |_| {
            // In a real implementation, this would schedule a new interview
            // For now, we'll just show a placeholder
            show_schedule_form.set(false);
        })
    };
    
    let on_toggle_schedule_form = {
        let show_schedule_form = show_schedule_form.clone();
        Callback::from(move |_| {
            show_schedule_form.set(!*show_schedule_form);
        })
    };
    
    html! {
        <div class="interview-scheduler">
            <div class="scheduler-header">
                <h1>{"Interview Scheduling"}</h1>
                <button onclick={on_toggle_schedule_form}>
                    {if *show_schedule_form { "Cancel" } else { "Schedule Interview" }}
                </button>
            </div>
            
            if *show_schedule_form {
                <div class="schedule-form">
                    <h2>{"Schedule New Interview"}</h2>
                    <form onsubmit={on_schedule_interview}>
                        <div class="form-group">
                            <label for="interview-date">{"Date"}</label>
                            <input
                                type="date"
                                id="interview-date"
                                value={(*selected_date).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    selected_date.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="interview-time">{"Time"}</label>
                            <input
                                type="time"
                                id="interview-time"
                                value={(*selected_time).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    selected_time.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="interview-location">{"Location/Meeting Link"}</label>
                            <input
                                type="text"
                                id="interview-location"
                                value={(*location).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    location.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="interview-notes">{"Notes"}</label>
                            <textarea
                                id="interview-notes"
                                value={(*notes).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                    notes.set(input.value());
                                })}
                            />
                        </div>
                        
                        <button type="submit">{"Schedule Interview"}</button>
                    </form>
                </div>
            }
            
            <div class="interviews-list">
                <h2>{"Scheduled Interviews"}</h2>
                if interviews.is_empty() {
                    <p>{"No interviews scheduled yet."}</p>
                } else {
                    <div class="interviews">
                        {for interviews.iter().map(|interview| {
                            html! {
                                <div class="interview-item">
                                    <h3>{"Interview"}</h3>
                                    <p>{"Date: "}{interview.scheduled_time.to_rfc3339()}</p>
                                    <p>{"Location: "}{interview.location.clone().unwrap_or_default()}</p>
                                    if let Some(notes) = &interview.notes {
                                        <p>{"Notes: "}{notes}</p>
                                    }
                                    <div class="interview-actions">
                                        <button>{"Edit"}</button>
                                        <button>{"Cancel"}</button>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}
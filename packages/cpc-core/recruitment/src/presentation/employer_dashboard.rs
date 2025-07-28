use yew::prelude::*;
use crate::domain::models::{Job, Application};
use crate::application::job_service::JobService;
use crate::application::application_service::ApplicationService;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct EmployerDashboardProps {
    pub job_service: JobService,
    pub application_service: ApplicationService,
    pub employer_id: String,
}

#[function_component(EmployerDashboard)]
pub fn employer_dashboard(props: &EmployerDashboardProps) -> Html {
    let jobs = use_state(|| vec![]);
    let selected_job = use_state(|| None);
    let applications = use_state(|| vec![]);
    let show_create_form = use_state(|| false);
    let new_job_title = use_state(|| String::new());
    let new_job_description = use_state(|| String::new());
    let new_job_location = use_state(|| String::new());
    let new_job_remote = use_state(|| false);
    
    // Load jobs for employer
    {
        let jobs = jobs.clone();
        let job_service = Rc::new(RefCell::new(props.job_service.clone()));
        let employer_id = props.employer_id.clone();
        
        use_effect_with_deps(
            move |_| {
                // In a real implementation, this would load the employer's jobs
                // For now, we'll just set up the state
                || ()
            },
            (),
        );
    }
    
    let on_create_job = {
        let job_service = Rc::new(RefCell::new(props.job_service.clone()));
        let show_create_form = show_create_form.clone();
        let new_job_title = new_job_title.clone();
        let new_job_description = new_job_description.clone();
        let new_job_location = new_job_location.clone();
        let new_job_remote = new_job_remote.clone();
        
        Callback::from(move |_| {
            // In a real implementation, this would create a new job
            // For now, we'll just show a placeholder
            show_create_form.set(false);
        })
    };
    
    let on_select_job = {
        let selected_job = selected_job.clone();
        let applications = applications.clone();
        let application_service = Rc::new(RefCell::new(props.application_service.clone()));
        
        Callback::from(move |job: Job| {
            selected_job.set(Some(job.clone()));
            
            // In a real implementation, this would load applications for the job
            // For now, we'll just clear the applications
            applications.set(vec![]);
        })
    };
    
    let on_toggle_create_form = {
        let show_create_form = show_create_form.clone();
        Callback::from(move |_| {
            show_create_form.set(!*show_create_form);
        })
    };
    
    html! {
        <div class="employer-dashboard">
            <div class="dashboard-header">
                <h1>{"Employer Dashboard"}</h1>
                <button onclick={on_toggle_create_form}>
                    {if *show_create_form { "Cancel" } else { "Post New Job" }}
                </button>
            </div>
            
            if *show_create_form {
                <div class="create-job-form">
                    <h2>{"Create New Job"}</h2>
                    <form onsubmit={on_create_job}>
                        <div class="form-group">
                            <label for="job-title">{"Job Title"}</label>
                            <input
                                type="text"
                                id="job-title"
                                value={(*new_job_title).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    new_job_title.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="job-description">{"Description"}</label>
                            <textarea
                                id="job-description"
                                value={(*new_job_description).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                    new_job_description.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="job-location">{"Location"}</label>
                            <input
                                type="text"
                                id="job-location"
                                value={(*new_job_location).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    new_job_location.set(input.value());
                                })}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label>
                                <input
                                    type="checkbox"
                                    checked={*new_job_remote}
                                    onchange={Callback::from(move |e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        new_job_remote.set(input.checked());
                                    })}
                                />
                                {"Remote Position"}
                            </label>
                        </div>
                        
                        <button type="submit">{"Post Job"}</button>
                    </form>
                </div>
            }
            
            <div class="dashboard-content">
                <div class="jobs-list">
                    <h2>{"Your Job Postings"}</h2>
                    <div class="jobs">
                        {for jobs.iter().map(|job| {
                            let job_clone = job.clone();
                            let on_select = on_select_job.clone();
                            html! {
                                <div 
                                    class="job-item" 
                                    onclick={move |_| on_select.emit(job_clone.clone())}
                                >
                                    <h3>{&job.title}</h3>
                                    <p>{&job.location.clone().unwrap_or_default()}</p>
                                    <span class="status">{format!("{:?}", job.status)}</span>
                                </div>
                            }
                        })}
                    </div>
                </div>
                
                if let Some(job) = selected_job.as_ref() {
                    <div class="applications-section">
                        <h2>{format!("Applications for {}", job.title)}</h2>
                        <div class="applications">
                            {for applications.iter().map(|app| {
                                html! {
                                    <div class="application-item">
                                        <h4>{"Candidate Name"}</h4>
                                        <p>{"Application status: "}{format!("{:?}", app.status)}</p>
                                        <button>{"View Details"}</button>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
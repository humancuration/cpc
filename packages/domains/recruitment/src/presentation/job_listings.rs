use yew::prelude::*;
use crate::domain::models::Job;
use crate::application::job_service::JobService;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct JobListingsProps {
    pub job_service: JobService,
}

#[function_component(JobListings)]
pub fn job_listings(props: &JobListingsProps) -> Html {
    let jobs = use_state(|| vec![]);
    let search_term = use_state(|| String::new());
    let location = use_state(|| String::new());
    let employment_type = use_state(|| String::new());
    let is_remote = use_state(|| None);
    
    let jobs_clone = jobs.clone();
    let job_service = Rc::new(RefCell::new(props.job_service.clone()));
    
    let on_search = {
        let jobs = jobs_clone.clone();
        let search_term = search_term.clone();
        let location = location.clone();
        let employment_type = employment_type.clone();
        let is_remote = is_remote.clone();
        
        Callback::from(move |_| {
            let jobs = jobs.clone();
            let search_term = search_term.clone();
            let location = location.clone();
            let employment_type = employment_type.clone();
            let is_remote = is_remote.clone();
            
            // In a real implementation, this would call the job service
            // For now, we'll just show a placeholder
            jobs.set(vec![]);
        })
    };
    
    html! {
        <div class="job-listings">
            <h1>{"Job Listings"}</h1>
            
            <div class="search-filters">
                <input
                    type="text"
                    placeholder="Search jobs..."
                    value={(*search_term).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        search_term.set(input.value());
                    })}
                />
                
                <input
                    type="text"
                    placeholder="Location"
                    value={(*location).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        location.set(input.value());
                    })}
                />
                
                <select
                    value={(*employment_type).clone()}
                    onchange={Callback::from(move |e: Event| {
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        employment_type.set(select.value());
                    })}
                >
                    <option value="">{"All Employment Types"}</option>
                    <option value="full_time">{"Full Time"}</option>
                    <option value="part_time">{"Part Time"}</option>
                    <option value="contract">{"Contract"}</option>
                    <option value="internship">{"Internship"}</option>
                </select>
                
                <label>
                    <input
                        type="checkbox"
                        checked={is_remote.as_ref().map(|b| *b).unwrap_or(false)}
                        onchange={Callback::from(move |e: Event| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            is_remote.set(Some(input.checked()));
                        })}
                    />
                    {"Remote Only"}
                </label>
                
                <button onclick={on_search}>{"Search"}</button>
            </div>
            
            <div class="job-results">
                {for jobs.iter().map(|job| html! { <JobCard job={job.clone()} /> })}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct JobCardProps {
    pub job: Job,
}

#[function_component(JobCard)]
pub fn job_card(props: &JobCardProps) -> Html {
    html! {
        <div class="job-card">
            <h2>{&props.job.title}</h2>
            <p class="company">{/* Company name would be fetched from employer */}</p>
            <p class="location">
                {&props.job.location.clone().unwrap_or_default()}
                {if props.job.is_remote { " (Remote)" } else { "" }}
            </p>
            <p class="description">{&props.job.description}</p>
            <div class="job-meta">
                <span class="employment-type">
                    {format!("{:?}", props.job.employment_type)}
                </span>
                // Salary range would be displayed here
            </div>
            <button>{"Apply"}</button>
        </div>
    }
}
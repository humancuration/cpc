use yew::prelude::*;
use crate::domain::models::Application;
use crate::application::application_service::ApplicationService;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct ApplicationTrackerProps {
    pub application_service: ApplicationService,
    pub user_id: String,
}

#[function_component(ApplicationTracker)]
pub fn application_tracker(props: &ApplicationTrackerProps) -> Html {
    let applications = use_state(|| vec![]);
    let selected_application = use_state(|| None);
    
    // Load applications for user
    {
        let applications = applications.clone();
        let application_service = Rc::new(RefCell::new(props.application_service.clone()));
        let user_id = props.user_id.clone();
        
        use_effect_with_deps(
            move |_| {
                // In a real implementation, this would load the user's applications
                // For now, we'll just set up the state
                || ()
            },
            (),
        );
    }
    
    let on_select_application = {
        let selected_application = selected_application.clone();
        Callback::from(move |app: Application| {
            selected_application.set(Some(app));
        })
    };
    
    let on_withdraw_application = {
        let application_service = Rc::new(RefCell::new(props.application_service.clone()));
        let selected_application = selected_application.clone();
        let applications = applications.clone();
        
        Callback::from(move |_| {
            // In a real implementation, this would withdraw the application
            // For now, we'll just show a placeholder
        })
    };
    
    html! {
        <div class="application-tracker">
            <h1>{"My Applications"}</h1>
            
            <div class="applications-content">
                <div class="applications-list">
                    <h2>{"Submitted Applications"}</h2>
                    if applications.is_empty() {
                        <p>{"You haven't submitted any applications yet."}</p>
                    } else {
                        <div class="applications">
                            {for applications.iter().map(|app| {
                                let app_clone = app.clone();
                                let on_select = on_select_application.clone();
                                html! {
                                    <div 
                                        class="application-item" 
                                        onclick={move |_| on_select.emit(app_clone.clone())}
                                    >
                                        <h3>{"Job Title"}</h3>
                                        <p>{"Company Name"}</p>
                                        <span class="status">{format!("{:?}", app.status)}</span>
                                        <p class="date">{app.created_at.to_rfc3339()}</p>
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
                
                if let Some(application) = selected_application.as_ref() {
                    <div class="application-details">
                        <h2>{"Application Details"}</h2>
                        <div class="job-info">
                            <h3>{"Job Title"}</h3>
                            <p>{"Company Name"}</p>
                            <p>{"Location"}</p>
                        </div>
                        
                        <div class="application-info">
                            <h3>{"Application Status"}</h3>
                            <p>{format!("{:?}", application.status)}</p>
                            
                            if let Some(cover_letter) = &application.cover_letter {
                                <div class="cover-letter">
                                    <h3>{"Cover Letter"}</h3>
                                    <p>{cover_letter}</p>
                                </div>
                            }
                        </div>
                        
                        <button 
                            class="withdraw-button"
                            onclick={on_withdraw_application}
                        >
                            {"Withdraw Application"}
                        </button>
                    </div>
                }
            </div>
        </div>
    }
}
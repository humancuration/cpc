use yew::prelude::*;
use yew::platform::spawn_local;
use uuid::Uuid;
use crate::types::SkillProgressData;
use crate::services::grpc_client::SkillDevelopmentClient;

#[function_component(ProgressTracker)]
pub fn progress_tracker() -> Html {
    let skills = use_state(|| Vec::<SkillProgressData>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let skills = skills.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            let skills = skills.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                // In a real app, we would get the current user ID from context
                let user_id = Uuid::new_v4();
                
                // Create gRPC client
                let mut client = match SkillDevelopmentClient::new("http://localhost:50051".to_string()).await {
                    Ok(client) => client,
                    Err(e) => {
                        error.set(Some(format!("Failed to connect to server: {}", e)));
                        loading.set(false);
                        return;
                    }
                };

                // Call gRPC service
                match client.get_user_skill_progress(user_id).await {
                    Ok(skills_data) => {
                        skills.set(skills_data);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch skill progress: {}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Skill Progress"}</h2>
                <button class="btn btn-primary">{"Add Skill"}</button>
            </div>
            if *loading {
                <div class="card-body">
                    <div class="text-center">
                        <div class="spinner-border" role="status">
                            <span class="sr-only">{"Loading..."}</span>
                        </div>
                    </div>
                </div>
            } else if let Some(err) = &*error {
                <div class="card-body">
                    <div class="alert alert-danger">
                        {err}
                    </div>
                </div>
            } else {
                <div class="skill-list">
                    {for skills.iter().map(|skill| html! {
                        <div class="skill-item">
                            <div class="skill-info">
                                <div class="skill-name">{&skill.skill_name}</div>
                                <div class="skill-level">
                                    {format!("{} → {}",
                                        format!("{:?}", skill.current_level),
                                        format!("{:?}", skill.target_level)
                                    )}
                                </div>
                                <div class="progress-bar">
                                    <div class="progress-fill" style={format!("width: {}%", skill.progress_percentage)}></div>
                                </div>
                                <div>{format!("{} hours invested", skill.total_hours_invested)}</div>
                            </div>
                            <button class="btn btn-secondary">{"Update"}</button>
                        </div>
                    })}
                </div>
            }
        </div>
    }
}
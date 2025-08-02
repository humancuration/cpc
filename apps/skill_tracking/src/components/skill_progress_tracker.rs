use yew::prelude::*;
use yew::platform::spawn_local;
use uuid::Uuid;
use crate::types::{SkillProgressData, SkillLevel};
use crate::services::grpc_client::SkillDevelopmentClient;

#[derive(Properties, PartialEq)]
pub struct SkillProgressTrackerProps {
    pub on_progress_updated: Callback<SkillProgressData>,
}

#[function_component(SkillProgressTracker)]
pub fn skill_progress_tracker(props: &SkillProgressTrackerProps) -> Html {
    let skill_name = use_state(|| String::new());
    let current_level = use_state(|| SkillLevel::Beginner);
    let target_level = use_state(|| SkillLevel::Intermediate);
    let progress_percentage = use_state(|| 0.0_f32);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_skill_name_change = {
        let skill_name = skill_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            skill_name.set(input.value());
        })
    };

    let on_current_level_change = {
        let current_level = current_level.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            let level = match value.as_str() {
                "beginner" => SkillLevel::Beginner,
                "intermediate" => SkillLevel::Intermediate,
                "advanced" => SkillLevel::Advanced,
                "expert" => SkillLevel::Expert,
                "master" => SkillLevel::Master,
                _ => SkillLevel::Beginner,
            };
            current_level.set(level);
        })
    };

    let on_target_level_change = {
        let target_level = target_level.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            let level = match value.as_str() {
                "beginner" => SkillLevel::Beginner,
                "intermediate" => SkillLevel::Intermediate,
                "advanced" => SkillLevel::Advanced,
                "expert" => SkillLevel::Expert,
                "master" => SkillLevel::Master,
                _ => SkillLevel::Intermediate,
            };
            target_level.set(level);
        })
    };

    let on_progress_change = {
        let progress_percentage = progress_percentage.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f32>() {
                progress_percentage.set(value.min(100.0).max(0.0));
            }
        })
    };

    let on_submit = {
        let skill_name = skill_name.clone();
        let current_level = current_level.clone();
        let target_level = target_level.clone();
        let progress_percentage = progress_percentage.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_progress_updated = props.on_progress_updated.clone();

        Callback::from(move |_| {
            let skill_name = skill_name.clone();
            let current_level = current_level.clone();
            let target_level = target_level.clone();
            let progress_percentage = progress_percentage.clone();
            let error = error.clone();
            let loading = loading.clone();
            let on_progress_updated = on_progress_updated.clone();

            spawn_local(async move {
                // Validate input
                if skill_name.is_empty() {
                    error.set(Some("Skill name is required".to_string()));
                    return;
                }

                loading.set(true);
                error.set(None);

                // In a real app, we would get the current user ID and skill ID from context
                let user_id = Uuid::new_v4();
                let skill_id = Uuid::new_v4();
                
                // Convert levels to i32 for gRPC
                let current_level_int = match *current_level {
                    SkillLevel::Beginner => 0,
                    SkillLevel::Intermediate => 1,
                    SkillLevel::Advanced => 2,
                    SkillLevel::Expert => 3,
                    SkillLevel::Master => 4,
                };

                let target_level_int = match *target_level {
                    SkillLevel::Beginner => 0,
                    SkillLevel::Intermediate => 1,
                    SkillLevel::Advanced => 2,
                    SkillLevel::Expert => 3,
                    SkillLevel::Master => 4,
                };

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
                match client.track_skill_progress(
                    user_id,
                    skill_id,
                    current_level_int,
                    target_level_int,
                ).await {
                    Ok(progress_data) => {
                        loading.set(false);
                        on_progress_updated.emit(progress_data);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to track skill progress: {}", e)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Track Skill Progress"}</h2>
            </div>
            <div class="card-body">
                if let Some(err) = &*error {
                    <div class="alert alert-danger">
                        {err}
                    </div>
                }
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="skill-name">{"Skill Name"}</label>
                        <input
                            type="text"
                            id="skill-name"
                            class="form-control"
                            value={(*skill_name).clone()}
                            oninput={on_skill_name_change}
                            placeholder="Enter skill name"
                        />
                    </div>
                    <div class="form-row">
                        <div class="form-group col-md-6">
                            <label for="current-level">{"Current Level"}</label>
                            <select
                                id="current-level"
                                class="form-control"
                                onchange={on_current_level_change}
                                value={match *current_level {
                                    SkillLevel::Beginner => "beginner",
                                    SkillLevel::Intermediate => "intermediate",
                                    SkillLevel::Advanced => "advanced",
                                    SkillLevel::Expert => "expert",
                                    SkillLevel::Master => "master",
                                }}
                            >
                                <option value="beginner">{"Beginner"}</option>
                                <option value="intermediate">{"Intermediate"}</option>
                                <option value="advanced">{"Advanced"}</option>
                                <option value="expert">{"Expert"}</option>
                                <option value="master">{"Master"}</option>
                            </select>
                        </div>
                        <div class="form-group col-md-6">
                            <label for="target-level">{"Target Level"}</label>
                            <select
                                id="target-level"
                                class="form-control"
                                onchange={on_target_level_change}
                                value={match *target_level {
                                    SkillLevel::Beginner => "beginner",
                                    SkillLevel::Intermediate => "intermediate",
                                    SkillLevel::Advanced => "advanced",
                                    SkillLevel::Expert => "expert",
                                    SkillLevel::Master => "master",
                                }}
                            >
                                <option value="beginner">{"Beginner"}</option>
                                <option value="intermediate">{"Intermediate"}</option>
                                <option value="advanced">{"Advanced"}</option>
                                <option value="expert">{"Expert"}</option>
                                <option value="master">{"Master"}</option>
                            </select>
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="progress">{"Progress Percentage"}</label>
                        <input
                            type="number"
                            id="progress"
                            class="form-control"
                            value={format!("{}", *progress_percentage)}
                            oninput={on_progress_change}
                            min="0"
                            max="100"
                            step="0.1"
                        />
                        <div class="progress mt-2">
                            <div 
                                class="progress-bar" 
                                role="progressbar" 
                                style={format!("width: {}%", *progress_percentage)}
                                aria-valuenow={*progress_percentage as u32}
                                aria-valuemin="0"
                                aria-valuemax="100"
                            >
                                {format!("{}%", *progress_percentage)}
                            </div>
                        </div>
                    </div>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={*loading}
                    >
                        if *loading {
                            <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                            {" Updating..."}
                        } else {
                            {"Update Progress"}
                        }
                    </button>
                </form>
            </div>
        </div>
    }
}
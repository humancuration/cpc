use yew::prelude::*;
use yew::platform::spawn_local;
use uuid::Uuid;
use crate::types::{LearningPathData, DifficultyLevel};
use crate::services::grpc_client::SkillDevelopmentClient;

#[derive(Properties, PartialEq)]
pub struct LearningPathCreatorProps {
    pub on_path_created: Callback<LearningPathData>,
}

#[function_component(LearningPathCreator)]
pub fn learning_path_creator(props: &LearningPathCreatorProps) -> Html {
    let title = use_state(|| String::new());
    let description = use_state(|| String::new());
    let difficulty = use_state(|| DifficultyLevel::Beginner);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let on_difficulty_change = {
        let difficulty = difficulty.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            let level = match value.as_str() {
                "beginner" => DifficultyLevel::Beginner,
                "intermediate" => DifficultyLevel::Intermediate,
                "advanced" => DifficultyLevel::Advanced,
                _ => DifficultyLevel::Beginner,
            };
            difficulty.set(level);
        })
    };

    let on_submit = {
        let title = title.clone();
        let description = description.clone();
        let difficulty = difficulty.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_path_created = props.on_path_created.clone();

        Callback::from(move |_| {
            let title = title.clone();
            let description = description.clone();
            let difficulty = difficulty.clone();
            let error = error.clone();
            let loading = loading.clone();
            let on_path_created = on_path_created.clone();

            spawn_local(async move {
                // Validate input
                if title.is_empty() {
                    error.set(Some("Title is required".to_string()));
                    return;
                }

                loading.set(true);
                error.set(None);

                // In a real app, we would get the current user ID from context
                let creator_id = Uuid::new_v4();
                
                // Convert difficulty level to i32 for gRPC
                let difficulty_level = match *difficulty {
                    DifficultyLevel::Beginner => 0,
                    DifficultyLevel::Intermediate => 1,
                    DifficultyLevel::Advanced => 2,
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
                match client.create_learning_path(
                    title.to_string(),
                    description.to_string(),
                    creator_id,
                    difficulty_level,
                ).await {
                    Ok(path_data) => {
                        loading.set(false);
                        on_path_created.emit(path_data);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create learning path: {}", e)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Create Learning Path"}</h2>
            </div>
            <div class="card-body">
                if let Some(err) = &*error {
                    <div class="alert alert-danger">
                        {err}
                    </div>
                }
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="title">{"Title"}</label>
                        <input
                            type="text"
                            id="title"
                            class="form-control"
                            value={(*title).clone()}
                            oninput={on_title_change}
                            placeholder="Enter learning path title"
                        />
                    </div>
                    <div class="form-group">
                        <label for="description">{"Description"}</label>
                        <textarea
                            id="description"
                            class="form-control"
                            value={(*description).clone()}
                            oninput={on_description_change}
                            placeholder="Enter learning path description"
                            rows="3"
                        />
                    </div>
                    <div class="form-group">
                        <label for="difficulty">{"Difficulty Level"}</label>
                        <select
                            id="difficulty"
                            class="form-control"
                            onchange={on_difficulty_change}
                            value={match *difficulty {
                                DifficultyLevel::Beginner => "beginner",
                                DifficultyLevel::Intermediate => "intermediate",
                                DifficultyLevel::Advanced => "advanced",
                            }}
                        >
                            <option value="beginner">{"Beginner"}</option>
                            <option value="intermediate">{"Intermediate"}</option>
                            <option value="advanced">{"Advanced"}</option>
                        </select>
                    </div>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={*loading}
                    >
                        if *loading {
                            <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                            {" Creating..."}
                        } else {
                            {"Create Learning Path"}
                        }
                    </button>
                </form>
            </div>
        </div>
    }
}
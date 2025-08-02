use yew::prelude::*;
use crate::types::LearningPathData;

#[function_component(LearningPathVisualizer)]
pub fn learning_path_visualizer() -> Html {
    let paths = use_state(|| vec![
        LearningPathData {
            id: uuid::Uuid::new_v4(),
            title: "Full-Stack Developer".to_string(),
            description: "Complete path to becoming a full-stack developer".to_string(),
            estimated_duration_hours: 200,
            difficulty_level: crate::types::DifficultyLevel::Intermediate,
            progress: 45.0,
        },
        LearningPathData {
            id: uuid::Uuid::new_v4(),
            title: "Data Science Fundamentals".to_string(),
            description: "Learn data science from basics to advanced concepts".to_string(),
            estimated_duration_hours: 150,
            difficulty_level: crate::types::DifficultyLevel::Beginner,
            progress: 20.0,
        },
    ]);

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Learning Paths"}</h2>
                <button class="btn btn-primary">{"Create Path"}</button>
            </div>
            <div class="grid grid-cols-2">
                {for paths.iter().map(|path| html! {
                    <div class="card">
                        <h3>{&path.title}</h3>
                        <p>{&path.description}</p>
                        <div class="progress-bar">
                            <div class="progress-fill" style={format!("width: {}%", path.progress)}></div>
                        </div>
                        <div class="grid">
                            <span>{format!("{} hours estimated", path.estimated_duration_hours)}</span>
                            <span>{format!("{:?} difficulty", path.difficulty_level)}</span>
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}
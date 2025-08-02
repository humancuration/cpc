use yew::prelude::*;
use crate::components::{
    ProgressTracker, 
    CertificationDisplay, 
    LearningPathCreator, 
    SkillProgressTracker,
    LearningPathVisualizer
};
use crate::types::{SkillProgressData, LearningPathData, CertificationData};

#[function_component(MainDashboard)]
pub fn main_dashboard() -> Html {
    let skills = use_state(|| Vec::<SkillProgressData>::new());
    let certifications = use_state(|| Vec::<CertificationData>::new());
    let learning_paths = use_state(|| Vec::<LearningPathData>::new());

    let on_path_created = {
        let learning_paths = learning_paths.clone();
        Callback::from(move |new_path: LearningPathData| {
            let mut paths = (*learning_paths).clone();
            paths.push(new_path);
            learning_paths.set(paths);
        })
    };

    let on_progress_updated = {
        let skills = skills.clone();
        Callback::from(move |updated_progress: SkillProgressData| {
            let mut updated_skills = (*skills).clone();
            // In a real app, we would update the existing skill or add a new one
            updated_skills.push(updated_progress);
            skills.set(updated_skills);
        })
    };

    html! {
        <div class="container-fluid">
            <div class="row">
                <div class="col-12">
                    <h1>{"Skill Development Dashboard"}</h1>
                </div>
            </div>
            
            <div class="row">
                <div class="col-md-6">
                    <SkillProgressTracker on_progress_updated={on_progress_updated.clone()} />
                </div>
                <div class="col-md-6">
                    <LearningPathCreator on_path_created={on_path_created.clone()} />
                </div>
            </div>
            
            <div class="row mt-4">
                <div class="col-12">
                    <ProgressTracker />
                </div>
            </div>
            
            <div class="row mt-4">
                <div class="col-md-6">
                    <CertificationDisplay />
                </div>
                <div class="col-md-6">
                    <LearningPathVisualizer />
                </div>
            </div>
        </div>
    }
}
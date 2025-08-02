use yew::prelude::*;

use crate::components::{ProgressTracker, LearningPathVisualizer, CertificationDisplay};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="container">
            <div class="dashboard">
                <ProgressTracker />
                <LearningPathVisualizer />
                <CertificationDisplay />
            </div>
        </div>
    }
}
use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;

use crate::api::project::{get_project, ProjectDetail as Project};
use crate::project::routing::ProjectRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectDetailProps {
    pub id: Uuid,
}

#[function_component(ProjectDetail)]
pub fn project_detail(props: &ProjectDetailProps) -> Html {
    let project = use_state(|| None);
    let error = use_state(|| None);

    {
        let project = project.clone();
        let error = error.clone();
        let props_id = props.id;

        use_effect_with(props.id, move |_| {
            let project = project.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_project(props_id).await {
                    Ok(fetched_project) => {
                        project.set(fetched_project);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="project-detail-container">
            {
                if let Some(error) = &*error {
                    html! { <p class="error-message">{ format!("Error: {}", error) }</p> }
                } else if let Some(project) = &*project {
                    html! {
                        <div class="project-detail">
                            <h1>{ &project.name }</h1>
                            <p>{ project.description.as_deref().unwrap_or("No description available") }</p>
                            <div class="field is-grouped">
                                <div class="control">
                                    <Link<ProjectRoute> to={ProjectRoute::Edit { id: project.id }} classes="button is-primary">
                                        { "Edit" }
                                    </Link<ProjectRoute>>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! { <p>{ "Loading project..." }</p> }
                }
            }
        </div>
    }
}
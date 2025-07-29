use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::project::{list_projects, Project};
use crate::project::routing::ProjectRoute;

#[function_component(ProjectListComponent)]
pub fn project_list_component() -> Html {
    let projects = use_state(|| None);
    let error = use_state(|| None);

    {
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let projects = projects.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match list_projects().await {
                    Ok(mut fetched_projects) => {
                        fetched_projects.sort_by(|a, b| a.name.cmp(&b.name));
                        projects.set(Some(fetched_projects));
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
        <div class="project-list-container">
            <h1>{ "Projects" }</h1>
            <Link<ProjectRoute> to={ProjectRoute::Create}>
                <button>{ "Create New Project" }</button>
            </Link<ProjectRoute>>
            {
                if let Some(error) = &*error {
                    html! { <p class="error-message">{ format!("Error: {}", error) }</p> }
                } else if let Some(projects) = &*projects {
                    html! {
                        <ul class="project-list">
                            { for projects.iter().map(|project| html! {
                                <li key={project.id.clone()} class="project-item">
                                    <Link<ProjectRoute> to={ProjectRoute::Detail { id: project.id.parse().unwrap() }}>
                                        <h2>{ &project.name }</h2>
                                        <p>{ project.description.as_deref().unwrap_or("No description available") }</p>
                                    </Link<ProjectRoute>>
                                </li>
                            })}
                        </ul>
                    }
                } else {
                    html! { <p>{ "Loading projects..." }</p> }
                }
            }
        </div>
    }
}
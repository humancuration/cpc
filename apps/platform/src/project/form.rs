use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;
use web_sys::HtmlInputElement;

use crate::api::project::{create_project, get_project, update_project, CreateProjectInput, UpdateProjectInput};
use crate::cooperative::components::selector::CooperativeSelector;
use crate::project::routing::ProjectRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectFormProps {
    pub project_id: Option<Uuid>,
}

#[function_component(ProjectFormComponent)]
pub fn project_form_component(props: &ProjectFormProps) -> Html {
    let name_state = use_state(String::new);
    let description_state = use_state(String::new);
    let cooperative_id_state = use_state(|| None::<Uuid>);
    let error_state = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let is_edit_mode = props.project_id.is_some();

    {
        let name_state = name_state.clone();
        let description_state = description_state.clone();
        let project_id = props.project_id;

        use_effect_with(project_id, move |&project_id| {
            if let Some(id) = project_id {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_project(id).await {
                        Ok(Some(project)) => {
                            name_state.set(project.name);
                            description_state.set(project.description.unwrap_or_default());
                        }
                        _ => { /* Handle error or not found */ }
                    }
                });
            }
            || ()
        });
    }

    let on_cooperative_select = {
        let cooperative_id_state = cooperative_id_state.clone();
        Callback::from(move |cooperative_id: Option<Uuid>| {
            cooperative_id_state.set(cooperative_id);
        })
    };

    let onsubmit = {
        let name_state = name_state.clone();
        let description_state = description_state.clone();
        let cooperative_id_state = cooperative_id_state.clone();
        let error_state = error_state.clone();
        let navigator = navigator.clone();
        let project_id = props.project_id;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error_state.set(None);

            let name = (*name_state).clone();
            let description = (*description_state).clone();

            if name.is_empty() {
                error_state.set(Some("Name is required".to_string()));
                return;
            }

            let navigator = navigator.clone();
            let error_state = error_state.clone();

            if let Some(id) = project_id {
                // Update existing project
                wasm_bindgen_futures::spawn_local(async move {
                    let input = UpdateProjectInput {
                        id,
                        name: Some(name),
                        description: Some(description),
                    };
                    match update_project(input).await {
                        Ok(_) => navigator.push(&ProjectRoute::Detail { id }),
                        Err(e) => error_state.set(Some(e.to_string())),
                    }
                });
            } else {
                // Create new project
                let cooperative_id = match *cooperative_id_state {
                    Some(id) => id,
                    None => {
                        error_state.set(Some("Please select a cooperative".to_string()));
                        return;
                    }
                };

                wasm_bindgen_futures::spawn_local(async move {
                    let input = CreateProjectInput {
                        name,
                        description: Some(description),
                        cooperative_id,
                    };
                    match create_project(input).await {
                        Ok(data) => {
                            if let Some(project) = data.create_project {
                                let new_id = Uuid::parse_str(&project.id).unwrap();
                                navigator.push(&ProjectRoute::Detail { id: new_id })
                            }
                        },
                        Err(e) => error_state.set(Some(e.to_string())),
                    }
                });
            }
        })
    };
    
    let on_name_change = {
        let name_state = name_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                name_state.set(input.value());
            }
        })
    };
    let on_description_change = {
        let description_state = description_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                description_state.set(input.value());
            }
        })
    };

    html! {
        <div class="container">
            <h1 class="title">{ if is_edit_mode { "Edit Project" } else { "Create Project" } }</h1>

            <form onsubmit={onsubmit}>
                <div class="field">
                    <label class="label">{ "Name" }</label>
                    <div class="control">
                        <input class="input" type="text" value={(*name_state).clone()} onchange={on_name_change} required=true />
                    </div>
                </div>

                <div class="field">
                    <label class="label">{ "Description" }</label>
                    <div class="control">
                        <textarea class="textarea" value={(*description_state).clone()} onchange={on_description_change}></textarea>
                    </div>
                </div>

                if !is_edit_mode {
                    <CooperativeSelector on_select={on_cooperative_select} />
                }

                if let Some(error) = &*error_state {
                    <p class="has-text-danger">{ error }</p>
                }
                
                <div class="field is-grouped">
                    <div class="control">
                        <button class="button is-primary" type="submit">
                            { if is_edit_mode { "Update" } else { "Create" } }
                        </button>
                    </div>
                    <div class="control">
                        <Link<ProjectRoute> to={ProjectRoute::List} classes="button is-light">
                            { "Cancel" }
                        </Link<ProjectRoute>>
                    </div>
                </div>
            </form>
        </div>
    }
}
use yew::prelude::*;
use uuid::Uuid;
use crate::api::cooperative::{get_cooperative, get_cooperative::GetCooperativeCooperative};
use crate::api::project::{get_projects, get_projects::GetProjectsProjects};

#[derive(Properties, PartialEq, Clone)]
pub struct CooperativeDetailProps {
    pub id: Uuid,
}

#[function_component(CooperativeDetail)]
pub fn cooperative_detail(props: &CooperativeDetailProps) -> Html {
    let cooperative = use_state(|| None);
    let projects = use_state(Vec::new);
    let error = use_state(|| None);

    let id = props.id;

    {
        let cooperative = cooperative.clone();
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with_deps(move |id| {
            let cooperative = cooperative.clone();
            let projects = projects.clone();
            let error = error.clone();
            let id = *id;
            wasm_bindgen_futures::spawn_local(async move {
                match get_cooperative(id).await {
                    Ok(response_data) => {
                        if let Some(fetched_cooperative) = response_data.cooperative {
                            let coop_id_for_projects = fetched_cooperative.numeric_id;
                            cooperative.set(Some(fetched_cooperative));

                            match get_projects(coop_id_for_projects).await {
                                Ok(projs) => {
                                    projects.set(projs);
                                }
                                Err(e) => {
                                    let mut current_error = (*error).clone().unwrap_or_default();
                                    if !current_error.is_empty() {
                                        current_error.push_str("\n");
                                    }
                                    current_error.push_str(&format!("Error fetching projects: {}", e));
                                    error.set(Some(current_error));
                                }
                            }
                        } else {
                            error.set(Some("Cooperative not found".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
            });
            || ()
        }, id);
    }

    let render_projects_list = |projects: &[GetProjectsProjects]| -> Html {
        if projects.is_empty() {
            html! { <p>{ "No projects found for this cooperative." }</p> }
        } else {
            html! {
                <div class="projects-list">
                    <h3>{ "Projects" }</h3>
                    <ul>
                        { for projects.iter().map(|project| html! {
                            <li>
                                <h4>{ project.title.clone() }</h4>
                                <p>{ project.description.clone().unwrap_or_default() }</p>
                            </li>
                        })}
                    </ul>
                </div>
            }
        }
    };

    let render_cooperative = |cooperative: &GetCooperativeCooperative, projects: &[GetProjectsProjects]| -> Html {
        let members_list = if let Some(members) = &cooperative.members {
            if members.is_empty() {
                html! { <p>{ "No members found for this cooperative." }</p> }
            } else {
                html! {
                    <div class="members-list">
                        <h2>{ "Members" }</h2>
                        <ul>
                            { for members.iter().map(|member| html! {
                                <li>
                                    <p><b>{ "Name: " }</b>{ &member.user.name }</p>
                                    <p><b>{ "Role: " }</b>{ &member.role }</p>
                                    <p><b>{ "Joined At: " }</b>{ &member.joined_at }</p>
                                </li>
                            }) }
                        </ul>
                    </div>
                }
            }
        } else {
            html! {}
        };

        html! {
            <div class="cooperative-detail">
                <h1>{ &cooperative.name }</h1>
                <p>{ cooperative.description.as_deref().unwrap_or("No description available") }</p>
                <p><b>{ "Founded on: " }</b>{ cooperative.founded_date.clone() }</p>
                <p><b>{ "Website: " }</b><a href={cooperative.website.clone().unwrap_or_default()} target="_blank">{ cooperative.website.as_deref().unwrap_or("N/A") }</a></p>
                { members_list }
                { render_projects_list(projects) }
            </div>
        }
    };

    html! {
        <div>
            {
                if let Some(error) = &*error {
                    html! { <p class="error-message">{ format!("Error: {}", error) }</p> }
                } else if let Some(cooperative) = &*cooperative {
                    render_cooperative(cooperative, &projects)
                } else {
                    html! { <p>{ "Loading cooperative details..." }</p> }
                }
            }
        </div>
    }
}
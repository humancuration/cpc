use yew::prelude::*;
use yew_router::prelude::*;

use uuid::Uuid;
use crate::project::{
    list::ProjectListComponent,
    form::ProjectFormComponent,
    detail::ProjectDetail,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum ProjectRoute {
    #[at("/projects")]
    List,
    #[at("/projects/new")]
    Create,
    #[at("/projects/:id")]
    Detail { id: Uuid },
    #[at("/projects/:id/edit")]
    Edit { id: Uuid },
}

pub fn switch_project(routes: ProjectRoute) -> Html {
    match routes {
        ProjectRoute::List => html! { <ProjectListComponent /> },
        ProjectRoute::Create => html! { <ProjectFormComponent project_id={None} /> },
        ProjectRoute::Detail { id } => html! { <ProjectDetail id={id} /> },
        ProjectRoute::Edit { id } => html! { <ProjectFormComponent project_id={Some(id)} /> },
    }
}
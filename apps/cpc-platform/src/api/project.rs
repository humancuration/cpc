use graphql_client::{GraphQLQuery, Response};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response as FetchResponse};

use crate::api::{get_graphql_endpoint, GqlError};

// The paths are relative to the project root
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries/projects.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize"
)]
pub struct Projects;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/mutations/project.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Deserialize"
)
]
pub struct CreateProject;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/mutations/project.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Deserialize"
)]
pub struct UpdateProject;

pub type Project = projects::ProjectsProjects;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries/project.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize"
)]
pub struct ProjectQuery;

pub type ProjectDetail = project_query::ProjectQueryProject;

pub async fn get_project(id: Uuid) -> Result<Option<ProjectDetail>, GqlError> {
    let request_body = ProjectQuery::build_query(project_query::Variables { id: id.to_string() });
    let json_body = serde_json::to_string(&request_body).map_err(|e| GqlError::Serialization(e.to_string()))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(&json_body)));

    let request = Request::new_with_str_and_init(&get_graphql_endpoint(), &opts)
        .map_err(|e| GqlError::RequestCreation(e.as_string().unwrap_or_default()))?;
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| GqlError::Fetch(e.as_string().unwrap_or_default()))?;
    let resp: FetchResponse = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let response_data: Response<project_query::ResponseData> =
        serde_json::from_str(&json.as_string().unwrap()).unwrap();

    if let Some(errors) = response_data.errors {
        return Err(GqlError::Gql(errors));
    }

    Ok(response_data.data.map(|d| d.project))
}

pub async fn list_projects() -> Result<Vec<Project>, GqlError> {
    let request_body = Projects::build_query(projects::Variables);
    let json_body = serde_json::to_string(&request_body).map_err(|e| GqlError::Serialization(e.to_string()))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(&json_body)));

    let request = Request::new_with_str_and_init(&get_graphql_endpoint(), &opts)
        .map_err(|e| GqlError::RequestCreation(e.as_string().unwrap_or_default()))?;
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| GqlError::Fetch(e.as_string().unwrap_or_default()))?;
    let resp: FetchResponse = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let response_data: Response<projects::ResponseData> =
        serde_json::from_str(&json.as_string().unwrap()).unwrap();

    if let Some(errors) = response_data.errors {
        return Err(GqlError::Gql(errors));
    }

    Ok(response_data.data.map(|d| d.projects).unwrap_or_default())
}

pub async fn create_project(
    input: CreateProjectInput,
) -> Result<create_project::ResponseData, GqlError> {
    let request_body = CreateProject::build_query(create_project::Variables {
        input: create_project::CreateProjectInput {
            name: input.name,
            description: input.description,
            cooperative_id: input.cooperative_id.to_string(),
        },
    });

    let json_body = serde_json::to_string(&request_body).map_err(|e| GqlError::Serialization(e.to_string()))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(&json_body)));

    let request = Request::new_with_str_and_init(&get_graphql_endpoint(), &opts)
        .map_err(|e| GqlError::RequestCreation(e.as_string().unwrap_or_default()))?;
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| GqlError::Fetch(e.as_string().unwrap_or_default()))?;
    let resp: FetchResponse = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let response_data: Response<create_project::ResponseData> =
        serde_json::from_str(&json.as_string().unwrap()).unwrap();

    if let Some(errors) = response_data.errors {
        return Err(GqlError::Gql(errors));
    }

    Ok(response_data.data.unwrap())
}

pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub cooperative_id: Uuid,
}

pub struct UpdateProjectInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn update_project(
    input: UpdateProjectInput,
) -> Result<update_project::ResponseData, GqlError> {
    let request_body = UpdateProject::build_query(update_project::Variables {
        id: input.id.to_string(),
        name: input.name,
        description: input.description,
    });

    let json_body = serde_json::to_string(&request_body).map_err(|e| GqlError::Serialization(e.to_string()))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(&json_body)));

    let request = Request::new_with_str_and_init(&get_graphql_endpoint(), &opts)
        .map_err(|e| GqlError::RequestCreation(e.as_string().unwrap_or_default()))?;
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| GqlError::Fetch(e.as_string().unwrap_or_default()))?;
    let resp: FetchResponse = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let response_data: Response<update_project::ResponseData> =
        serde_json::from_str(&json.as_string().unwrap()).unwrap();

    if let Some(errors) = response_data.errors {
        return Err(GqlError::Gql(errors));
    }

    Ok(response_data.data.unwrap())
}
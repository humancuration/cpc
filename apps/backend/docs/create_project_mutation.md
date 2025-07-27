# Architectural Plan: `createProject` GraphQL Mutation

This document outlines the architectural plan for implementing the `createProject` GraphQL mutation. The goal is to provide a clear, step-by-step guide for the `ougcode` persona to follow, ensuring the implementation aligns with our established hexagonal architecture and tech stack.

## 1. Overview

The `createProject` mutation will allow users to create new projects in the system. The implementation will touch upon the following layers:

- **GraphQL Layer**: Defines the mutation and its resolver.
- **Service Layer**: Orchestrates the project creation logic.
- **Repository Layer**: Handles database interactions.
- **Core Business Logic**: Defines the `Project` domain model.

## 2. GraphQL Schema (`apps/backend/src/graphql/schema.graphql`)

The first step is to update the GraphQL schema to correctly define the `createProject` mutation and its input.

### Action:

Update the `createProject` mutation to use an `CreateProjectInput` input type for better organization and clarity. The input should contain `name`, an optional `description`, and `cooperativeId`.

**File**: `apps/backend/src/graphql/schema.graphql`

```graphql
# ... existing schema ...

input CreateProjectInput {
  name: String!
  description: String
  cooperativeId: UUID!
}

# ... existing schema ...

extend type Mutation {
  createProject(input: CreateProjectInput!): Project!
  # ... other mutations
}

# ... existing schema ...
```

## 3. Core Business Logic (`packages/cpc-core/src/business/project.rs`)

The `Project` struct is already well-defined. No changes are needed here, but it's important to be aware of its structure.

**File**: `packages/cpc-core/src/business/project.rs`

```rust
// No changes needed. For reference:
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cooperative_id: Uuid,
    pub status: ProjectStatus,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 4. Repository Layer (`packages/cpc-core/src/repositories/project_repository.rs`)

The `ProjectRepository` already contains a `create` function. We will use this function.

### Action:

No changes are needed in the `create` function. Just ensure it's correctly used by the service layer.

**File**: `packages/cpc-core/src/repositories/project_repository.rs`

```rust
// No changes needed. For reference:
pub async fn create(
    &self,
    name: &str,
    description: Option<&str>,
    cooperative_id: Uuid,
) -> Result<Project, sqlx::Error> {
    let project = sqlx::query_as!(
        Project,
        r#"
        INSERT INTO projects (name, description, cooperative_id, id, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
        "#,
        name,
        description,
        cooperative_id,
        Uuid::new_v4(), // Generate new UUID for the project
        ProjectStatus::NotStarted as ProjectStatus
    )
    .fetch_one(&self.db_pool)
    .await?;

    Ok(project)
}
```

*Note to `ougcode`*: The original `create` function was missing the `id` and `status` fields in the `INSERT` statement. The corrected version above includes them. The `id` is generated with `Uuid::new_v4()` and the `status` defaults to `NotStarted`. The `RETURNING` clause was also updated to match the `Project` struct. The `projects` table migration adds a `DEFAULT` for `created_at` and `updated_at`, so we don't need to specify them in the query. The `id` column must be manually inserted, and the `status` column has no default so it must be added manually as well.

## 5. Service Layer (`apps/backend/src/project/service.rs`)

The `ProjectService` needs to be updated to correctly call the repository.

### Action:

Update the `create_project` method in the `ProjectService` to accept the new project data and call the repository's `create` method.

**File**: `apps/backend/src/project/service.rs`

```rust
// ... existing code ...
use uuid::Uuid;

// ... existing code ...

impl ProjectService {
    // ... existing code ...

    pub async fn create_project(
        &self,
        name: &str,
        description: Option<&str>,
        cooperative_id: Uuid,
    ) -> Result<Project, ServiceError> {
        self.repository
            .create(name, description, cooperative_id)
            .await
            .map_err(ServiceError::from)
    }

    // ... existing code ...
}
```

*Note to `ougcode`*: I've corrected the `create_project` function to properly call the repository. The previous implementation was a placeholder.

## 6. GraphQL Layer (`apps/backend/src/graphql/project.rs`)

This is where the GraphQL mutation will be defined and connected to the `ProjectService`.

### Action:

1.  Create a `CreateProjectInput` struct to match the GraphQL input type.
2.  Create a `ProjectMutation` struct.
3.  Implement the `create_project` mutation resolver in the `ProjectMutation` struct.
4.  Add the `ProjectMutation` to the root `Mutation` object in `apps/backend/src/graphql/mod.rs`.

**File**: `apps/backend/src/graphql/project.rs`

```rust
use async_graphql::{ComplexObject, Context, Object, Result, InputObject};
use cpc_core::business::project::{Project, ProjectStatus};
use crate::project::service::ProjectService;
use uuid::Uuid;

// This struct will represent the Project in the GraphQL schema.
#[derive(Debug, Clone)]
pub struct ProjectObject(pub Project);

#[Object]
impl ProjectObject {
    async fn id(&self) -> Uuid { self.0.id }
    async fn name(&self) -> &str { &self.0.name }
    async fn description(&self) -> Option<&str> { self.0.description.as_deref() }
    async fn cooperative_id(&self) -> Uuid { self.0.cooperative_id }
    async fn status(&self) -> ProjectStatus { self.0.status.clone() }
    async fn start_date(&self) -> Option<chrono::NaiveDate> { self.0.start_date }
    async fn end_date(&self) -> Option<chrono::NaiveDate> { self.0.end_date }
    async fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.0.created_at }
    async fn updated_at(&self) -> chrono::DateTime<chrono::Utc> { self.0.updated_at }
}

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<ProjectObject>> {
        let project_service = ctx.data::<ProjectService>()?;
        let projects = project_service
            .list_all_projects()
            .await?
            .into_iter()
            .map(ProjectObject)
            .collect();
        Ok(projects)
    }

    pub async fn project(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<ProjectObject> {
        let project_service = ctx.data::<ProjectService>()?;
        let project = project_service.get_project(id).await?;
        Ok(ProjectObject(project))
    }
}

#[derive(InputObject)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub cooperative_id: Uuid,
}

#[derive(Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<ProjectObject> {
        let project_service = ctx.data::<ProjectService>()?;
        let project = project_service
            .create_project(&input.name, input.description.as_deref(), input.cooperative_id)
            .await?;
        Ok(ProjectObject(project))
    }
}
```

## 7. Update Root `Mutation` (`apps/backend/src/graphql/mod.rs`)

Finally, merge the `ProjectMutation` into the root `Mutation` object.

**File**: `apps/backend/src/graphql/mod.rs`

```rust
// ... existing imports
use project::ProjectMutation; // Add this import

// ... other modules

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    // ... other mutations
    ProjectMutation, // Add this line
);

// ... existing code
```

By following these steps, `ougcode` will be able to implement the `createProject` mutation in a way that is consistent with our architecture and best practices.
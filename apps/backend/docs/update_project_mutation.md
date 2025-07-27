# Architectural Plan: `updateProject` GraphQL Mutation

This document outlines the architectural plan for the `updateProject` GraphQL mutation. The goal is to provide a clear guide to how the `updateProject` mutation is implemented, ensuring it aligns with our established hexagonal architecture and tech stack.

## 1. Overview

The `updateProject` mutation allows users to update existing projects in the system. The implementation touches upon the following layers:

-   **GraphQL Layer**: Defines the mutation and its resolver.
-   **Service Layer**: Orchestrates the project update logic.
-   **Repository Layer**: Handles database interactions.
-   **Core Business Logic**: Defines the `UpdateProject` struct.

## 2. GraphQL Schema (`apps/backend/src/graphql/schema.graphql`)

The GraphQL schema defines the `updateProject` mutation and its `UpdateProjectInput`.

**File**: `apps/backend/src/graphql/schema.graphql`

```graphql
input UpdateProjectInput {
    id: UUID!
    name: String
    description: String
}

extend type Mutation {
  updateProject(input: UpdateProjectInput!): Project!
}
```

## 3. Core Business Logic (`packages/cpc-core/src/business/project.rs`)

The `UpdateProject` struct defines the data payload for updating a project.

**File**: `packages/cpc-core/src/business/project.rs`

```rust
/// Represents the payload for updating a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
}
```

## 4. Repository Layer (`packages/cpc-core/src/repositories/project_repository.rs`)

The `ProjectRepository` contains the `update` function to handle the database update.

**File**: `packages/cpc-core/src/repositories/project_repository.rs`

```rust
/// Updates a project's details.
pub async fn update(&self, id: Uuid, payload: UpdateProject) -> Result<Project, sqlx::Error> {
    let project = sqlx::query_as!(
        Project,
        r#"
        UPDATE projects
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            updated_at = now()
        WHERE id = $3
        RETURNING id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
        "#,
        payload.name,
        payload.description,
        id
    )
    .fetch_one(&self.db_pool)
    .await?;

    Ok(project)
}
```

## 5. Service Layer (`apps/backend/src/project/service.rs`)

The `ProjectService` orchestrates the update by calling the repository.

**File**: `apps/backend/src/project/service.rs`

```rust
pub async fn update_project(&self, id: Uuid, payload: UpdateProject) -> Result<Project, ServiceError> {
    self.repository
        .update(id, payload)
        .await
        .map_err(ServiceError::from)
}
```

## 6. GraphQL Layer (`apps/backend/src/graphql/project.rs`)

This is where the GraphQL mutation is defined and connected to the `ProjectService`.

**File**: `apps/backend/src/graphql/project.rs`

```rust
#[derive(InputObject)]
pub struct UpdateProjectInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[Object]
impl ProjectMutation {
    pub async fn update_project(
        &self,
        ctx: &Context<'_>,
        input: UpdateProjectInput,
    ) -> Result<ProjectObject> {
        let project_service = ctx.data<ProjectService>()?;
        let payload = UpdateProject {
            name: input.name,
            description: input.description,
        };
        let project = project_service
            .update_project(input.id, payload)
            .await?;
        Ok(ProjectObject(project))
    }
}
```

## 7. Root `Mutation` (`apps/backend/src/graphql/mod.rs`)

The `ProjectMutation` is already merged into the root `Mutation` object, so no changes are needed here.
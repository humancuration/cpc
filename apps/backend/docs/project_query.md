# Project Query Implementation Plan

This document outlines the steps to implement a standardized GraphQL query to fetch a single project by its ID.

## 1. Goal

To create a single, clear GraphQL query `project(id: ID!): Project` that replaces the existing duplicate project queries.

## 2. Technical Plan

The implementation will involve changes across the GraphQL schema and resolver layers. The service and repository layers are already correct and require no changes.

### Step 1: Unify the GraphQL Schema (`apps/backend/src/graphql/schema.graphql`)

We will remove the redundant queries and establish `project(id: ID!): Project` as the single source of truth.

-   **Action:**
    1.  In the main `Query` type, remove `getProject(id: ID!): Project`.
    2.  In the `extend type Query` block at the end of the file, remove `project(id: UUID!): Project`.
    3.  Add `project(id: ID!): Project` to the main `Query` type.

### Step 2: Refactor the GraphQL Resolvers (`apps/backend/src/graphql/project.rs`)

We need to align our resolvers with the schema changes, creating a single `project` resolver function.

-   **Action:**
    1.  Remove the `get_project` resolver function (the one decorated with `#[graphql(name = "getProject")]`).
    2.  Modify the signature of the existing `project` function to accept `id: ID` instead of `id: Uuid`.
    3.  Move the ID parsing logic from the old `get_project` function into the updated `project` function. This will convert the `ID` string into a `Uuid` before calling the service.

### Step 3: Verify Service and Repository Layers (No Changes Needed)

-   **`apps/backend/src/project/service.rs`:** The `get_project(id: Uuid)` function is correctly implemented and will be called by our updated resolver.
-   **`packages/cpc-core/src/repositories/project_repository.rs`:** The `find_by_id(id: Uuid)` function correctly fetches the data from the database.

## 3. Summary of Changes

-   **`apps/backend/src/graphql/schema.graphql`**
    -   Consolidate `getProject` and `project` queries into one: `project(id: ID!): Project`.
-   **`apps/backend/src/graphql/project.rs`**
    -   Remove `get_project` resolver.
    -   Update `project` resolver to match the new schema definition.
-   **`apps/backend/src/project/service.rs`**
    -   No changes.
-   **`packages/cpc-core/src/repositories/project_repository.rs`**
    -   No changes.
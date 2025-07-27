# Project GraphQL Queries Implementation Plan

## 1. Goal

This document provides a comprehensive plan to expose two key GraphQL queries for project data:
1.  `project(id: ID!): Project`: Fetches a single project by its unique ID.
2.  `projects: [Project!]!`: Fetches a list of all projects.

This will provide a standardized and complete API surface for project-related data retrieval.

## 2. Implementation Plan

The service and repository layers are already equipped with the necessary functions (`get_project`, `list_all_projects`), so the work is focused on the GraphQL layer.

### Part 1: Single Project Query: `project(id: ID!): Project`

**Status: Implemented and Verified.**

The work to consolidate and implement a single query for fetching a project by its ID has been completed. The previous plan from `project_query.md` has been successfully executed.

Here is a summary of the current implementation for documentation purposes:

*   **GraphQL Schema (`apps/backend/src/graphql/schema.graphql`)**:
    *   The `Query` type correctly defines the query:
        ```graphql
        type Query {
          # ... other queries
          project(id: ID!): Project
          # ... other queries
        }
        ```

*   **GraphQL Resolver (`apps/backend/src/graphql/project.rs`)**:
    *   A resolver function `project` exists within the `ProjectQuery` implementation.
    *   It correctly parses the `ID` to a `Uuid` and calls the `get_project` method on the `ProjectService`.

### Part 2: All Projects Query: `projects: [Project!]!`

**Status: Pending a minor schema update.**

The resolver logic for fetching all projects is already implemented. The only remaining task is to expose it through the GraphQL schema.

**Step 1: Update GraphQL Schema (`apps/backend/src/graphql/schema.graphql`)**

We need to add the `projects` query to the root `Query` type.

*   **File:** `apps/backend/src/graphql/schema.graphql`
*   **Action:** Add `projects: [Project!]!` to the main `Query` type. It's best to place it alongside the singular `project` query for consistency.

    **Change:**
    ```graphql
    type Query {
      getImpactReport(userId: ID!): ImpactReport
      getProcessingStatus(jobId: ID!): ProcessingStatus
      getOrganizationImpactReport(orgId: UUID!, year: Int!): OrganizationImpactReport
      communities(first: Int!, after: String): CommunityConnection!
      community(id: ID!): Community
      getSupplyChainByProduct(productId: UUID!): SupplyChain
      listProductsWithSupplyChains: [ProductSummary!]
      listProductionStagesForProduct(productId: UUID!): [ProductionStage!]!
      project(id: ID!): Project
      projects: [Project!]!  # <-- ADD THIS LINE
    }
    ```

**Step 2: Verify Resolver Implementation (No Changes Needed)**

The resolver function that will serve this new query endpoint already exists.

*   **File:** `apps/backend/src/graphql/project.rs`
*   **Action:** No changes are needed. The existing `projects` function in `ProjectQuery` correctly calls the `list_all_projects` method from the `ProjectService`.

    ```rust
    // in apps/backend/src/graphql/project.rs
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
        // ... other resolvers
    }
    ```

## 3. Summary

By adding a single line to our GraphQL schema, we will have successfully implemented both project queries. The backend is ready, and this small change will complete the feature.
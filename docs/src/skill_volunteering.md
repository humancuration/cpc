# Skill Volunteering Service Architecture

This document outlines the architecture of the `skill_volunteering` service, a component of the CPC platform responsible for managing volunteer opportunities and related skills.

## 1. Overview

The `skill_volunteering` service follows a Hexagonal (Ports and Adapters) and Screaming Architecture. The core domain logic is isolated from external concerns like databases and transport protocols. This is achieved through the use of traits (ports) and their concrete implementations (adapters).

The service is structured into two main sub-domains:

-   **Skill Management**: Manages the skills that can be associated with volunteer opportunities.
-   **Opportunity Management**: Manages the creation, retrieval, updating, and deletion of volunteer opportunities.

## 2. Directory Structure

The package is organized as follows, reflecting the screaming architecture:

```
shared_packages/skill_volunteering/
├── src/
│   ├── opportunity_management/
│   │   ├── models.rs         # Domain models (e.g., VolunteerOpportunity)
│   │   ├── repository.rs     # Port (trait) for the opportunity repository
│   │   └── service.rs        # Core application service for opportunities
│   ├── skill_management/
│   │   ├── models.rs
│   │   ├── repository.rs
│   │   └── service.rs
│   ├── postgres/
│   │   ├── opportunity_repository.rs # PostgreSQL adapter for the opportunity repository
│   │   └── skill_repository.rs     # PostgreSQL adapter for the skill repository
│   ├── proto/
│   │   └── skill_volunteering.proto # gRPC service definition
│   ├── service.rs              # gRPC service implementation (primary adapter)
│   └── lib.rs                  # Crate root
└── build.rs                    # Compiles the .proto file
```

## 3. Opportunity Management Flow

This section details the flow for managing volunteer opportunities, including the new `delete_opportunity` functionality.

### 3.1. Creating, Reading, and Updating Opportunities

These operations follow a standard pattern:

1.  A gRPC request is received by `SkillVolunteeringServiceImpl`.
2.  The request is converted into a domain-specific command.
3.  The corresponding method in `OpportunityService` is called.
4.  The `OpportunityService` performs business logic validation (e.g., checking if a `Cause` exists via the `CauseServiceClient`).
5.  The `OpportunityService` interacts with the `OpportunityRepository` trait to persist or retrieve data.
6.  The `PostgresOpportunityRepository` adapter implements the trait, executing the necessary SQL queries against the PostgreSQL database.

### 3.2. Deleting an Opportunity

The `delete_opportunity` functionality will be implemented following the same hexagonal architecture principles.

#### 3.2.1. Port: `OpportunityRepository` Trait

The `opportunity_management/repository.rs` file will be updated to include a `delete_opportunity` method in the `OpportunityRepository` trait.

*   **File**: `shared_packages/skill_volunteering/src/opportunity_management/repository.rs`
*   **Method Signature**:
    ```rust
    // in trait OpportunityRepository
    async fn delete_opportunity(&self, id: Uuid) -> Result<(), OpportunityRepositoryError>;
    ```

#### 3.2.2. Adapter: `PostgresOpportunityRepository`

The PostgreSQL adapter will implement the new method.

*   **File**: `shared_packages/skill_volunteering/src/postgres/opportunity_repository.rs`
*   **Implementation**: This implementation will execute a `DELETE` SQL statement on the `volunteer_opportunities` table, removing the row with the matching `id`.

#### 3.2.3. Application Service: `OpportunityService`

The core service will expose a method to handle the deletion logic.

*   **File**: `shared_packages/skill_volunteering/src/opportunity_management/service.rs`
*   **Method Signature**:
    ```rust
    // in impl OpportunityService
    pub async fn delete_opportunity(&mut self, opportunity_id: Uuid) -> Result<(), OpportunityServiceError>;
    ```
*   **Logic**: This method will call the `delete_opportunity` method on the injected `OpportunityRepository`. It may also include logic to check if the opportunity exists before attempting deletion.

#### 3.2.4. Primary Adapter: `SkillVolunteeringServiceImpl` (gRPC)

The gRPC service implementation will handle the incoming request and orchestrate the call to the application service.

*   **File**: `shared_packages/skill_volunteering/src/service.rs`
*   **Method**: `delete_opportunity`
*   **Logic**:
    1.  Parse the `opportunity_id` from the `DeleteOpportunityRequest`.
    2.  Call the `delete_opportunity` method on the `OpportunityService`.
    3.  Return a `DeleteOpportunityResponse` upon success, or a `tonic::Status` on failure.

### 3.3. Listing User Applications

This feature allows users to retrieve a list of all volunteer opportunities they have applied for. The implementation follows the hexagonal architecture.

#### 3.3.1. Port: `OpportunityRepository` Trait

To support filtering applications, a new struct will be added, and the `OpportunityRepository` trait will be extended with a new method for listing applications associated with a specific user.

*   **File**: `shared_packages/skill_volunteering/src/opportunity_management/repository.rs`
*   **New Struct**:
    ```rust
    pub struct ListUserApplicationsFilters {
        pub user_id: Uuid,
        pub status: Option<String>,
        pub limit: i64,
        pub offset: i64,
    }
    ```
*   **Method Signature**:
    ```rust
    // in trait OpportunityRepository
    async fn list_user_applications(
        &self,
        filters: &ListUserApplicationsFilters,
    ) -> Result<(Vec<OpportunityApplication>, i64), OpportunityRepositoryError>;
    ```

#### 3.3.2. Adapter: `PostgresOpportunityRepository`

The PostgreSQL adapter will implement the `list_user_applications` method to query the database.

*   **File**: `shared_packages/skill_volunteering/src/postgres/opportunity_repository.rs`
*   **Implementation**: This implementation will use `sqlx::QueryBuilder` to construct a `SELECT` query on the `opportunity_applications` table.
    *   It will filter results by `user_id`.
    *   It will conditionally add a `WHERE` clause for the `status` if provided.
    *   It will perform a separate `COUNT(*)` query to get the total number of matching applications.
    *   It will apply `LIMIT` and `OFFSET` for pagination.

#### 3.3.3. Application Service: `OpportunityService`

The `OpportunityService` will orchestrate the retrieval of user applications.

*   **File**: `shared_packages/skill_volunteering/src/opportunity_management/service.rs`
*   **Method Signature**:
    ```rust
    // in impl OpportunityService
    pub async fn list_user_applications(
        &self,
        user_id: Uuid,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<OpportunityApplication>, i64), OpportunityServiceError>;
    ```
*   **Logic**: This method will construct a `ListUserApplicationsFilters` struct and call the corresponding method on the `OpportunityRepository`. In the future, it could be extended to validate the existence of the user by calling a user management service.

#### 3.3.4. Primary Adapter: `SkillVolunteeringServiceImpl` (gRPC)

The gRPC service will expose the `ListUserApplications` endpoint.

*   **File**: `shared_packages/skill_volunteering/src/service.rs`
*   **Method**: `list_user_applications`
*   **Logic**:
    1.  Parse `user_id`, `limit`, `offset`, and `status` from the `ListUserApplicationsRequest`.
    2.  Call the `list_user_applications` method on the `OpportunityService`.
    3.  Map the resulting domain `OpportunityApplication` models to the protobuf-generated `OpportunityApplication` types.
    4.  Return a `ListUserApplicationsResponse` containing the list of applications and the total count.

## 4. Next Steps

The next step is to implement the changes described in this document. A new task will be created for `ougcode` mode with the specific implementation details.
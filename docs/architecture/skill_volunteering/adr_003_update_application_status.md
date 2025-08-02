# ADR 003: Implement Update Application Status Feature

**Date**: 2025-08-01

**Status**: Proposed

## Context

The `skill_volunteering` service needs the ability to update the status of a volunteer's application for an opportunity. This is a crucial function for opportunity managers to accept, reject, or mark applications as complete. This ADR outlines the architectural changes required to implement this feature.

The implementation will follow our established architectural principles: Hexagonal Architecture, Screaming Architecture, and Vertical Slices.

## Decision

We will implement an `update_application_status` feature that flows through the gRPC service, into the application service, and down to the database repository.

### 1. Protobuf Definition (`proto/skill_volunteering.proto`)

The gRPC interface will be updated to formally define the request and response for this new RPC method.

-   **`UpdateApplicationStatusRequest`**:
    -   `application_id` (string, UUID)
    -   `status` (string)
    -   `optional volunteer_hours` (string, representing a Decimal)
-   **`UpdateApplicationStatusResponse`**:
    -   `OpportunityApplication` (the full, updated application object)

The service definition will include the RPC method:
```protobuf
rpc UpdateApplicationStatus(UpdateApplicationStatusRequest) returns (UpdateApplicationStatusResponse);
```
*(Note: This is already present in the current proto file, so no changes are needed, but it's documented here for completeness.)*

### 2. Domain Model (`opportunity_management/models.rs`)

No changes are required. The existing `OpportunityApplication` struct and `ApplicationStatus` enum are sufficient. The `volunteer_hours` field is an `Option<Decimal>`, which correctly supports the requirement.

### 3. Application Service (`opportunity_management/service.rs`)

A new method, `update_application_status`, will be added to the `OpportunityService`.

**Method Signature:**

```rust
// In impl OpportunityService

pub async fn update_application_status(
    &self,
    application_id: Uuid,
    new_status: ApplicationStatus,
    volunteer_hours: Option<rust_decimal::Decimal>,
) -> Result<OpportunityApplication, OpportunityServiceError>
```

**Logic:**

1.  Call the repository to find the application by `application_id`. If not found, return an `OpportunityServiceError::NotFound`.
2.  Update the `status` field of the retrieved application object with `new_status`.
3.  If `new_status` is `ApplicationStatus::Completed` and `volunteer_hours` is `Some`, update the application's `volunteer_hours` field.
4.  Call the new `update_application` method on the repository to persist the changes.
5.  Return the updated `OpportunityApplication` object.

### 4. Repository Trait (`opportunity_management/repository.rs`)

To facilitate updating applications, a new method will be added to the `OpportunityRepository` trait.

**Trait Method:**

```rust
// In trait OpportunityRepository

/// Updates an existing opportunity application.
async fn update_application(
    &self,
    application: &OpportunityApplication,
) -> Result<OpportunityApplication, OpportunityRepositoryError>;
```

This approach of passing the entire `OpportunityApplication` object makes the function more flexible for future updates.

### 5. PostgreSQL Repository (`postgres/opportunity_repository.rs`)

The `PostgresOpportunityRepository` will implement the new `update_application` method.

**Implementation Logic:**

1.  Construct an `UPDATE` SQL query targeting the `opportunity_applications` table.
2.  The query will update the `status` and `volunteer_hours` columns for the row matching the provided `application.id`.
3.  Use the `RETURNING *` clause to get the updated row back from the database.
4.  Map the resulting row back to an `OpportunityApplication` model and return it.

**Pseudo-SQL:**

```sql
UPDATE opportunity_applications
SET
    status = $2,
    volunteer_hours = $3
WHERE id = $1
RETURNING id, opportunity_id, user_id, applied_at, status, volunteer_hours;
```

### 6. gRPC Service (`service.rs`)

The `update_application_status` RPC method in `SkillVolunteeringServiceImpl` will be implemented.

**Implementation Logic:**

1.  Parse and validate the `application_id` from the `UpdateApplicationStatusRequest`.
2.  Parse the `status` string into an `ApplicationStatus` enum. Return `Status::invalid_argument` on failure.
3.  Parse the optional `volunteer_hours` string into a `rust_decimal::Decimal`. Return `Status::invalid_argument` on failure.
4.  Call the `update_application_status` method on the `OpportunityService`.
5.  Map any `OpportunityServiceError` to an appropriate `tonic::Status`.
6.  On success, convert the returned domain `OpportunityApplication` into a proto `OpportunityApplication` using the existing `domain_app_to_proto` helper function.
7.  Wrap the proto application in an `UpdateApplicationStatusResponse` and return it.

## Consequences

-   **Positive**:
    -   Enables a core piece of functionality for the skill volunteering workflow.
    -   The changes are well-encapsulated within the `skill_volunteering` package.
    -   The design is consistent with the existing architecture.
-   **Negative**:
    -   None identified. This is a standard feature implementation.
-   **Risks**:
    -   The string-to-enum parsing for `status` and string-to-decimal parsing for `volunteer_hours` must be robust to handle invalid inputs gracefully.
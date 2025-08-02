# Architecture: User Skill Profile

This document outlines the architectural design for implementing the User Skill Profile feature within the Skill Volunteering platform.

## 1. Overview

The User Skill Profile feature allows users to associate skills with their profiles. This is the foundational step for the Opportunity Matching Service, which will proactively recommend volunteering opportunities.

The implementation consists of three main parts:
- A new database table to store user skill associations.
- Additions to the gRPC API for managing user skills.
- A new service module to encapsulate the business logic, following our hexagonal architecture principles.

## 2. Database Schema

We will introduce a new table, `user_skills`, to create a many-to-many relationship between `users` and `skills`.

A new SQL migration file will be created at `shared_packages/skill_volunteering/migrations/20250801000002_create_user_skills_table.sql`.

### `user_skills` Table Definition

| Column | Type | Constraints | Description |
|---|---|---|---|
| `user_id` | `UUID` | `PRIMARY KEY`, `FOREIGN KEY` to `users(id)` | The ID of the user. |
| `skill_id` | `UUID` | `PRIMARY KEY`, `FOREIGN KEY` to `skills(id)` | The ID of the skill. |
| `skill_level` | `skill_level_enum` | `NOT NULL` | The user's self-assessed proficiency level. |
| `created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp of when the skill was added. |
| `updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp of the last update. |

### SQL Migration

```sql
-- Create a custom enum type for skill levels for data consistency.
CREATE TYPE skill_level_enum AS ENUM ('beginner', 'intermediate', 'advanced');

-- Create the user_skills table
CREATE TABLE user_skills (
  user_id UUID NOT NULL, -- Assuming a users table exists
  skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
  skill_level skill_level_enum NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  PRIMARY KEY (user_id, skill_id)
);

-- Create indexes for efficient querying
CREATE INDEX idx_user_skills_user_id ON user_skills(user_id);
CREATE INDEX idx_user_skills_skill_id ON user_skills(skill_id);

-- Note: We are assuming a `users` table with a `UUID id` primary key exists.
-- A foreign key constraint to `users(id)` should be added once the users table is formally managed within a migration.
```

## 3. gRPC API Changes

The `shared_packages/skill_volunteering/proto/skill_volunteering.proto` file will be updated to include new messages and RPCs for managing user skills.

### New Messages

```protobuf
// Represents a skill associated with a user.
message UserSkill {
  string user_id = 1;
  string skill_id = 2;
  string skill_level = 3; // "beginner", "intermediate", "advanced"
  google.protobuf.Timestamp created_at = 4;
  google.protobuf.Timestamp updated_at = 5;
}

// Represents detailed information about a user's skill, including the skill name and category.
// Used in list operations.
message UserSkillDetails {
    string user_id = 1;
    Skill skill = 2; // from existing definitions
    string skill_level = 3;
    google.protobuf.Timestamp created_at = 4;
}
```

### New RPCs in `SkillVolunteeringService`

```protobuf
service SkillVolunteeringService {
  // ... existing RPCs

  // User Skill Management
  rpc AddUserSkill(AddUserSkillRequest) returns (AddUserSkillResponse);
  rpc ListUserSkills(ListUserSkillsRequest) returns (ListUserSkillsResponse);
  rpc RemoveUserSkill(RemoveUserSkillRequest) returns (RemoveUserSkillResponse);
}
```

### Request/Response Definitions

```protobuf
// AddUserSkill
message AddUserSkillRequest {
  string user_id = 1;
  string skill_id = 2;
  string skill_level = 3;
}

message AddUserSkillResponse {
  UserSkill user_skill = 1;
}

// ListUserSkills
message ListUserSkillsRequest {
  string user_id = 1;
}

message ListUserSkillsResponse {
  repeated UserSkillDetails user_skills = 1;
}

// RemoveUserSkill
message RemoveUserSkillRequest {
  string user_id = 1;
  string skill_id = 2;
}

message RemoveUserSkillResponse {
  bool success = 1;
}
```

## 4. Service Architecture

A new module, `user_skill_management`, will be created in `shared_packages/skill_volunteering/src/`. This follows our established pattern of separating concerns by domain.

### Module Structure: `user_skill_management`

- **`mod.rs`**: Declares the sub-modules.
- **`models.rs`**: Contains the core Rust domain models.
  - `SkillLevel` enum (`Beginner`, `Intermediate`, `Advanced`) with `FromStr` and `Display` implementations.
  - `UserSkill` struct, representing the `user_skills` table record.
  - `UserSkillDetails` struct, for queries that join with the `skills` table.
- **`service.rs`**: The application layer.
  - `UserSkillService` struct will orchestrate the logic. It will depend on the `UserSkillRepository` trait.
  - Methods will correspond to the gRPC calls: `add_user_skill`, `list_user_skills`, `remove_user_skill`.
- **`repository.rs`**: The data access port (trait).
  - `UserSkillRepository` trait will define the contract for data persistence.
  - Methods: `add`, `list_by_user_id`, `remove`.

### Postgres Implementation

The implementation of the `UserSkillRepository` trait will be located in a new file: `shared_packages/skill_volunteering/src/postgres/user_skill_repository.rs`. This keeps our database-specific code isolated, adhering to the Hexagonal Architecture principle.

## 5. General Skill Endpoints

To facilitate skill discovery for frontend components, such as populating dropdowns or search fields, a general endpoint for listing all available skills is required. This is distinct from listing a *user's* skills.

### New RPC in `SkillVolunteeringService`

The `ListSkills` RPC is added to the main service definition.

```protobuf
// In service SkillVolunteeringService { ... }
rpc ListSkills(ListSkillsRequest) returns (ListSkillsResponse);
```

### Request/Response Definitions

These messages support pagination and filtering for the general skill list.

```protobuf
message ListSkillsRequest {
  optional string category = 1;
  optional int32 limit = 2;
  optional int32 offset = 3;
}

message ListSkillsResponse {
  repeated Skill skills = 1;
  int32 total_count = 2;
}
```

This endpoint allows clients to retrieve a paginated list of all skills in the system, with an option to filter by category.

- **`category`**: If provided, only skills matching the category will be returned.
- **`limit`**: The maximum number of skills to return in one response.
- **`offset`**: The starting point for pagination, used to skip a certain number of results.
- **`skills`**: The list of `Skill` objects returned by the query.
- **`total_count`**: The total number of skills that match the filter criteria, which is useful for building client-side pagination controls.
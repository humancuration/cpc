# Collaboration Engine Domain Model

## Overview
The domain model for the Collaboration Engine encompasses the core concepts needed for real-time collaborative editing, including documents, operations, presence, and versioning.

## Core Entities

### Document
Represents a collaborative document that can be edited by multiple users simultaneously.

**Attributes:**
- `id: Uuid` - Unique identifier for the document
- `content: String` - Current content of the document
- `version: u64` - Current version number
- `operations: Vec<Operation>` - History of operations applied
- `created_at: DateTime<Utc>` - Creation timestamp
- `updated_at: DateTime<Utc>` - Last update timestamp
- `event_version: String` - Current event schema version (semver)

### Operation
Represents an atomic change that can be applied to a document.

**Variants:**
- `Insert` - Insert text at a specific position
- `Delete` - Delete text between two positions
- `Replace` - Replace text between two positions with new text

**Common Attributes:**
- `position: Position` - Location in the document
- `user_id: Uuid` - User who performed the operation
- `timestamp: DateTime<Utc>` - When the operation was performed

### Presence
Represents a user's current state in a collaborative session.

**Attributes:**
- `user_id: Uuid` - Unique identifier for the user
- `user_name: String` - Display name of the user
- `cursor_position: Position` - Current cursor position
- `selection_start: Option<Position>` - Start of selection if any
- `selection_end: Option<Position>` - End of selection if any
- `last_seen: DateTime<Utc>` - When the user was last active
- `is_typing: bool` - Whether the user is currently typing
- `color: String` - Color used to display the user's cursor
- `qos_tier: u8` - Quality of Service tier (0=critical, 1=medium, 2=low)

### Conflict
Represents a conflict between concurrent operations.

**Attributes:**
- `id: Uuid` - Unique identifier for the conflict
- `document_id: Uuid` - Reference to the document
- `conflicting_operations: Vec<Operation>` - Operations that conflict
- `resolution_strategy: ResolutionStrategy` - Strategy to resolve the conflict
- `resolved: bool` - Whether the conflict has been resolved
- `resolved_operations: Vec<Operation>` - Operations after resolution
- `resolved_at: Option<DateTime<Utc>>` - When the conflict was resolved
- `created_at: DateTime<Utc>` - When the conflict was detected
- `metadata: ConflictMetadata` - Additional metadata about the conflict

### ResolutionStrategy
Strategies for resolving conflicts between operations.

**Variants:**
- `TimestampOrder` - Resolve by chronological order
- `UserPriority` - Resolve by user priority
- `Merge` - Attempt to merge operations
- `Manual` - Require manual resolution

### ConflictMetadata
Metadata about conflict detection and resolution.

**Attributes:**
- `detection_method: String` - How the conflict was detected
- `transformation_history: Vec<TransformationRecord>` - History of transformations
- `resolution_details: Option<String>` - Additional details about resolution

### TransformationRecord
Record of an operation transformation.

**Attributes:**
- `original_operation: Operation` - The operation before transformation
- `transformed_operation: Operation` - The operation after transformation
- `transformation_type: String` - Type of transformation applied
- `timestamp: DateTime<Utc>` - When the transformation occurred

### EventEnvelope
Wrapper for all collaboration events ensuring version compatibility.

**Attributes:**
- `id: Uuid` - Unique event identifier
- `event_type: String` - Type of event (OperationApplied, PresenceUpdated, etc.)
- `payload: serde_json::Value` - Event data
- `version: String` - Event schema version (semver)
- `batch_id: Option<Uuid>` - Batch identifier if batched
- `created_at: DateTime<Utc>` - Creation timestamp

### DocumentVersion
Represents a specific version of a document in the version history.

**Attributes:**
- `id: Uuid` - Unique identifier for the version
- `document_id: Uuid` - Reference to the document
- `version_number: u64` - Version number
- `content: String` - Content at this version
- `operations: Vec<Operation>` - Operations that led to this version
- `author_id: Uuid` - User who created this version
- `author_name: String` - Name of the author
- `created_at: DateTime<Utc>` - Creation timestamp
- `commit_message: Option<String>` - Optional message describing the changes
- `conflict_metadata: Option<serde_json::Value>` - Optional conflict resolution metadata

### JsonSchema
Represents a JSON schema for validating events.

**Attributes:**
- `definition: serde_json::Value` - The schema definition in JSON format
- `created_at: DateTime<Utc>` - When this schema was created
- `deprecated: bool` - Whether this schema is deprecated
- `deprecated_until: Option<DateTime<Utc>>` - If deprecated, when it will be removed

## Value Objects

### Position
Represents a position in a document using line and column coordinates.

**Attributes:**
- `line: usize` - Line number (0-based)
- `column: usize` - Column number (0-based)

## Domain Services

### CRDTDocument
Implements Conflict-free Replicated Data Types for distributed editing.

**Methods:**
- `apply_operation(user_id, operation)` - Apply an operation from a specific user
- `merge(other)` - Merge operations from another CRDT document

### PresenceManager
Manages presence information for users in a collaborative session.

**Methods:**
- `update_presence(presence)` - Update a user's presence information
- `remove_presence(user_id)` - Remove a user's presence
- `get_presences()` - Get all active presences
- `is_user_active(user_id)` - Check if a user is currently active

### ConflictResolver
Detects and resolves conflicts between concurrent operations using Operational Transformation.

**Methods:**
- `detect_conflicts(operations)` - Detect conflicts between operations
- `resolve_conflict(conflict_id)` - Resolve a specific conflict
- `set_user_priority(user_id, priority)` - Set priority for conflict resolution
- `get_user_priority(user_id)` - Get user priority considering QoS tier
- `transform_operation(op1, op2)` - Transform operation op1 against op2
- `set_presence_manager(presence_manager)` - Set the presence manager for QoS integration
- `set_version_manager(version_manager)` - Set the version manager for version creation

### VersionManager
Manages version history for collaborative documents.

**Methods:**
- `create_version(document, author_id, author_name, commit_message, conflict_metadata)` - Create a new version with conflict metadata
- `get_version(version_number)` - Get a specific version
- `create_branch(branch_name, version_number)` - Create a new branch
- `create_tag(tag_name, version_number)` - Create a tag for a version
- `compare_versions(version_a, version_b)` - Compare two versions including conflict history

### SchemaRegistry
Manages JSON schemas for events and provides versioning capabilities.

**Methods:**
- `register_schema(event_type, version, schema)` - Register a schema for an event type
- `get_schema(event_type, version)` - Get a schema for an event type
- `register_transformation(event_type, from_version, to_version, transformer)` - Register a transformation function
- `get_transformer(event_type, from_version, to_version)` - Get a transformation function
- `validate(event)` - Validate an event against its schema
- `transform(event, target_version)` - Transform an event to a target version
- `is_deprecated(event_type, version)` - Check if a schema version is deprecated
- `list_versions(event_type)` - List all versions for an event type
- `list_versions(event_type)` - List all versions for an event type
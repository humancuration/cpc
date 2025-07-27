# Calendar Module Migration Guide

This guide provides instructions for migrating existing calendar data to the new calendar module.

## Migration Process

### 1. Task Manager Deadlines

Convert all task deadlines to calendar events:
- Map task ID to event ID
- Set event type to `TaskDeadline(task_id)`
- Preserve original deadline metadata
- Maintain backward compatibility with task module

### 2. Legacy Calendar Data

Import existing calendar events from external services:
- ICS import pipeline with validation
- Conflict detection for overlapping events
- User-controlled mapping of categories
- Preserve creation and modification timestamps

## Database Migration

### Schema Changes

The new calendar module requires the following database tables:

```sql
-- Calendar events table
CREATE TABLE calendar_events (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start TIMESTAMP WITH TIME ZONE NOT NULL,
    "end" TIMESTAMP WITH TIME ZONE NOT NULL,
    event_type TEXT NOT NULL,
    visibility TEXT NOT NULL,
    recurrence TEXT,
    location TEXT,
    attachments TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Work shifts table
CREATE TABLE work_shifts (
    id UUID PRIMARY KEY,
    cooperative_id UUID NOT NULL,
    position VARCHAR(255) NOT NULL,
    schedule TEXT NOT NULL,
    coverage TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Event reminders table
CREATE TABLE event_reminders (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    trigger_time TIMESTAMP WITH TIME ZONE NOT NULL,
    method TEXT NOT NULL,
    escalation_level INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    message TEXT NOT NULL
);

-- Participants table
CREATE TABLE participants (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time TIMESTAMP WITH TIME ZONE
);

-- Shift assignments table
CREATE TABLE shift_assignments (
    id UUID PRIMARY KEY,
    shift_id UUID NOT NULL,
    user_id UUID NOT NULL,
    assigned_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Availability slots table
CREATE TABLE availability_slots (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    start TIMESTAMP WITH TIME ZONE NOT NULL,
    "end" TIMESTAMP WITH TIME ZONE NOT NULL,
    availability_type TEXT NOT NULL
);
```

### Data Migration Scripts

#### Migrate Task Deadlines

```sql
-- Insert task deadlines as calendar events
INSERT INTO calendar_events (
    id, user_id, title, description, start, "end", event_type, visibility, created_at, updated_at
)
SELECT 
    gen_random_uuid(), -- Generate new ID
    task.user_id,
    task.title,
    task.description,
    task.deadline, -- Use deadline as both start and end time
    task.deadline,
    '{"TaskDeadline": "' || task.id || '"}', -- Set event type to TaskDeadline
    '"Private"', -- Default to private visibility
    task.created_at,
    task.updated_at
FROM tasks task
WHERE task.deadline IS NOT NULL;
```

#### Migrate Legacy Calendar Events

Use the ICS import functionality to import legacy calendar data:
1. Export existing calendar data to ICS format
2. Use the `IcsImporter` to import the data
3. Validate imported events
4. Map categories and visibility settings

## Integration with Existing Modules

### Task Manager Integration

Maintain bidirectional sync between tasks and calendar events:
- When a task deadline is updated, update the corresponding calendar event
- When a calendar event of type `TaskDeadline` is updated, update the corresponding task
- Handle deletions in both directions

### Notes & Memos Integration

Link event-attached notes to the Notes module:
- Store note references as event attachments
- Maintain cross-module linking via shared UUID identifiers
- Implement unified search across calendar events and notes

## Testing Migration

### Unit Tests

- Test data conversion functions
- Verify schema compatibility
- Validate business rule enforcement

### Integration Tests

- Test end-to-end migration process
- Verify data integrity after migration
- Check integration with existing modules

## Rollback Procedure

In case of migration issues:

1. Restore database from backup
2. Revert to previous calendar implementation
3. Investigate and fix migration issues
4. Retry migration with fixes

## Performance Considerations

- Batch process large data sets to avoid memory issues
- Use database transactions to ensure data consistency
- Monitor migration progress with logging
- Provide user feedback during long migrations
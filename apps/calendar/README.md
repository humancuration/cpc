# Calendar Module

The calendar module provides comprehensive calendar functionality for the CPC platform, supporting both personal and business scheduling needs.

## Features

- Personal and business event management
- Work shift scheduling for cooperatives
- Smart reminder system with escalation
- p2p synchronization using p2panda
- Integration with task manager and other modules
- ICS import/export for external calendar integration

## Architecture

The calendar module follows a hexagonal architecture with vertical slices, implementing all functionality within the `packages/cpc-core/calendar/` directory.

### Module Structure

```
apps/calendar/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── domain/          # Pure business logic (events, participants, shifts)
    │   ├── event.rs
    │   ├── participant.rs
    │   ├── shift.rs
    │   ├── reminder.rs
    │   ├── primitives.rs
    │   └── mod.rs
    ├── application/     # Use cases and service orchestration
    │   ├── scheduling_service.rs
    │   ├── shift_management.rs
    │   ├── reminder_service.rs
    │   └── mod.rs
    ├── infrastructure/  # Concrete implementations
    │   ├── database/
    │   │   ├── models.rs
    │   │   ├── repositories.rs
    │   │   └── mod.rs
    │   ├── sync/
    │   │   ├── ics_importer.rs
    │   │   ├── p2p_sync.rs
    │   │   └── mod.rs
    │   └── mod.rs
    └── presentation/    # UI components (Bevy, Yew) - to be implemented
        └── mod.rs
```

## Domain Models

### CalendarEvent

Represents a calendar event with support for:
- Personal and business event types
- Recurring events with complex recurrence rules
- Location-based events with geofencing
- Privacy controls with multiple visibility levels

### WorkShift

Manages work shifts for cooperative scheduling:
- Fixed and rotating shift schedules
- Staff coverage tracking
- Shift swap requests

### EventReminder

Smart reminder system with escalation:
- Multiple delivery methods (push, email, SMS, in-app, location-based)
- Configurable escalation rules
- Delivery status tracking

## Services

### SchedulingService

Handles calendar event creation, modification, and querying:
- Conflict detection and resolution
- Participant coordination
- Event sharing through p2p network

### ShiftManagementService

Manages cooperative work shift scheduling:
- Shift rotation algorithms
- Coverage requirement enforcement
- Shift swap request system

### ReminderService

Processes event reminders and notifications:
- Smart escalation logic
- Multiple notification channels
- Delivery status tracking

## Integration Points

- **Task Manager**: Bidirectional sync for task deadlines ↔ calendar events
- **Notes & Memos**: Event-attached notes capability
- **Business Health Dashboard**: Time tracking metrics extraction
- **Health Module**: Health appointment events

## Privacy & Modularity

The calendar module follows strict privacy principles:
- Business/Personal data separation
- True modularity with feature flags
- Cooperative principles implementation
- Privacy-first design with data minimization

All business features can be disabled at runtime, and the module can be "uninstalled" without affecting other parts of the system.
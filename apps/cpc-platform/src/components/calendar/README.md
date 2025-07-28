# Calendar Components

This directory contains the UI components for the calendar feature of the CPC Platform.

## Structure

```
calendar/
├── mod.rs                 # Module declaration
├── styles.css             # Shared styles for calendar components
├── cards/                 # Event and shift card components
│   ├── mod.rs             # Cards module declaration
│   ├── event_card.rs      # Event card component
│   ├── event_card_test.rs # Tests for event card
├── state/                 # State management
│   ├── mod.rs             # State module declaration
│   ├── view.rs            # Calendar view types
│   ├── store.rs           # Calendar store implementation
│   ├── store_test.rs      # Tests for calendar store
├── utils/                 # Utility functions
│   ├── mod.rs             # Utils module declaration
│   ├── format_recurrence.rs        # Recurrence formatting utilities
│   ├── format_recurrence_test.rs   # Tests for recurrence formatting
│   ├── sync.rs            # Conflict resolution utilities
│   ├── touch.rs           # Touch gesture handlers
├── views/                 # View components
│   ├── mod.rs             # Views module declaration
│   ├── switcher.rs        # Calendar view switcher
├── integration_test.rs    # Integration tests
```

## Components

### EventCard
A component for displaying calendar events with all visualization elements:
- EventTypeBadge: Color-coded badge based on EventType
- PrivacyIndicator: Unique icon for each EventVisibility level
- LocationTag: Map pin icon + truncated address
- RecurringIndicator: Circular arrow icon (only for recurring events)
- GeofenceVisualization: Radius circle on map view when location has radius

### CalendarViewSwitcher
A component for switching between different calendar views:
- Month view
- Week view
- Day view
- Shift schedule view

### CalendarStore
State management for the calendar using Yew Context API:
- Manages current view and selected date
- Stores events and work shifts
- Handles loading and error states

## Mobile Support

The calendar components are optimized for mobile devices with:
- Touch-friendly controls (minimum 48x48px targets)
- Responsive layout breakpoints
- Swipe gestures for date navigation
- Long press detection for context menus
- Haptic feedback for drag operations

## Testing

The calendar components include unit tests and integration tests:
- Unit tests for individual components and utilities
- Integration tests for component interactions
- Tests for recurrence rule formatting
- Tests for state management

## GraphQL Integration

The calendar integrates with the backend through GraphQL:
- `fetch_events` function for retrieving events
- Real-time synchronization with `setup_realtime_sync`
- Conflict resolution for concurrent edits

## Tauri Integration

Mobile-specific functionality is provided through Tauri commands:
- `get_current_position` for geolocation access
- `request_shift_swap` for shift management
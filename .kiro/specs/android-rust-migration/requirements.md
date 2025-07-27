# Requirements Document

## Introduction

This feature focuses on porting functionality from the legacy Android codebase (`apps/cpc-platform/android/WheresThisFrom`) to the shared Rust codebase for desktop and mobile applications. The primary goal is to create a unified codebase that enables consistent functionality across platforms while prioritizing the desktop application development. This port will leverage Rust's cross-platform capabilities along with technologies like Bevy Engine, p2panda, Tauri, and Axum to create a cohesive application ecosystem.

## Requirements

### Requirement 1: Core Functionality Port

**User Story:** As a developer, I want to port the core functionality from the legacy Android codebase to Rust, so that we can maintain a single codebase for both desktop and mobile platforms.

#### Acceptance Criteria

1. WHEN identifying core features in the legacy Android codebase THEN the system SHALL document all essential functionalities that need to be ported
2. WHEN porting core functionality THEN the system SHALL maintain feature parity with the original Android implementation
3. WHEN implementing ported features THEN the system SHALL use Rust as the primary language for shared logic
4. WHEN implementing platform-specific features THEN the system SHALL minimize platform-specific code and maximize shared Rust code
5. WHEN porting UI components THEN the system SHALL use Yew for the frontend implementation (migrating from Svelte)

### Requirement 2: Networking and P2P Implementation

**User Story:** As a user, I want the application to maintain its peer-to-peer networking capabilities, so that I can collaborate and share data with other users without relying on external cloud providers.

#### Acceptance Criteria

1. WHEN implementing networking features THEN the system SHALL use p2panda for P2P functionality
2. WHEN handling API communications for UIs THEN the system SHALL use GraphQL Mutations for task initiation and GraphQL Subscriptions for results
3. WHEN managing service-to-service communications THEN the system SHALL use gRPC Server Streaming for long-running jobs
4. WHEN implementing data synchronization THEN the system SHALL ensure consistent behavior across desktop and mobile platforms
5. IF network connectivity is lost THEN the system SHALL gracefully handle offline scenarios and resynchronize when connectivity is restored

### Requirement 3: Desktop Application Priority

**User Story:** As a product manager, I want to prioritize the desktop application development, so that we can deliver a functional desktop experience first.

#### Acceptance Criteria

1. WHEN planning the port implementation THEN the system SHALL prioritize desktop-specific features and optimizations
2. WHEN implementing shared components THEN the system SHALL ensure they work correctly on desktop platforms first
3. WHEN designing the architecture THEN the system SHALL use Tauri and wry for desktop application framework
4. WHEN implementing desktop-specific features THEN the system SHALL ensure they integrate properly with the shared Rust codebase
5. WHEN testing ported functionality THEN the system SHALL verify correct operation on desktop platforms before mobile platforms

### Requirement 4: Data Storage and Management

**User Story:** As a user, I want my data to be properly stored and managed across platforms, so that I have a consistent experience regardless of which platform I use.

#### Acceptance Criteria

1. WHEN implementing data storage THEN the system SHALL use rusqlite for local database functionality
2. WHEN handling date and time data THEN the system SHALL use chrono for consistent date/time operations
3. WHEN serializing and deserializing data THEN the system SHALL use serde and JSON for data transformation
4. WHEN implementing data models THEN the system SHALL ensure consistency between desktop and mobile implementations
5. WHEN migrating user data THEN the system SHALL preserve all existing user data and ensure backward compatibility

### Requirement 5: Backend Services Integration

**User Story:** As a developer, I want to integrate the ported functionality with our backend services, so that all application components work together seamlessly.

#### Acceptance Criteria

1. WHEN implementing backend communication THEN the system SHALL use Axum for the web server framework
2. WHEN implementing worker functionality THEN the system SHALL integrate with the cpc-node worker system
3. WHEN handling long-running jobs THEN the system SHALL use gRPC Server Streaming between backend and workers
4. WHEN implementing API endpoints THEN the system SHALL follow the established GraphQL patterns for UI communication
5. WHEN integrating with existing services THEN the system SHALL maintain compatibility with the current service architecture
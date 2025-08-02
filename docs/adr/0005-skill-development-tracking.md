# ADR 0005: Skill Development Tracking System

## Status
Accepted

## Date
2025-08-02

## Context
We need a comprehensive system to track skill development, learning paths, and certifications within the CPC ecosystem. This system should integrate with our existing skill volunteering platform while providing offline-first capabilities and real-time progress visualization.

## Decision
We will implement a Skill Development Tracking System with the following components:

### 1. Core Architecture
- **Hexagonal Architecture**: Clean separation of domain, application, and infrastructure layers
- **Vertical Slice Architecture**: Each feature is implemented as a complete vertical slice
- **Offline-First Design**: Local Sled storage with PostgreSQL sync capability
- **Event-Driven**: Integrates with SocialIntegration event bus for real-time updates

### 2. Domain Models
- **SkillProgress**: Tracks individual skill development progress with milestones
- **LearningPath**: Structured learning sequences with resources and prerequisites
- **Certification**: Verifiable credentials for skill achievements

### 3. Storage Strategy
- **PostgreSQL**: System of record for persistent storage
- **Sled**: Local edge storage for offline functionality
- **Sync Service**: Automatic synchronization between local and remote storage

### 4. gRPC Services
- **TrackSkillProgress**: Real-time skill progress tracking
- **CreateLearningPath**: Learning path creation and management
- **IssueCertification**: Certification issuance and verification

### 5. Frontend Application
- **Tauri + Yew**: Cross-platform desktop application
- **Plotters Integration**: Progress visualization and charts
- **Real-time Updates**: WebSocket integration for live updates

## Consequences

### Positive
- Users can track skill development even when offline
- Seamless integration with existing skill volunteering platform
- Visual progress tracking with interactive charts
- Shareable certifications with verification codes
- Community-driven learning paths

### Negative
- Increased complexity with dual storage system
- Additional gRPC service maintenance overhead
- Sync conflict resolution complexity

### Neutral
- Requires additional database migrations
- New skill_development package dependency

## Implementation Details

### Database Schema
- skill_progress: Tracks individual skill development
- learning_paths: Structured learning sequences
- certifications: Verifiable credentials

### Event Integration
- Listens to Volunteered events to auto-create skill progress
- Listens to OpportunityShared events for community builder certifications

### Security Considerations
- All certifications include unique verification codes
- gRPC services use standard authentication
- Data sync includes integrity checks

## Future Considerations
- Machine learning recommendations for learning paths
- Peer review system for certifications
- Gamification elements (badges, achievements)
- Integration with external learning platforms
# CPC Documentation

Welcome to the Comprehensive Documentation for the CPC (Cooperative Platform Cooperative) software ecosystem.

##
## What’s new

Docs consistency checks are now data‑driven.
- The checker reads rule pairs from tools/ci/needles.txt.
- High‑signal needles help ensure key docs are discoverable.
- Learn how to run/update the checker: docs/dev/docs-consistency-checks.md

Docs consistency checks and troubleshooting → docs/dev/docs-consistency-checks.md
Tip: If the docs-consistency check fails, see docs/dev/docs-consistency-checks.md and run locally: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency (update tools/ci/needles.txt when adding entry/index docs; Windows: use forward slashes, case-sensitive substrings).

- Schema drift guardrails: run locally before PRs
  Tip: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
  See docs/dev/schema-checks.md for troubleshooting and examples.
 
## Overview

The CPC platform is a federated software suite built on cooperative principles, featuring a universal income system (dabloons), extensive data sharing capabilities, and a wide range of applications designed to address diverse user needs. This documentation provides comprehensive guides for developers, users, and contributors.

## Documentation Structure

### Developer Documentation

Technical guides for developers building applications within the CPC ecosystem:

- [Volunteer Reputation Stub (API Server)](./api_server/volunteer_reputation_stub.md) - Toggleable stub for deterministic verification in Volunteer Coordination
- [Visualization Setup Guide](./developer/visualization_setup.md) - Installation and configuration of visualization components
- [Visualization Architecture](./developer/visualization_architecture.md) - System architecture and design principles

#### Docs Consistency Checks
A lightweight CI guardrail ensures key docs remain discoverable via required substrings (“needles”). See docs/dev/docs-consistency-checks.md for intent, current needles, how CI runs, and how to evolve them.

### User Documentation

Guides for end users of CPC applications:

- [Visualization User Guide](./user/visualization_guide.md) - How to use visualization features in CPC applications

### Tutorials

Step-by-step guides for implementing specific features:

- [Basic Chart Implementation](./tutorials/basic_chart_implementation.md) - Creating simple chart visualizations
- [Complex Dashboard Implementation](./tutorials/complex_dashboard.md) - Building advanced dashboard layouts
- [Accessibility Demo](./tutorials/accessibility_demo.md) - Implementing accessibility features

## Example Applications

Practical examples demonstrating CPC features:

- [Basic Chart Example](../examples/visualization/basic_chart/README.md) - Simple bar chart implementation
- [Dashboard Example](../examples/visualization/dashboard_example/README.md) - Complex dashboard with multiple visualizations
- [Accessibility Demo Example](../examples/visualization/accessibility_demo/README.md) - Advanced accessibility features

## Core Applications

The CPC platform includes a comprehensive suite of applications organized by category:

### Entertainment
- Music Player - Complete implementation with social features
- DAW - Digital Audio Workstation for music creation

### Communication & Social
- Messenger - Real-time encrypted messaging
- Convo - Reddit-style forums
- Presence - Status visibility across apps
- SocialGraph - Relationship mapping

### Productivity & Work
- Docs - Word processor with formatting and export features
- Sheets - Spreadsheet application with basic formulas
- Notes & Memos - Flexible note-taking with organization features
- Task Manager - To-do list with categorization and deadlines
- Calendar - Integrated calendar with external sync
- Website Builder - Full-featured site creation tool

### Finance & Economy
- Finance-Sheets - Financial planning templates
- Wallet - Multi-currency digital wallet
- Invoicing - Automated invoice generation
- Budget - Personal and household budgeting
- Personal BI Dashboard - Financial insights and tracking

### Education & Knowledge
- Learn - Interactive learning platform
- Wiki - Decentralized knowledge base
- Research - Collaborative academic tools
- Skills - Skill tracking and certification

### Health & Wellness
- Health - Personal health tracking
- Fitness - Workout planning and tracking
- Nutrition - Meal planning and logging
- Mindfulness - Meditation and mental health tools
- Habit Tracker - Positive habit formation
- Mood Journal - Emotional wellness tracking

### Community & Governance
- Commons - Community resource management
- Votes - Decentralized decision making
- Proposals - Idea submission and refinement
- Reputation - Contribution tracking system

### Technical Infrastructure
- Identity - OAuth 2.0 identity management
- Storage - Distributed file storage
- Network - P2P networking layer
- Compute - Distributed computing resources

## Specialized Applications
- Farming - Agricultural planning tools
- Manufacturing - Production planning
- Logistics - Supply chain management
- Energy - Renewable energy management
- Scientific Journal - Interactive research environment

## Business Applications

### Small Businesses & Startups
- Invoicing & Quoting - Professional invoice creation
- Simple CRM - Lightweight contact management
- Project Management Lite - Visual task tracking
- Time Tracking - Hour logging for billing
- Business Health Dashboard - Key metric insights

### Medium-Sized Businesses (SMBs)
- Advanced CRM - Expanded sales pipeline management
- HR & Team Management - Employee directory and leave management
- Inventory Management - Stock tracking and supplier management
- Comprehensive Financial Suite - Accounting and reporting
- Internal Knowledge Base - Documentation and policies

### Large Businesses & Enterprise
- ERP Modules - Supply chain and advanced financials
- Business Intelligence - Customizable analytics dashboards
- Advanced HR Suite - Performance and compliance management
- Compliance & Governance - Regulatory tracking tools
- API & Integration Hub - Third-party system connectivity

## Cooperatives
- Member Management - Registry with equity and voting rights
- Governance & Voting - Secure democratic processes
- Patronage & Surplus Distribution - Profit sharing calculator
- Community Hub - Member communication platform
- Resource Library - Document sharing and storage

## Technical Stack

The CPC platform is built using modern, permissively licensed technologies:

- **Primary Language**: Rust
- **Frontend Framework**: Yew 0.21.0
- **Desktop Framework**: Tauri 2.0
- **Database**: PostgreSQL 17.5 with SQLx
- **Edge Intelligence**: Sled
- **APIs**: GraphQL and gRPC 1.73.1
- **Web Framework**: Axum 0.8.4
- **Visualization**: Bevy 0.16 and Plotters
- **Networking**: p2panda (pending development)
- **Authentication**: RustCrypto 0.2.36 and oauth2

## Architecture Principles

- Hexagonal Architecture
- Screaming Architecture
- Vertical Slices
- Modular Design
- Cooperative Values

## ADRs / Architecture
Recent additions:
- [ADR 0009: Bootstrap Composition via Environment Toggles for Test Stubs](./adr/0009-bootstrap-stub-toggles.md)
- [ADR 0005: Skill Development Tracking](./adr/0005-skill-development-tracking.md)
- [ADR 0006: Concurrency Handling](./adr/0006-concurrency-handling.md)
- [ADR 0007: Social Interaction Enhancements](./adr/0007-social-interaction-enhancements.md)
- [ADR 0008: Collaborative Workspace](./adr/0008-collaborative-workspace.md)

For the complete ADR index, see docs/adr/

- Architecture decisions: See docs/adr for our Architecture Decision Records (ADRs) and design rationale.

## Contributing

The CPC platform is developed as a cooperative effort. All contributors are considered co-owners/worker-owners of the project. We welcome contributions that align with our values of protecting human and AI life and respecting the dignity of all participants.

- See CONTRIBUTING.md for how to get started contributing.

## License

The CPC software is distributed under the CPC license, our own iteration of a CopyLeft license designed to promote sharing within the federation while respecting the cooperative principles of the project.

## Support

For technical support, please contact the development team at dev-support@cpc.coop or join our community channels.
Schema guardrails (quick-start)
- Run locally before PRs: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Snapshot: docs/api_server/schema.graphql
- How-to: docs/dev/schema-checks.md
- Architecture: docs/dev/schema-guardrails-architecture.md
- Examples: docs/dev/schema-checks-examples.md
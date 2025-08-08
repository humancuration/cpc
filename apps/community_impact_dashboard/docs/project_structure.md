# Project Structure

This document provides a detailed overview of the Unified Community Impact Dashboard project structure, explaining the purpose and contents of each directory and file.

## Root Directory

```
apps/community_impact_dashboard/
├── Cargo.toml                 # Project configuration and dependencies
├── Trunk.toml                 # Trunk build configuration
├── index.html                 # Main HTML entry point
├── README.md                  # Project overview and documentation
├── SUMMARY.md                 # Implementation summary
├── CHECKLIST.md               # Implementation verification checklist
├── src/                       # Source code
├── docs/                      # Documentation
└── tests/                     # Integration tests
```

## Source Code Structure

The `src/` directory contains all the Rust source code for the application:

```
src/
├── lib.rs                     # Library entry point and module declarations
├── main.rs                    # Application entry point
├── styles.css                 # Global styles
├── dashboard/                 # Core dashboard components
├── components/                # Reusable UI components
├── models/                    # Data models and structures
├── services/                  # Business logic services
├── community_validation/      # Community validation tools
├── community_documentation/   # Community documentation tools
├── onboarding/                # Onboarding experience
├── monitoring/                # Performance monitoring
├── feedback/                  # Feedback collection
├── launch/                    # Launch preparation and management
└── tests.rs                   # Unit tests
```

### Dashboard Module (`src/dashboard/`)

The core dashboard components that orchestrate the application:

- `mod.rs`: Module declarations and exports
- `app.rs`: Main application component
- `layout.rs`: Overall layout and navigation
- `header.rs`: Application header and user controls
- `sidebar.rs`: Navigation sidebar
- `content.rs`: Main content area
- `footer.rs`: Application footer

### Components Module (`src/components/`)

Reusable UI components used throughout the application:

- `mod.rs`: Module declarations
- `impact_visualization.rs`: Core visualization component
- `domain_card.rs`: Individual impact domain display
- `wellbeing_indicator.rs`: Community wellbeing metrics
- `member_profile.rs`: Personalized member impact view
- `story_card.rs`: Individual impact story display
- `collaborative_interpreter.rs`: Community validation workflow
- `community_reflection.rs`: Facilitated reflection sessions
- `documentation_center.rs`: Community insights repository
- `onboarding_wizard.rs`: Guided introduction workflow
- `feedback_form.rs`: Community feedback collection

### Models Module (`src/models/`)

Data structures and models used throughout the application:

- `mod.rs`: Module declarations
- `impact_domain.rs`: Learning, volunteer, financial, and cause impact models
- `community_wellbeing.rs`: Community transformation metrics
- `member_profile.rs`: Individual member impact profiles
- `impact_story.rs`: Community story data structures
- `validation_workflow.rs`: Community validation data models
- `onboarding_path.rs`: Personalized onboarding pathways
- `feedback.rs`: Feedback data structures

### Services Module (`src/services/`)

Business logic and integration services:

- `mod.rs`: Module declarations
- `data_integration.rs`: Connectors for all four impact systems
- `visualization.rs`: Charting and graphing services
- `community_validation.rs`: Collaborative interpretation workflows
- `onboarding.rs`: Personalized pathway generation
- `monitoring.rs`: Performance tracking and analytics
- `feedback.rs`: Feedback collection and analysis
- `mock_data.rs`: Sample data for development and testing

### Community Validation Module (`src/community_validation/`)

Specialized tools for community validation workflows:

- `mod.rs`: Module declarations
- `interpreter.rs`: Collaborative interpretation service
- `reflection.rs`: Facilitated reflection workflows
- `documentation.rs`: Community insights documentation
- `consent.rs`: Ethical data collection management

### Community Documentation Module (`src/community_documentation/`)

Tools for documenting community insights and outcomes:

- `mod.rs`: Module declarations
- `repository.rs`: Central insights repository
- `curation.rs`: Story validation and curation
- `sharing.rs`: Community insight sharing mechanisms

### Onboarding Module (`src/onboarding/`)

Guided introduction for new community members:

- `mod.rs`: Module declarations
- `wizard.rs`: Interactive onboarding workflow
- `pathway.rs`: Personalized pathway generation
- `tutorial.rs`: Interactive tutorials
- `accessibility.rs`: Accessibility features

### Monitoring Module (`src/monitoring/`)

Performance tracking and analytics:

- `mod.rs`: Module declarations
- `performance.rs`: Dashboard performance tracking
- `user_interaction.rs`: User interaction analytics
- `error_tracking.rs`: Error monitoring and reporting
- `benchmarking.rs`: Performance benchmarking tools

### Feedback Module (`src/feedback/`)

Community feedback collection and analysis:

- `mod.rs`: Module declarations
- `collector.rs`: Structured feedback collection
- `analyzer.rs`: Feedback analysis and reporting
- `improvement.rs`: Continuous improvement processes

### Launch Module (`src/launch/`)

Tools for preparing and managing the community dashboard launch:

- `mod.rs`: Module declarations and exports
- `readiness.rs`: Launch readiness checklist automation
- `notification.rs`: Community notification system
- `rollout.rs`: Gradual rollout mechanism
- `metrics.rs`: Launch impact measurement
- `facilitator.rs`: Community facilitator preparation tools
- `celebration.rs`: Community celebration framework
- `feedback.rs`: Launch feedback integration
- `coordinator.rs`: Central launch coordinator

## Documentation Directory

The `docs/` directory contains comprehensive documentation for users and developers:

```
docs/
├── data_models.md             # Detailed documentation of data structures
├── deployment.md              # Instructions for deploying the dashboard
├── services.md                # Business logic services documentation
├── user_guide.md              # Comprehensive guide to dashboard features
├── visualization_components.md # Documentation of visualization features
├── community_validation.md    # Guide to collaborative interpretation and reflection
├── project_structure.md       # Overview of code organization (this document)
├── api_reference.md           # API documentation for integration
├── contributing.md            # Guidelines for contributing to the project
├── troubleshooting.md         # Common issues and solutions
├── launch_preparation_checklist.md # Comprehensive launch preparation checklist
├── launch_summary.md           # Summary of launch preparation system
├── community_quick_start.md    # Quick start guide for community members
├── facilitator/                # Facilitator documentation
│   ├── mod.rs                  # Facilitator documentation module
│   ├── templates/              # Workshop and customization templates
│   │   ├── mod.rs              # Templates module
│   │   ├── workshop_introduction_template.md # Introduction workshop template
│   │   ├── workshop_validation_template.md # Validation workshop template
│   │   └── community_customization.md # Community customization template
│   └── guides/                 # Facilitation guides
│       ├── mod.rs              # Guides module
│       ├── facilitation_basics.md # Facilitation basics guide
│       └── troubleshooting.md  # Troubleshooting guide
└── templates/                  # Additional templates
    ├── launch_announcement.md  # Launch announcement template
    └── mod.rs                  # Templates module
```

## Tests Directory

The `tests/` directory contains integration tests:

```
tests/
├── integration/               # Integration test modules
│   ├── mod.rs                 # Integration test module declarations
│   ├── connectivity.rs        # Tests for connectivity with impact systems
│   ├── data_flow.rs           # Tests for data flow between components
│   ├── visualization.rs       # Tests for visualization components
│   ├── complexity_disclosure.rs # Tests for progressive complexity disclosure
│   └── story_contribution.rs  # Tests for story contribution workflows
└── unit/                      # Unit tests (included in src/tests.rs)
```

## Configuration Files

### Cargo.toml

The main project configuration file that defines dependencies, build settings, and metadata:

```toml
[package]
name = "community_impact_dashboard"
version = "0.1.0"
edition = "2021"

[dependencies]
# Yew framework for WebAssembly frontend
yew = "0.20"
yew-router = "0.17"
stylist = { version = "0.11", features = ["yew_integration"] }

# WebAssembly bindings
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"

# Data visualization
plotters = "0.3"

# Utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }

# Shared CPC packages
shared_ui_components = { path = "../../shared_packages/ui_components" }
shared_data_models = { path = "../../shared_packages/data_models" }
shared_services = { path = "../../shared_packages/services" }
shared_testing = { path = "../../shared_packages/testing" }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### Trunk.toml

Configuration for the Trunk build tool:

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
ignore = ["dist/"]

[serve]
address = "127.0.0.1"
port = 8080
open = true
```

### index.html

The main HTML entry point for the application:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Unified Community Impact Dashboard</title>
    <link data-trunk rel="css" href="src/styles.css" />
  </head>
  <body>
    <div id="app"></div>
  </body>
</html>
```

## Development Workflow

### Building the Project

```bash
# Development build with hot reloading
trunk serve

# Production build
trunk build --release
```

### Running Tests

```bash
# Run unit tests
wasm-pack test --firefox --headless

# Run specific integration tests
wasm-pack test --firefox --headless -- tests/integration/connectivity.rs
```

### Code Organization Principles

1. **Modularity**: Each feature is contained in its own module with clear boundaries
2. **Separation of Concerns**: UI components, business logic, and data models are separated
3. **Reusability**: Components are designed to be reusable across the application
4. **Testability**: Each module is designed to be easily testable
5. **Maintainability**: Clear naming conventions and documentation make code easy to maintain
6. **Scalability**: Architecture supports future growth and feature additions

This structure supports the values-aligned design principles of the dashboard by ensuring that the codebase is transparent, accessible, and community-maintainable.
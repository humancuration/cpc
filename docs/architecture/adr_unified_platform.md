# ADR-001: Unified Web/Desktop Platform Architecture

## Status
Proposed

## Context
Our application suite needs to support both web and native desktop environments while maximizing code reuse and maintaining consistent user experiences. The current separate implementations for web and desktop are inefficient and create maintenance challenges.

## Decision
We will implement a unified architecture using:
- **Tauri** for desktop application framework
- **Yew** for UI components (using Rust/WASM)
- Shared Rust core logic cross-compiled for both platforms
- Conditional compilation for platform-specific features

## Directory Structure
```
apps/
  launcher/       # Desktop entry point (Tauri)
  web_portal/     # Browser entry point (Yew)
shared_packages/
  core_app_logic/ # Business logic shared across platforms
  ui_toolkit/     # Reusable UI components
```

## Consequences
### Advantages
- 80%+ code reuse between web and desktop
- Consistent UI/UX across platforms
- Single codebase for business logic
- Native desktop features via Tauri

### Challenges
- Learning curve for Tauri/Yew integration
- WASM size optimization for web
- Platform-specific testing requirements

## Integration Strategy
1. Shared state via Yew Context API
2. Feature flags for platform-specific code
3. Common design system in `ui_toolkit`
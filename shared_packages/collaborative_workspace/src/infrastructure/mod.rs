/*!
Infrastructure layer for collaborative workspace.

Exports:
- event_bus: adapters to publish collaboration events via the shared SocialEventBus
- postgres_repository: SQLx-backed repositories for documents, projects, files, and meetings
*/
pub mod event_bus;
pub mod postgres_repository;
/*!
Application module exports for collaborative workspace.

These are vertical-slice service modules that depend on domain repositories and
the shared SocialEventBus as needed. Concrete service implementations will be
added in their respective files.
*/
pub mod document_service;
pub mod project_service;
pub mod file_service;
pub mod meeting_service;
pub mod notification_integration;
// pub mod whiteboard_service; // Placeholder for future whiteboard functionality
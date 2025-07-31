//! Yew components for the consent manager UI.

pub mod consent_dashboard;
pub mod permission_toggle;
pub mod audit_log_viewer;
pub mod indicators;

pub use consent_dashboard::ConsentDashboard;
pub use permission_toggle::PermissionToggle;
pub use audit_log_viewer::AuditLogViewer;
pub use indicators::ConsentIndicator;
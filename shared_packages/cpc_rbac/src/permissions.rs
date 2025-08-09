// Permission constants and utilities
pub const READ_PERMISSION: &str = "read";
pub const WRITE_PERMISSION: &str = "write";
pub const DELETE_PERMISSION: &str = "delete";
pub const ADMIN_PERMISSION: &str = "admin";

pub fn format_permission(resource: &str, action: &str) -> String {
    format!("{}:{}", resource, action)
}

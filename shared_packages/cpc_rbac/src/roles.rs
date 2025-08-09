// Standard role definitions
use crate::{Permission, Role};
use uuid::Uuid;

pub fn create_admin_role() -> Role {
    Role {
        id: Uuid::new_v4(),
        name: "admin".to_string(),
        permissions: vec![Permission {
            id: Uuid::new_v4(),
            name: "admin_all".to_string(),
            resource: "*".to_string(),
            action: "*".to_string(),
        }],
    }
}

pub fn create_user_role() -> Role {
    Role {
        id: Uuid::new_v4(),
        name: "user".to_string(),
        permissions: vec![Permission {
            id: Uuid::new_v4(),
            name: "read_own".to_string(),
            resource: "user".to_string(),
            action: "read".to_string(),
        }],
    }
}

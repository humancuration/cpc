// Community management functionality
use uuid::Uuid;

pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Community {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
        }
    }
}

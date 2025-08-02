use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SkillProgress {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub user_id: Uuid,
    #[validate(range(min = 0, max = 100))]
    pub progress: u8, // 0-100
    pub updated_at: DateTime<Utc>,
}

impl SkillProgress {
    pub fn new(skill_id: Uuid, user_id: Uuid, progress: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            skill_id,
            user_id,
            progress: progress.min(100),
            updated_at: Utc::now(),
        }
    }

    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_skill_progress_creation() {
        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let progress = SkillProgress::new(skill_id, user_id, 75);

        assert_eq!(progress.skill_id, skill_id);
        assert_eq!(progress.user_id, user_id);
        assert_eq!(progress.progress, 75);
    }

    #[test]
    fn test_skill_progress_update() {
        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut progress = SkillProgress::new(skill_id, user_id, 50);

        progress.update_progress(80);
        assert_eq!(progress.progress, 80);

        // Test that progress is capped at 100
        progress.update_progress(150);
        assert_eq!(progress.progress, 100);
    }

    #[test]
    fn test_skill_progress_validation() {
        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let progress = SkillProgress::new(skill_id, user_id, 75);

        let validation = progress.validate();
        assert!(validation.is_ok());
    }
}
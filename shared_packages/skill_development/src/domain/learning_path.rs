use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LearningPath {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub milestones: Vec<Milestone>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Milestone {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub is_completed: bool,
    pub order_index: i32,
    pub estimated_duration_hours: Option<i32>,
}

impl LearningPath {
    pub fn new(
        user_id: Uuid,
        skill_id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            skill_id,
            title,
            description,
            milestones: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_milestone(&mut self, milestone: Milestone) {
        self.milestones.push(milestone);
        self.updated_at = Utc::now();
    }

    pub fn complete_milestone(&mut self, milestone_id: Uuid) -> bool {
        for milestone in &mut self.milestones {
            if milestone.id == milestone_id {
                milestone.is_completed = true;
                self.updated_at = Utc::now();
                return true;
            }
        }
        false
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.milestones.is_empty() {
            return 0.0;
        }

        let completed = self
            .milestones
            .iter()
            .filter(|m| m.is_completed)
            .count() as f32;

        (completed / self.milestones.len() as f32) * 100.0
    }
}

impl Milestone {
    pub fn new(
        title: String,
        description: Option<String>,
        order_index: i32,
        estimated_duration_hours: Option<i32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            is_completed: false,
            order_index,
            estimated_duration_hours,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_path_creation() {
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            Some("Learn Rust from basics to advanced".to_string()),
        );

        assert_eq!(path.user_id, user_id);
        assert_eq!(path.skill_id, skill_id);
        assert_eq!(path.title, "Rust Programming");
        assert_eq!(path.milestones.len(), 0);
    }

    #[test]
    fn test_milestone_creation() {
        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            Some("Learn about variables and data types in Rust".to_string()),
            1,
            Some(2),
        );

        assert_eq!(milestone.title, "Variables and Data Types");
        assert_eq!(milestone.order_index, 1);
        assert_eq!(milestone.estimated_duration_hours, Some(2));
        assert!(!milestone.is_completed);
    }

    #[test]
    fn test_add_milestone() {
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let mut path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            None,
        );

        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            None,
            1,
            Some(2),
        );

        path.add_milestone(milestone);
        assert_eq!(path.milestones.len(), 1);
    }

    #[test]
    fn test_complete_milestone() {
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let mut path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            None,
        );

        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            None,
            1,
            Some(2),
        );

        let milestone_id = milestone.id;
        path.add_milestone(milestone);

        assert!(path.complete_milestone(milestone_id));
        assert!(path.milestones[0].is_completed);
    }

    #[test]
    fn test_progress_percentage() {
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let mut path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            None,
        );

        // Test with no milestones
        assert_eq!(path.progress_percentage(), 0.0);

        // Add milestones
        let mut milestone1 = Milestone::new(
            "Variables".to_string(),
            None,
            1,
            Some(2),
        );
        milestone1.is_completed = true;

        let milestone2 = Milestone::new(
            "Functions".to_string(),
            None,
            2,
            Some(3),
        );

        path.add_milestone(milestone1);
        path.add_milestone(milestone2);

        // 1 out of 2 completed = 50%
        assert_eq!(path.progress_percentage(), 50.0);
    }

    #[test]
    fn test_learning_path_validation() {
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            Some("Learn Rust from basics to advanced".to_string()),
        );

        let validation = path.validate();
        assert!(validation.is_ok());
    }

    #[test]
    fn test_milestone_validation() {
        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            Some("Learn about variables and data types in Rust".to_string()),
            1,
            Some(2),
        );

        let validation = milestone.validate();
        assert!(validation.is_ok());
    }
}
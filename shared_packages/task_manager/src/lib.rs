pub mod task_core;

#[cfg(test)]
mod tests {
    use super::task_core::entities::*;
    use super::task_core::value_objects::*;
    use super::task_core::services::*;
    use super::task_core::errors::*;
    use uuid::Uuid;
    use chrono::{Utc, NaiveDate};

    #[test]
    fn test_due_date_validation() {
        // Test that due dates in the past are rejected
        let past_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert!(DueDate::new(past_date).is_err());

        // Test that due dates in the future are accepted
        let future_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        assert!(DueDate::new(future_date).is_ok());
    }

    #[test]
    fn test_progress_percentage_validation() {
        // Test that values above 100 are rejected
        assert!(ProgressPercentage::new(101).is_err());

        // Test that values within range are accepted
        assert!(ProgressPercentage::new(50).is_ok());
        assert!(ProgressPercentage::new(0).is_ok());
        assert!(ProgressPercentage::new(100).is_ok());
    }

    #[test]
    fn test_task_cannot_depend_on_itself() {
        let mut task = Task {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Try to add the task as a dependency of itself
        assert!(DependencyResolver::add_dependency(&mut task, task.id).is_err());
    }

    #[test]
    fn test_completed_task_cannot_be_assigned() {
        let mut task = Task {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::Completed,
            progress: ProgressPercentage(100),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Try to assign a completed task
        let assignee_id = Uuid::new_v4();
        assert!(TaskAssignmentService::assign_task(&mut task, assignee_id).is_err());
    }

    #[test]
    fn test_progress_calculator_updates_status() {
        let mut task = Task {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::InProgress,
            progress: ProgressPercentage(50),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Update progress to 100% and check that status is updated
        assert!(ProgressCalculator::update_progress(&mut task, 100).is_ok());
        assert_eq!(task.status, TaskStatus::Completed);
    }
        
    #[test]
    fn test_completed_task_cannot_be_modified() {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::Completed,
            progress: ProgressPercentage(100),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Try to update progress of a completed task
        assert!(ProgressCalculator::update_progress(&mut task, 50).is_err());
    }
    }

    #[test]
    fn test_high_priority_tasks_must_be_assigned() {
        let task = Task {
            id: Uuid::new_v4(),
            title: "High Priority Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::High,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test that high-priority tasks must be assigned
        let mut task = task.clone();
        let assignee_id = Uuid::new_v4();
        assert!(TaskAssignmentService::assign_task(&mut task, assignee_id).is_ok());
        
        // Test that unassigned high-priority tasks are rejected
        let unassigned_task = Task {
            id: Uuid::new_v4(),
            title: "High Priority Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::High,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        // Create a nil UUID to represent an unassigned task
        let nil_uuid = Uuid::nil();
        assert!(TaskAssignmentService::assign_task(&mut unassigned_task, nil_uuid).is_err());
    }

    #[test]
    fn test_recurring_task_generation() {
        let task = Task {
            id: Uuid::new_v4(),
            title: "Recurring Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: Some(RecurrenceRule {
                pattern: RecurrencePattern::Daily,
                interval: 1,
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test that the RecurrenceGenerator can generate a next task
        let next_task = RecurrenceGenerator::generate_next_task(&task);
        // Our implementation now returns a new Task instance for recurring tasks
        assert!(next_task.is_some());
        
        let next_task = next_task.unwrap();
        assert_eq!(next_task.title, task.title);
        assert_eq!(next_task.priority, task.priority);
    }
}
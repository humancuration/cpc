use super::entities::*;
use super::value_objects::*;
use super::errors::DomainError;
use uuid::Uuid;

pub struct TaskAssignmentService;

impl TaskAssignmentService {
    pub fn assign_task(task: &mut Task, assignee: Uuid) -> Result<(), DomainError> {
        if task.status == TaskStatus::Completed {
            return Err(DomainError::InvalidOperation("Cannot assign completed task"));
        }
        
        // Enforce that high-priority tasks must be assigned
        if matches!(task.priority, TaskPriority::High | TaskPriority::Critical) && assignee.is_nil() {
            return Err(DomainError::InvalidOperation("High-priority tasks must be assigned"));
        }
        
        task.assignee = Some(assignee);
        Ok(())
    }
}

pub struct DependencyResolver;

impl DependencyResolver {
    pub fn add_dependency(task: &mut Task, dependency_id: Uuid) -> Result<(), DomainError> {
        if task.id == dependency_id {
            return Err(DomainError::InvalidDependency("Cannot depend on self"));
        }
        
        // Check for circular dependencies
        if task.dependencies.has_circular_dependency(dependency_id) {
            return Err(DomainError::InvalidDependency("Circular dependency detected"));
        }
        
        task.dependencies.add_dependency(dependency_id)
    }
    }
}

pub struct RecurrenceGenerator;

impl RecurrenceGenerator {
    pub fn generate_next_task(task: &Task) -> Option<Task> {
        if let Some(recurrence) = &task.recurrence {
            // In a real implementation, this would generate a new task based on the recurrence rule
            // For now, we'll just return a basic implementation
            Some(Task {
                id: Uuid::new_v4(),
                title: task.title.clone(),
                description: task.description.clone(),
                due_date: None, // Would be calculated based on recurrence rule
                priority: task.priority,
                status: TaskStatus::NotStarted,
                progress: ProgressPercentage(0),
                assignee: task.assignee,
                dependencies: DependencyGraph::new(),
                recurrence: Some(recurrence.clone()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        } else {
            None
        }
    }
}

pub struct ProgressCalculator;

impl ProgressCalculator {
    pub fn update_progress(task: &mut Task, progress: u8) -> Result<(), DomainError> {
        if task.status == TaskStatus::Completed {
            return Err(DomainError::InvalidOperation("Cannot modify completed task"));
        }
        
        task.progress = ProgressPercentage::new(progress)?;
        if progress == 100 {
            task.status = TaskStatus::Completed;
        }
        Ok(())
    }
}
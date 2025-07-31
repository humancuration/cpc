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
        task.dependencies.push(dependency_id);
        Ok(())
    }
}

pub struct RecurrenceGenerator;

impl RecurrenceGenerator {
    pub fn generate_next_task(task: &Task) -> Option<Task> {
        // Implementation logic
        None
    }
}

pub struct ProgressCalculator;

impl ProgressCalculator {
    pub fn update_progress(task: &mut Task, progress: u8) -> Result<(), DomainError> {
        task.progress = ProgressPercentage::new(progress)?;
        if progress == 100 {
            task.status = TaskStatus::Completed;
        }
        Ok(())
    }
}
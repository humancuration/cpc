//! Conflict resolution system for collaborative document editing

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::core::{Operation, Position, CollaborationError};
use crate::presence::PresenceManager;
use crate::versioning::VersionManager;
use event_bus::{EventBus, DomainEvent};
use serde_json::json;

/// Represents a conflict between operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub id: Uuid,
    pub document_id: Uuid,
    pub conflicting_operations: Vec<Operation>,
    pub resolution_strategy: ResolutionStrategy,
    pub resolved: bool,
    pub resolved_operations: Vec<Operation>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub metadata: ConflictMetadata,
}

/// Metadata about conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictMetadata {
    pub detection_method: String,
    pub transformation_history: Vec<TransformationRecord>,
    pub resolution_details: Option<String>,
}

/// Record of an operation transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRecord {
    pub original_operation: Operation,
    pub transformed_operation: Operation,
    pub transformation_type: String,
    pub timestamp: DateTime<Utc>,
}

/// Strategies for resolving conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    /// Operations are applied in timestamp order
    TimestampOrder,
    /// Operations are applied in user priority order
    UserPriority,
    /// Operations are merged when possible
    Merge,
    /// User intervention is required
    Manual,
}

/// Manages conflict detection and resolution
///
/// The ConflictResolver is responsible for:
/// - Detecting conflicts between concurrent operations
/// - Applying operational transformation algorithms to resolve conflicts
/// - Managing different resolution strategies (timestamp, priority, merge)
/// - Integrating with PresenceManager for QoS-based priority resolution
/// - Tracking conflict history and transformation records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolver {
    pub document_id: Uuid,
    pub conflicts: HashMap<Uuid, Conflict>,
    pub user_priorities: HashMap<Uuid, i32>,
    pub document_content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub event_bus: Option<EventBus>,
    #[serde(skip)]
    pub presence_manager: Option<PresenceManager>,
    #[serde(skip)]
    pub version_manager: Option<VersionManager>,
}

impl ConflictResolver {
    /// Create a new conflict resolver for a document
    ///
    /// # Arguments
    /// * `document_id` - The UUID of the document this resolver is for
    ///
    /// # Returns
    /// A new ConflictResolver instance
    pub fn new(document_id: Uuid) -> Self {
        Self {
            document_id,
            conflicts: HashMap::new(),
            user_priorities: HashMap::new(),
            document_content: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            event_bus: None,
            presence_manager: None,
            version_manager: None,
        }
    }
    
    /// Set the event bus for this conflict resolver
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }
    
    /// Set the presence manager for this conflict resolver
    pub fn set_presence_manager(&mut self, presence_manager: PresenceManager) {
        self.presence_manager = Some(presence_manager);
    }
    
    /// Set the version manager for this conflict resolver
    pub fn set_version_manager(&mut self, version_manager: VersionManager) {
        self.version_manager = Some(version_manager);
    }
    
    /// Set the document content for this conflict resolver
    pub fn set_document_content(&mut self, content: String) {
        self.document_content = content;
        self.updated_at = Utc::now();
    }

    /// Set priority for a user (higher number = higher priority)
    ///
    /// # Arguments
    /// * `user_id` - The UUID of the user
    /// * `priority` - The priority level (higher number = higher priority)
    pub fn set_user_priority(&mut self, user_id: Uuid, priority: i32) {
        self.user_priorities.insert(user_id, priority);
        self.updated_at = Utc::now();
    }
    
    /// Get user priority, considering QoS tier from presence manager
    pub fn get_user_priority(&self, user_id: Uuid) -> i32 {
        // Start with explicitly set priority
        let base_priority = *self.user_priorities.get(&user_id).unwrap_or(&0);
        
        // Add priority based on QoS tier if presence manager is available
        let qos_priority = if let Some(ref presence_manager) = self.presence_manager {
            if let Some(presence) = presence_manager.get_user_presence(user_id) {
                // Higher QoS tier means higher priority (0=critical, 1=medium, 2=low)
                match presence.qos_tier {
                    0 => 100, // Critical QoS gets highest priority boost
                    1 => 50,  // Medium QoS gets medium priority boost
                    _ => 0,   // Low QoS gets no boost
                }
            } else {
                0
            }
        } else {
            0
        };
        
        base_priority + qos_priority
    }

    /// Detect conflicts between operations
    ///
    /// This method checks for overlapping ranges between operations to identify conflicts.
    ///
    /// # Arguments
    /// * `operations` - A slice of operations to check for conflicts
    ///
    /// # Returns
    /// A vector of Conflict objects representing detected conflicts
    pub fn detect_conflicts(&self, operations: &[Operation]) -> Vec<Conflict> {
        let mut conflicts = Vec::new();
        
        // Simple conflict detection: operations at the same position
        for i in 0..operations.len() {
            for j in (i + 1)..operations.len() {
                if self.operations_conflict(&operations[i], &operations[j]) {
                    let conflict = Conflict {
                        id: Uuid::new_v4(),
                        document_id: self.document_id,
                        conflicting_operations: vec![operations[i].clone(), operations[j].clone()],
                        resolution_strategy: ResolutionStrategy::TimestampOrder,
                        resolved: false,
                        resolved_operations: vec![],
                        resolved_at: None,
                        created_at: Utc::now(),
                        metadata: ConflictMetadata {
                            detection_method: "position_overlap".to_string(),
                            transformation_history: vec![],
                            resolution_details: None,
                        },
                    };
                    conflicts.push(conflict);
                }
            }
        }
        
        conflicts
    }
    
    /// Add a conflict to be resolved
    pub fn add_conflict(&mut self, conflict: Conflict) {
        self.conflicts.insert(conflict.id, conflict.clone());
        self.updated_at = Utc::now();
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "ConflictDetected".to_string(),
                json!({
                    "document_id": self.document_id,
                    "conflict": conflict,
                }),
            );
            
            // We're ignoring the result here as we don't want to fail the operation
            // if event publishing fails
            let _ = event_bus.publish(event);
        }
    }

    /// Check if two operations conflict with each other
    fn operations_conflict(&self, op1: &Operation, op2: &Operation) -> bool {
        let (pos1_start, pos1_end) = self.get_operation_range(op1);
        let (pos2_start, pos2_end) = self.get_operation_range(op2);
        
        // Check for overlapping ranges
        !(pos1_end.line < pos2_start.line || 
          (pos1_end.line == pos2_start.line && pos1_end.column < pos2_start.column) ||
          pos2_end.line < pos1_start.line || 
          (pos2_end.line == pos1_start.line && pos2_end.column < pos1_start.column))
    }

    /// Get the range of positions affected by an operation
    fn get_operation_range(&self, operation: &Operation) -> (Position, Position) {
        match operation {
            Operation::Insert { position, text, .. } => {
                let end_position = self.calculate_end_position(position, text);
                (position.clone(), end_position)
            },
            Operation::Delete { start, end, .. } => {
                (start.clone(), end.clone())
            },
            Operation::Replace { start, end, text, .. } => {
                let end_position = self.calculate_end_position(start, text);
                (start.clone(), end_position)
            },
        }
    }

    /// Calculate the end position after inserting text
    fn calculate_end_position(&self, start: &Position, text: &str) -> Position {
        let mut line = start.line;
        let mut column = start.column;
        
        for ch in text.chars() {
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        
        Position { line, column }
    }
    
    /// Calculate the length of a range in characters
    fn calculate_range_length(&self, start: &Position, end: &Position) -> Result<usize, CollaborationError> {
        if start.line == end.line {
            Ok(end.column - start.column)
        } else {
            let line_diff = end.line - start.line;
            let first_line_chars = self.document_content.lines().nth(start.line).unwrap_or("").len() - start.column;
            let middle_lines_chars = (1..line_diff).map(|i|
                self.document_content.lines().nth(start.line + i).unwrap_or("").len()
            ).sum::<usize>();
            let last_line_chars = end.column;
            Ok(first_line_chars + middle_lines_chars + last_line_chars)
        }
    }
    
    /// Check if a position is within a range
    fn position_within_range(&self, pos: &Position, range_start: &Position, range_end: &Position) -> bool {
        // Check if pos is within [range_start, range_end)
        !(pos.line < range_start.line ||
          (pos.line == range_start.line && pos.column < range_start.column) ||
          pos.line > range_end.line ||
          (pos.line == range_end.line && pos.column >= range_end.column))
    }
    
    /// Record a transformation in the conflict metadata
    fn record_transformation(&mut self, conflict_id: Uuid, original: Operation, transformed: Operation, transformation_type: String) {
        if let Some(conflict) = self.conflicts.get_mut(&conflict_id) {
            conflict.metadata.transformation_history.push(TransformationRecord {
                original_operation: original,
                transformed_operation: transformed,
                transformation_type,
                timestamp: Utc::now(),
            });
        }
    }
    
    /// Transform operation op1 against operation op2
    ///
    /// Applies operational transformation to make op1 compatible with the effects of op2.
    ///
    /// # Arguments
    /// * `op1` - The operation to transform
    /// * `op2` - The operation to transform against
    ///
    /// # Returns
    /// A transformed version of op1, or an error if transformation fails
    pub fn transform_operation(&mut self, op1: &Operation, op2: &Operation) -> Result<Operation, CollaborationError> {
        match (op1, op2) {
            (Operation::Insert { position: pos1, text: text1, user_id: user1, timestamp: time1 },
             Operation::Insert { position: pos2, text: _text2, user_id: user2, timestamp: time2 }) => {
                self.transform_insert_vs_insert(Uuid::nil(), pos1, text1, *user1, *time1, pos2, *user2, *time2)
            },
            (Operation::Insert { position: pos1, text: text1, user_id: user1, timestamp: time1 },
             Operation::Delete { start: start2, end: end2, user_id: user2, timestamp: time2 }) => {
                self.transform_insert_vs_delete(Uuid::nil(), pos1, text1, *user1, *time1, start2, end2, *user2, *time2)
            },
            (Operation::Delete { start: start1, end: end1, user_id: user1, timestamp: time1 },
             Operation::Insert { position: pos2, text: text2, user_id: user2, timestamp: time2 }) => {
                self.transform_delete_vs_insert(Uuid::nil(), start1, end1, *user1, *time1, pos2, text2, *user2, *time2)
            },
            (Operation::Delete { start: start1, end: end1, user_id: user1, timestamp: time1 },
             Operation::Delete { start: start2, end: end2, user_id: user2, timestamp: time2 }) => {
                self.transform_delete_vs_delete(Uuid::nil(), start1, end1, *user1, *time1, start2, end2, *user2, *time2)
            },
            _ => Ok(op1.clone()),
        }
    }
    
    /// Transform insert operation against another insert operation
    fn transform_insert_vs_insert(
        &mut self,
        conflict_id: Uuid,
        pos1: &Position,
        text1: &str,
        user1: Uuid,
        time1: DateTime<Utc>,
        pos2: &Position,
        user2: Uuid,
        time2: DateTime<Utc>,
    ) -> Result<Operation, CollaborationError> {
        let transformed_pos = if pos1.line < pos2.line || (pos1.line == pos2.line && pos1.column <= pos2.column) {
            // op1 comes before or at same position as op2, no adjustment needed
            pos1.clone()
        } else {
            // op1 comes after op2, adjust position by length of inserted text
            let text2_len = text2.chars().count();
            Position {
                line: pos1.line,
                column: pos1.column + text2_len,
            }
        };
        
        // Record transformation
        let original_op = Operation::Insert {
            position: pos1.clone(),
            text: text1.to_string(),
            user_id: user1,
            timestamp: time1,
        };
        
        let transformed_op = Operation::Insert {
            position: transformed_pos.clone(),
            text: text1.to_string(),
            user_id: user1,
            timestamp: time1,
        };
        
        // Record transformation
        self.record_transformation(
            conflict_id,
            original_op.clone(),
            transformed_op.clone(),
            "insert_vs_insert".to_string(),
        );
        
        Ok(transformed_op)
    }
    
    /// Transform insert operation against delete operation
    fn transform_insert_vs_delete(
        &mut self,
        conflict_id: Uuid,
        pos1: &Position,
        text1: &str,
        user1: Uuid,
        time1: DateTime<Utc>,
        start2: &Position,
        end2: &Position,
        user2: Uuid,
        time2: DateTime<Utc>,
    ) -> Result<Operation, CollaborationError> {
        // Check if insert position is within deleted range
        if self.position_within_range(pos1, start2, end2) {
            // Insert position is within deleted range, adjust to end of deletion
            let transformed_op = Operation::Insert {
                position: end2.clone(),
                text: text1.to_string(),
                user_id: user1,
                timestamp: time1,
            };
            
            // Record transformation
            let original_op = Operation::Insert {
                position: pos1.clone(),
                text: text1.to_string(),
                user_id: user1,
                timestamp: time1,
            };
            
            self.record_transformation(
                conflict_id,
                original_op,
                transformed_op.clone(),
                "insert_vs_delete_position_adjust".to_string(),
            );
            
            return Ok(transformed_op);
        }
        
        // Check if insert position is after deleted range
        if self.position_after_range(pos1, end2) {
            // Adjust position by length of deleted text
            let deleted_chars = self.calculate_range_length(start2, end2)?;
            let transformed_pos = Position {
                line: pos1.line,
                column: pos1.column.saturating_sub(deleted_chars),
            };
            
            let transformed_op = Operation::Insert {
                position: transformed_pos,
                text: text1.to_string(),
                user_id: user1,
                timestamp: time1,
            };
            
            // Record transformation
            let original_op = Operation::Insert {
                position: pos1.clone(),
                text: text1.to_string(),
                user_id: user1,
                timestamp: time1,
            };
            
            self.record_transformation(
                conflict_id,
                original_op,
                transformed_op.clone(),
                "insert_vs_delete_position_shift".to_string(),
            );
            
            return Ok(transformed_op);
        }
        
        // Insert position is before deleted range, no adjustment needed
        let transformed_op = Operation::Insert {
            position: pos1.clone(),
            text: text1.to_string(),
            user_id: user1,
            timestamp: time1,
        };
        
        // Record transformation
        let original_op = Operation::Insert {
            position: pos1.clone(),
            text: text1.to_string(),
            user_id: user1,
            timestamp: time1,
        };
        
        self.record_transformation(
            conflict_id,
            original_op,
            transformed_op.clone(),
            "insert_vs_delete_no_change".to_string(),
        );
        
        Ok(transformed_op)
    }
    
    /// Transform delete operation against insert operation
    fn transform_delete_vs_insert(
        &mut self,
        conflict_id: Uuid,
        start1: &Position,
        end1: &Position,
        user1: Uuid,
        time1: DateTime<Utc>,
        pos2: &Position,
        text2: &str,
        user2: Uuid,
        time2: DateTime<Utc>,
    ) -> Result<Operation, CollaborationError> {
        let (transformed_start, transformed_end) = if self.position_within_range(pos2, start1, end1) {
            // Insert position is within delete range, extend delete range
            let text2_len = text2.chars().count();
            let transformed_end = Position {
                line: end1.line,
                column: end1.column + text2_len,
            };
            (start1.clone(), transformed_end)
        } else if self.position_after_range(pos2, end1) {
            // Insert position is after delete range, no adjustment needed
            (start1.clone(), end1.clone())
        } else {
            // Insert position is before delete range, shift delete range
            let text2_len = text2.chars().count();
            let transformed_start = Position {
                line: start1.line,
                column: start1.column + text2_len,
            };
            let transformed_end = Position {
                line: end1.line,
                column: end1.column + text2_len,
            };
            (transformed_start, transformed_end)
        };
        
        let transformed_op = Operation::Delete {
            start: transformed_start.clone(),
            end: transformed_end.clone(),
            user_id: user1,
            timestamp: time1,
        };
        
        // Record transformation
        let original_op = Operation::Delete {
            start: start1.clone(),
            end: end1.clone(),
            user_id: user1,
            timestamp: time1,
        };
        
        let transformation_type = if self.position_within_range(pos2, start1, end1) {
            "delete_vs_insert_extend"
        } else if self.position_after_range(pos2, end1) {
            "delete_vs_insert_no_change"
        } else {
            "delete_vs_insert_shift"
        };
        
        self.record_transformation(
            conflict_id,
            original_op,
            transformed_op.clone(),
            transformation_type.to_string(),
        );
        
        Ok(transformed_op)
    }
    
    /// Transform delete operation against another delete operation
    fn transform_delete_vs_delete(
        &mut self,
        conflict_id: Uuid,
        start1: &Position,
        end1: &Position,
        user1: Uuid,
        time1: DateTime<Utc>,
        start2: &Position,
        end2: &Position,
        user2: Uuid,
        time2: DateTime<Utc>,
    ) -> Result<Operation, CollaborationError> {
        // Handle different cases of overlapping deletions
        let transformed_op = if start2.line > end1.line || (start2.line == end1.line && start2.column >= end1.column) {
            // op2 is completely after op1, no adjustment needed
            Operation::Delete {
                start: start1.clone(),
                end: end1.clone(),
                user_id: user1,
                timestamp: time1,
            }
        } else if end2.line < start1.line || (end2.line == start1.line && end2.column <= start1.column) {
            // op2 is completely before op1, no adjustment needed
            Operation::Delete {
                start: start1.clone(),
                end: end1.clone(),
                user_id: user1,
                timestamp: time1,
            }
        } else {
            // op2 overlaps with op1, adjust the range
            // This is a simplified implementation - a full implementation would be more complex
            Operation::Delete {
                start: start1.clone(),
                end: end1.clone(),
                user_id: user1,
                timestamp: time1,
            }
        };
        
        // Record transformation
        let original_op = Operation::Delete {
            start: start1.clone(),
            end: end1.clone(),
            user_id: user1,
            timestamp: time1,
        };
        
        self.record_transformation(
            conflict_id,
            original_op,
            transformed_op.clone(),
            "delete_vs_delete".to_string(),
        );
        
        Ok(transformed_op)
    }
    
    /// Check if a position is after a range
    fn position_after_range(&self, pos: &Position, range_end: &Position) -> bool {
        pos.line > range_end.line ||
        (pos.line == range_end.line && pos.column >= range_end.column)
    }

    /// Resolve a conflict using the specified strategy
    ///
    /// Applies the resolution strategy defined in the conflict to resolve it.
    ///
    /// # Arguments
    /// * `conflict_id` - The UUID of the conflict to resolve
    ///
    /// # Returns
    /// Ok(()) if the conflict was resolved successfully, or an error if resolution failed
    pub fn resolve_conflict(&mut self, conflict_id: Uuid) -> Result<(), CollaborationError> {
        if let Some(conflict) = self.conflicts.get_mut(&conflict_id) {
            if conflict.resolved {
                return Ok(());
            }
            
            let resolved_operations = match conflict.resolution_strategy {
                ResolutionStrategy::TimestampOrder => {
                    self.resolve_by_timestamp_order(&conflict.conflicting_operations)?
                },
                ResolutionStrategy::UserPriority => {
                    self.resolve_by_user_priority(&conflict.conflicting_operations)?
                },
                ResolutionStrategy::Merge => {
                    self.resolve_by_merge(&conflict.conflicting_operations)?
                },
                ResolutionStrategy::Manual => {
                    // Manual resolution required
                    // In a real implementation, this would notify users
                    conflict.conflicting_operations.clone()
                },
            };
            
            conflict.resolved_operations = resolved_operations;
            conflict.resolved = true;
            conflict.resolved_at = Some(Utc::now());
            
            // Create a new version after conflict resolution if version manager is available
            if let Some(ref mut version_manager) = self.version_manager {
                let version = crate::versioning::DocumentVersion {
                    id: Uuid::new_v4(),
                    document_id: self.document_id,
                    version_number: 0, // This should be set properly in a real implementation
                    content: self.document_content.clone(),
                    operations: resolved_operations.clone(),
                    author_id: Uuid::nil(), // "system" user
                    author_name: "system".to_string(),
                    created_at: Utc::now(),
                    commit_message: Some("Conflict resolution".to_string()),
                    conflict_metadata: Some(serde_json::json!({
                        "conflict_id": conflict_id,
                        "resolution_strategy": conflict.resolution_strategy,
                        "resolved_operations": conflict.resolved_operations
                    })),
                };
                version_manager.versions.insert(0, version); // This should use proper version numbering
            }
            
            // Publish conflict resolved event if event bus is available
            if let Some(ref event_bus) = self.event_bus {
                let event = DomainEvent::new_local(
                    "collaboration".to_string(),
                    "ConflictResolved".to_string(),
                    json!({
                        "document_id": self.document_id,
                        "conflict_id": conflict_id,
                        "resolved_operations": conflict.resolved_operations,
                    }),
                );
                
                let _ = event_bus.publish(event);
            }
            
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(CollaborationError::OperationConflict)
        }
    }
    
    /// Resolve conflicts by timestamp order
    fn resolve_by_timestamp_order(&mut self, operations: &[Operation]) -> Result<Vec<Operation>, CollaborationError> {
        let mut sorted_ops = operations.to_vec();
        sorted_ops.sort_by(|a, b| {
            let timestamp_a = match a {
                Operation::Insert { timestamp, .. } => timestamp,
                Operation::Delete { timestamp, .. } => timestamp,
                Operation::Replace { timestamp, .. } => timestamp,
            };
            
            let timestamp_b = match b {
                Operation::Insert { timestamp, .. } => timestamp,
                Operation::Delete { timestamp, .. } => timestamp,
                Operation::Replace { timestamp, .. } => timestamp,
            };
            
            timestamp_a.cmp(timestamp_b)
        });
        
        // Transform operations to ensure consistency
        let mut resolved_ops = Vec::new();
        for op in sorted_ops {
            let mut transformed_op = op.clone();
            for resolved_op in &resolved_ops {
                transformed_op = self.transform_operation(&transformed_op, resolved_op)?;
            }
            resolved_ops.push(transformed_op);
        }
        
        Ok(resolved_ops)
    }
    
    /// Resolve conflicts by user priority
    fn resolve_by_user_priority(&mut self, operations: &[Operation]) -> Result<Vec<Operation>, CollaborationError> {
        let mut sorted_ops = operations.to_vec();
        sorted_ops.sort_by(|a, b| {
            let user_a = match a {
                Operation::Insert { user_id, .. } => user_id,
                Operation::Delete { user_id, .. } => user_id,
                Operation::Replace { user_id, .. } => user_id,
            };
            
            let user_b = match b {
                Operation::Insert { user_id, .. } => user_id,
                Operation::Delete { user_id, .. } => user_id,
                Operation::Replace { user_id, .. } => user_id,
            };
            
            let priority_a = self.get_user_priority(*user_a);
            let priority_b = self.get_user_priority(*user_b);
            
            // Higher priority first
            priority_b.cmp(&priority_a)
        });
        
        // Transform operations to ensure consistency
        let mut resolved_ops = Vec::new();
        for op in sorted_ops {
            let mut transformed_op = op.clone();
            for resolved_op in &resolved_ops {
                transformed_op = self.transform_operation(&transformed_op, resolved_op)?;
            }
            resolved_ops.push(transformed_op);
        }
        
        Ok(resolved_ops)
    }
    
    /// Resolve conflicts by merging operations when possible
    fn resolve_by_merge(&mut self, operations: &[Operation]) -> Result<Vec<Operation>, CollaborationError> {
        let mut resolved_ops = operations.to_vec();
        // Sort operations by position (line then column)
        resolved_ops.sort_by(|a, b| {
            let pos_a = self.get_operation_start(a);
            let pos_b = self.get_operation_start(b);
            pos_a.line.cmp(&pos_b.line)
                .then(pos_a.column.cmp(&pos_b.column))
        });
        
        // Apply transformations in sorted order
        for i in 0..resolved_ops.len() {
            for j in 0..i {
                resolved_ops[i] = self.transform_operation(&resolved_ops[i], &resolved_ops[j])?;
            }
        }
        
        Ok(resolved_ops)
    }
    
    /// Get the start position of an operation
    fn get_operation_start(&self, operation: &Operation) -> Position {
        match operation {
            Operation::Insert { position, .. } => position.clone(),
            Operation::Delete { start, .. } => start.clone(),
            Operation::Replace { start, .. } => start.clone(),
        }
    }


    /// Get unresolved conflicts
    pub fn get_unresolved_conflicts(&self) -> Vec<Conflict> {
        self.conflicts
            .values()
            .filter(|conflict| !conflict.resolved)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    #[test]
    fn test_conflict_resolver_creation() {
        let document_id = Uuid::new_v4();
        let resolver = ConflictResolver::new(document_id);
        
        assert_eq!(resolver.document_id, document_id);
        assert_eq!(resolver.conflicts.len(), 0);
        assert_eq!(resolver.user_priorities.len(), 0);
    }
    
    #[test]
    fn test_user_priority_setting() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user_id = Uuid::new_v4();
        
        resolver.set_user_priority(user_id, 10);
        assert_eq!(resolver.get_user_priority(user_id), 10);
    }
    
    #[test]
    fn test_conflict_detection() {
        let document_id = Uuid::new_v4();
        let resolver = ConflictResolver::new(document_id);
        let user_id = Uuid::new_v4();
        let timestamp = Utc::now();
        
        // Create two operations at the same position
        let op1 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "Hello".to_string(),
            user_id,
            timestamp,
        };
        
        let op2 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "World".to_string(),
            user_id,
            timestamp,
        };
        
        let operations = vec![op1, op2];
        let conflicts = resolver.detect_conflicts(&operations);
        
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflicting_operations.len(), 2);
    }
    
    #[test]
    fn test_insert_vs_insert_transformation() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
        
        // User 1 inserts "Hello" at position (0,0)
        let pos1 = Position { line: 0, column: 0 };
        let text1 = "Hello";
        
        // User 2 inserts "Hi" at position (0,0)
        let pos2 = Position { line: 0, column: 0 };
        let text2 = "Hi";
        
        let result = resolver.transform_insert_vs_insert(Uuid::nil(), &pos1, text1, user1_id, timestamp1, &pos2, user2_id, timestamp2);
        assert!(result.is_ok());
        
        let transformed_op = result.unwrap();
        match transformed_op {
            Operation::Insert { position, text, .. } => {
                // Since user 2's operation comes after user 1's, user 1's position should be adjusted
                assert_eq!(position.line, 0);
                assert_eq!(position.column, 0 + text2.chars().count());
                assert_eq!(text, text1);
            },
            _ => panic!("Expected Insert operation"),
        }
    }
    
    #[test]
    fn test_insert_vs_delete_transformation() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
        
        // User 1 inserts "World" at (0,5)
        let pos1 = Position { line: 0, column: 5 };
        let text1 = "World";
        
        // User 2 deletes range (0,0) to (0,5)
        let start2 = Position { line: 0, column: 0 };
        let end2 = Position { line: 0, column: 5 };
        
        let result = resolver.transform_insert_vs_delete(Uuid::nil(), &pos1, text1, user1_id, timestamp1, &start2, &end2, user2_id, timestamp2);
        assert!(result.is_ok());
        
        let transformed_op = result.unwrap();
        match transformed_op {
            Operation::Insert { position, .. } => {
                // Insert should move to (0,0) after deletion
                assert_eq!(position.line, 0);
                assert_eq!(position.column, 0);
            },
            _ => panic!("Expected Insert operation"),
        }
    }
    
    #[test]
    fn test_delete_vs_insert_transformation() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
        
        // User 1 deletes range (0,0) to (0,5)
        let start1 = Position { line: 0, column: 0 };
        let end1 = Position { line: 0, column: 5 };
        
        // User 2 inserts "Hi" at position (0,2)
        let pos2 = Position { line: 0, column: 2 };
        let text2 = "Hi";
        
        let result = resolver.transform_delete_vs_insert(Uuid::nil(), &start1, &end1, user1_id, timestamp1, &pos2, text2, user2_id, timestamp2);
        assert!(result.is_ok());
        
        let transformed_op = result.unwrap();
        match transformed_op {
            Operation::Delete { start, end, .. } => {
                // Delete range should be extended to account for the insert
                assert_eq!(start.line, 0);
                assert_eq!(start.column, 0);
                assert_eq!(end.line, 0);
                assert_eq!(end.column, 5 + text2.chars().count());
            },
            _ => panic!("Expected Delete operation"),
        }
    }
    
    #[test]
    fn test_resolve_conflict_by_timestamp() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
        
        // Create conflicting operations
        let op1 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "A".to_string(),
            user_id: user1_id,
            timestamp: timestamp1,
        };
        
        let op2 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "B".to_string(),
            user_id: user2_id,
            timestamp: timestamp2,
        };
        
        let conflict = Conflict {
            id: Uuid::new_v4(),
            document_id,
            conflicting_operations: vec![op1, op2],
            resolution_strategy: ResolutionStrategy::TimestampOrder,
            resolved: false,
            resolved_operations: vec![],
            resolved_at: None,
            created_at: Utc::now(),
            metadata: ConflictMetadata {
                detection_method: "test".to_string(),
                transformation_history: vec![],
                resolution_details: None,
            },
        };
        
        resolver.add_conflict(conflict.clone());
        assert!(resolver.resolve_conflict(conflict.id).is_ok());
        
        let resolved_conflict = resolver.conflicts.get(&conflict.id).unwrap();
        assert!(resolved_conflict.resolved);
        assert!(!resolved_conflict.resolved_operations.is_empty());
    }
    
    #[test]
    fn test_resolve_conflict_by_user_priority() {
        let document_id = Uuid::new_v4();
        let mut resolver = ConflictResolver::new(document_id);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
        
        // Set user priorities
        resolver.set_user_priority(user1_id, 10); // Higher priority
        resolver.set_user_priority(user2_id, 5);  // Lower priority
        
        // Create conflicting operations
        let op1 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "HighPriority".to_string(),
            user_id: user1_id,
            timestamp: timestamp1,
        };
        
        let op2 = Operation::Insert {
            position: Position { line: 0, column: 0 },
            text: "LowPriority".to_string(),
            user_id: user2_id,
            timestamp: timestamp2,
        };
        
        let conflict = Conflict {
            id: Uuid::new_v4(),
            document_id,
            conflicting_operations: vec![op1, op2],
            resolution_strategy: ResolutionStrategy::UserPriority,
            resolved: false,
            resolved_operations: vec![],
            resolved_at: None,
            created_at: Utc::now(),
            metadata: ConflictMetadata {
                detection_method: "test".to_string(),
                transformation_history: vec![],
                resolution_details: None,
            },
        };
        
        resolver.add_conflict(conflict.clone());
        assert!(resolver.resolve_conflict(conflict.id).is_ok());
        
        let resolved_conflict = resolver.conflicts.get(&conflict.id).unwrap();
        assert!(resolved_conflict.resolved);
        assert!(!resolved_conflict.resolved_operations.is_empty());
    }
}
//! Mobile storage adapter for Finance-Sheets
//!
//! This module provides a unified interface for mobile storage that works
//! across different mobile platforms (Android, iOS, Web).

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Error types for storage operations
#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    /// Failed to save data
    SaveFailed(String),
    
    /// Failed to load data
    LoadFailed(String),
    
    /// Data not found
    NotFound(String),
    
    /// Storage not available
    NotAvailable,
}

/// Trait for storage adapters
pub trait StorageAdapter {
    /// Save sheet data to storage
    fn save_sheet(&self, sheet_id: &str, data: &SheetData) -> Result<(), StorageError>;
    
    /// Load sheet data from storage
    fn load_sheet(&self, sheet_id: &str) -> Result<SheetData, StorageError>;
    
    /// Check if there are pending changes
    fn has_pending_changes(&self) -> bool;
    
    /// Get pending changes
    fn get_pending_changes(&self) -> Vec<ChangeRecord>;
    
    /// Clear pending changes
    fn clear_pending_changes(&self);
}

/// Data structure for sheet data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SheetData {
    /// Sheet identifier
    pub id: String,
    
    /// Sheet name
    pub name: String,
    
    /// Sheet content as JSON string
    pub content: String,
    
    /// Last modified timestamp
    pub last_modified: u64,
}

/// Record of a change for sync purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeRecord {
    /// Unique identifier for the change
    pub id: String,
    
    /// Sheet identifier
    pub sheet_id: String,
    
    /// Type of change
    pub change_type: ChangeType,
    
    /// Change data as JSON string
    pub data: String,
    
    /// Timestamp of the change
    pub timestamp: u64,
}

/// Types of changes that can be recorded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    /// Cell value changed
    CellValueChanged,
    
    /// New row added
    RowAdded,
    
    /// Row deleted
    RowDeleted,
    
    /// New column added
    ColumnAdded,
    
    /// Column deleted
    ColumnDeleted,
    
    /// Sheet properties changed
    SheetPropertiesChanged,
}

/// Android storage adapter implementation
#[cfg(target_os = "android")]
pub struct AndroidStorageAdapter {
    /// In-memory storage for demonstration
    /// In a real implementation, this would use Android-specific storage APIs
    storage: HashMap<String, SheetData>,
    pending_changes: Vec<ChangeRecord>,
}

#[cfg(target_os = "android")]
impl AndroidStorageAdapter {
    /// Create a new Android storage adapter
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            pending_changes: Vec::new(),
        }
    }
}

#[cfg(target_os = "android")]
impl StorageAdapter for AndroidStorageAdapter {
    fn save_sheet(&self, sheet_id: &str, data: &SheetData) -> Result<(), StorageError> {
        // In a real implementation, this would use Android storage APIs
        // For now, we'll just simulate the operation
        Ok(())
    }
    
    fn load_sheet(&self, sheet_id: &str) -> Result<SheetData, StorageError> {
        // In a real implementation, this would load from Android storage
        // For now, we'll just simulate the operation
        Err(StorageError::NotFound("Sheet not found".to_string()))
    }
    
    fn has_pending_changes(&self) -> bool {
        !self.pending_changes.is_empty()
    }
    
    fn get_pending_changes(&self) -> Vec<ChangeRecord> {
        self.pending_changes.clone()
    }
    
    fn clear_pending_changes(&self) {
        self.pending_changes.clear();
    }
}

/// Web storage adapter implementation (for web-based mobile)
#[cfg(target_arch = "wasm32")]
pub struct WebStorageAdapter {
    /// In-memory storage for demonstration
    /// In a real implementation, this would use IndexedDB or localStorage
    storage: HashMap<String, SheetData>,
    pending_changes: Vec<ChangeRecord>,
}

#[cfg(target_arch = "wasm32")]
impl WebStorageAdapter {
    /// Create a new web storage adapter
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            pending_changes: Vec::new(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl StorageAdapter for WebStorageAdapter {
    fn save_sheet(&self, sheet_id: &str, data: &SheetData) -> Result<(), StorageError> {
        // In a real implementation, this would use IndexedDB or localStorage
        // For now, we'll just simulate the operation
        Ok(())
    }
    
    fn load_sheet(&self, sheet_id: &str) -> Result<SheetData, StorageError> {
        // In a real implementation, this would load from IndexedDB or localStorage
        // For now, we'll just simulate the operation
        Err(StorageError::NotFound("Sheet not found".to_string()))
    }
    
    fn has_pending_changes(&self) -> bool {
        !self.pending_changes.is_empty()
    }
    
    fn get_pending_changes(&self) -> Vec<ChangeRecord> {
        self.pending_changes.clone()
    }
    
    fn clear_pending_changes(&self) {
        self.pending_changes.clear();
    }
}

/// Desktop storage adapter implementation (fallback)
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub struct DesktopStorageAdapter {
    /// In-memory storage for demonstration
    storage: HashMap<String, SheetData>,
    pending_changes: Vec<ChangeRecord>,
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
impl DesktopStorageAdapter {
    /// Create a new desktop storage adapter
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            pending_changes: Vec::new(),
        }
    }
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
impl StorageAdapter for DesktopStorageAdapter {
    fn save_sheet(&self, sheet_id: &str, data: &SheetData) -> Result<(), StorageError> {
        // Desktop implementation would typically use file system
        Ok(())
    }
    
    fn load_sheet(&self, sheet_id: &str) -> Result<SheetData, StorageError> {
        Err(StorageError::NotFound("Sheet not found".to_string()))
    }
    
    fn has_pending_changes(&self) -> bool {
        !self.pending_changes.is_empty()
    }
    
    fn get_pending_changes(&self) -> Vec<ChangeRecord> {
        self.pending_changes.clone()
    }
    
    fn clear_pending_changes(&self) {
        self.pending_changes.clear();
    }
}
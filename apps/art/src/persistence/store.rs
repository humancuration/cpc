//! Persistence layer for the Art application
//!
//! This module provides storage functionality using Sled as the backend,
//! allowing for local project saving/loading and preferences storage.

use sled::Db;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::core::models::{Project, Layer, Brush};
use std::path::Path;
use std::fs;

/// Error types for persistence operations
#[derive(Debug)]
pub enum PersistenceError {
    SledError(sled::Error),
    SerializationError(String),
    ProjectNotFound,
    IoError(std::io::Error),
}

impl From<sled::Error> for PersistenceError {
    fn from(err: sled::Error) -> Self {
        PersistenceError::SledError(err)
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(err: std::io::Error) -> Self {
        PersistenceError::IoError(err)
    }
}

/// Storage adapter for projects using Sled
pub struct ProjectStore {
    db: Db,
}

impl ProjectStore {
    /// Create a new project store
    pub fn new(db_path: &str) -> Result<Self, PersistenceError> {
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }
    
    /// Save a project to the database
    pub fn save_project(&self, project: &Project) -> Result<(), PersistenceError> {
        let key = project.id.to_string();
        let serialized = serde_json::to_vec(project)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        
        self.db.insert(key.as_bytes(), serialized)?;
        self.db.flush()?;
        
        Ok(())
    }
    
    /// Load a project from the database
    pub fn load_project(&self, project_id: Uuid) -> Result<Project, PersistenceError> {
        let key = project_id.to_string();
        
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let project: Project = serde_json::from_slice(&bytes)
                    .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
                Ok(project)
            }
            None => Err(PersistenceError::ProjectNotFound),
        }
    }
    
    /// List all projects in the database
    pub fn list_projects(&self) -> Result<Vec<Uuid>, PersistenceError> {
        let mut project_ids = Vec::new();
        
        for result in self.db.iter() {
            let (key, _) = result?;
            if let Ok(key_str) = std::str::from_utf8(&key) {
                if let Ok(uuid) = Uuid::parse_str(key_str) {
                    project_ids.push(uuid);
                }
            }
        }
        
        Ok(project_ids)
    }
    
    /// Delete a project from the database
    pub fn delete_project(&self, project_id: Uuid) -> Result<(), PersistenceError> {
        let key = project_id.to_string();
        self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
}

/// Refactor away from ffmpeg dependency for image import/export
pub struct ImageIO;

impl ImageIO {
    /// Import an image from a file
    pub fn import_image(path: &Path) -> Result<Vec<u8>, PersistenceError> {
        // In a real implementation, we would use ffmpeg to decode the image
        // For now, we'll just read the file as bytes
        let data = fs::read(path)?;
        Ok(data)
    }
    
    /// Export image data to a file in PNG format
    pub fn export_png(data: &[u8], path: &Path) -> Result<(), PersistenceError> {
        // In a real implementation, we would use ffmpeg to encode the image
        // For now, we'll just write the data to a file
        fs::write(path, data)?;
        Ok(())
    }
    
    /// Export image data to a file in WebP format
    pub fn export_webp(data: &[u8], path: &Path) -> Result<(), PersistenceError> {
        // In a real implementation, we would use ffmpeg to encode the image
        // For now, we'll just write the data to a file
        fs::write(path, data)?;
        Ok(())
    }
}

/// Preferences storage for user settings
pub struct PreferencesStore {
    db: Db,
}

impl PreferencesStore {
    /// Create a new preferences store
    pub fn new(db_path: &str) -> Result<Self, PersistenceError> {
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }
    
    /// Save a brush preset
    pub fn save_brush_preset(&self, brush: &Brush) -> Result<(), PersistenceError> {
        let key = format!("brush_{}", brush.id);
        let serialized = serde_json::to_vec(brush)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        
        self.db.insert(key.as_bytes(), serialized)?;
        self.db.flush()?;
        
        Ok(())
    }
    
    /// Load a brush preset
    pub fn load_brush_preset(&self, brush_id: Uuid) -> Result<Brush, PersistenceError> {
        let key = format!("brush_{}", brush_id);
        
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let brush: Brush = serde_json::from_slice(&bytes)
                    .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
                Ok(brush)
            }
            None => Err(PersistenceError::ProjectNotFound),
        }
    }
    
    /// List all brush presets
    pub fn list_brush_presets(&self) -> Result<Vec<Uuid>, PersistenceError> {
        let mut brush_ids = Vec::new();
        
        for result in self.db.iter() {
            let (key, _) = result?;
            if let Ok(key_str) = std::str::from_utf8(&key) {
                if key_str.starts_with("brush_") {
                    if let Some(id_str) = key_str.strip_prefix("brush_") {
                        if let Ok(uuid) = Uuid::parse_str(id_str) {
                            brush_ids.push(uuid);
                        }
                    }
                }
            }
        }
        
        Ok(brush_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::core::models::{LayerType};
    
    #[test]
    fn test_project_store() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        let store = ProjectStore::new(db_path.to_str().unwrap()).expect("Failed to create store");
        
        // Create a test project
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        let layer = Layer::new("Test Layer".to_string(), 800, 600, LayerType::Raster);
        project.add_layer(layer);
        
        // Save the project
        let project_id = project.id;
        store.save_project(&project).expect("Failed to save project");
        
        // List projects
        let projects = store.list_projects().expect("Failed to list projects");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0], project_id);
        
        // Load the project
        let loaded_project = store.load_project(project_id).expect("Failed to load project");
        assert_eq!(loaded_project.name, "Test Project");
        assert_eq!(loaded_project.width, 800);
        assert_eq!(loaded_project.height, 600);
        assert_eq!(loaded_project.layers.len(), 1);
        
        // Delete the project
        store.delete_project(project_id).expect("Failed to delete project");
        let projects = store.list_projects().expect("Failed to list projects");
        assert_eq!(projects.len(), 0);
    }
    
    #[test]
    fn test_preferences_store() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("prefs.db");
        let store = PreferencesStore::new(db_path.to_str().unwrap()).expect("Failed to create store");
        
        // Create a test brush
        let mut brush = Brush::new("Test Brush".to_string());
        brush.set_size(20.0);
        brush.set_hardness(0.8);
        brush.set_opacity(0.9);
        
        // Save the brush preset
        let brush_id = brush.id;
        store.save_brush_preset(&brush).expect("Failed to save brush preset");
        
        // List brush presets
        let brushes = store.list_brush_presets().expect("Failed to list brush presets");
        assert_eq!(brushes.len(), 1);
        assert_eq!(brushes[0], brush_id);
        
        // Load the brush preset
        let loaded_brush = store.load_brush_preset(brush_id).expect("Failed to load brush preset");
        assert_eq!(loaded_brush.name, "Test Brush");
        assert_eq!(loaded_brush.size, 20.0);
        assert_eq!(loaded_brush.hardness, 0.8);
        assert_eq!(loaded_brush.opacity, 0.9);
    }
}
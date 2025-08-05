//! Integration tests for the Art application
//!
//! These tests verify that all components work together correctly.

use art::core::models::{Project, Layer, LayerType, Brush};
use art::core::undo::HistoryManager;
use art::persistence::store::{ProjectStore, PreferencesStore};
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn test_full_workflow() {
    // Create a temporary directory for the database
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test.db");
    
    // Create project store
    let store = ProjectStore::new(db_path.to_str().unwrap()).expect("Failed to create store");
    
    // Create a new project
    let mut project = Project::new("Test Project".to_string(), 800, 600);
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.width, 800);
    assert_eq!(project.height, 600);
    
    // Add a layer
    let layer = Layer::new("Background".to_string(), 800, 600, LayerType::Raster);
    let layer_id = layer.id;
    project.add_layer(layer);
    assert_eq!(project.layers.len(), 1);
    
    // Create a brush
    let mut brush = Brush::new("Test Brush".to_string());
    brush.set_size(20.0);
    brush.set_hardness(0.8);
    brush.set_opacity(0.9);
    
    // Create history manager
    let mut history = HistoryManager::new(10);
    assert!(!history.can_undo());
    assert!(!history.can_redo());
    
    // Save the project
    store.save_project(&project).expect("Failed to save project");
    
    // List projects
    let projects = store.list_projects().expect("Failed to list projects");
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0], project.id);
    
    // Load the project
    let loaded_project = store.load_project(project.id).expect("Failed to load project");
    assert_eq!(loaded_project.name, "Test Project");
    assert_eq!(loaded_project.layers.len(), 1);
    
    // Test preferences store
    let prefs_db_path = temp_dir.path().join("prefs.db");
    let prefs_store = PreferencesStore::new(prefs_db_path.to_str().unwrap()).expect("Failed to create prefs store");
    
    // Save brush preset
    prefs_store.save_brush_preset(&brush).expect("Failed to save brush preset");
    
    // List brush presets
    let brushes = prefs_store.list_brush_presets().expect("Failed to list brush presets");
    assert_eq!(brushes.len(), 1);
    assert_eq!(brushes[0], brush.id);
    
    // Load brush preset
    let loaded_brush = prefs_store.load_brush_preset(brush.id).expect("Failed to load brush preset");
    assert_eq!(loaded_brush.name, "Test Brush");
    assert_eq!(loaded_brush.size, 20.0);
    
    // Test undo/redo
    let layer2 = Layer::new("Layer 2".to_string(), 800, 600, LayerType::Raster);
    let layer2_id = layer2.id;
    
    // Add to history
    history.add_action(art::core::models::Action::LayerAdded { layer: layer2.clone() });
    assert!(history.can_undo());
    assert!(!history.can_redo());
    
    // Add layer to project
    project.add_layer(layer2);
    assert_eq!(project.layers.len(), 2);
    
    // Undo the action
    history.undo(&mut project).expect("Failed to undo");
    assert!(!history.can_undo());
    assert!(history.can_redo());
    assert_eq!(project.layers.len(), 1);
    
    // Redo the action
    history.redo(&mut project).expect("Failed to redo");
    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(project.layers.len(), 2);
}
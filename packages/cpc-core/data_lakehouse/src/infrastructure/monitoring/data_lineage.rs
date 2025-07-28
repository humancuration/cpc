//! Data lineage tracking implementation

use crate::domain::models::{DataLineage, DataAsset};
use uuid::Uuid;
use std::collections::HashMap;

/// Data lineage tracker
pub struct DataLineageTracker {
    lineages: HashMap<Uuid, DataLineage>,
}

impl DataLineageTracker {
    pub fn new() -> Self {
        Self {
            lineages: HashMap::new(),
        }
    }

    /// Record the lineage of a data asset
    pub fn record_lineage(&mut self, asset_id: Uuid, lineage: DataLineage) {
        self.lineages.insert(asset_id, lineage);
    }

    /// Get the lineage of a data asset
    pub fn get_lineage(&self, asset_id: Uuid) -> Option<&DataLineage> {
        self.lineages.get(&asset_id)
    }

    /// Track a transformation from source to target asset
    pub fn track_transformation(&mut self, source_id: Uuid, target_id: Uuid, transformation: &str) {
        // Add source to target's lineage
        let lineage = self.lineages.entry(target_id).or_insert_with(DataLineage::new);
        if !lineage.sources.contains(&source_id) {
            lineage.sources.push(source_id);
        }
        
        if !lineage.transformations.contains(&transformation.to_string()) {
            lineage.transformations.push(transformation.to_string());
        }
    }
}
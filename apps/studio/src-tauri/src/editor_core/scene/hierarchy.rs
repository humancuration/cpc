use uuid::Uuid;
use std::collections::{HashMap, HashSet};

pub struct HierarchyGraph {
    pub parent_to_children: HashMap<Option<Uuid>, HashSet<Uuid>>,
    pub child_to_parent: HashMap<Uuid, Option<Uuid>>,
}

impl HierarchyGraph {
    pub fn new() -> Self {
        HierarchyGraph {
            parent_to_children: HashMap::new(),
            child_to_parent: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity_id: Uuid, parent_id: Option<Uuid>) {
        // Add parent-child relationship
        self.parent_to_children
            .entry(parent_id)
            .or_insert_with(HashSet::new)
            .insert(entity_id);

        // Add child-parent relationship
        self.child_to_parent.insert(entity_id, parent_id);
    }

    pub fn remove_entity(&mut self, entity_id: Uuid) {
        if let Some(parent_id) = self.child_to_parent.remove(&entity_id) {
            // Remove from parent's children
            if let Some(children) = self.parent_to_children.get_mut(&parent_id) {
                children.remove(&entity_id);
            }
        }

        // Remove all children of this entity
        if let Some(children) = self.parent_to_children.remove(&Some(entity_id)) {
            for child_id in children {
                self.child_to_parent.remove(&child_id);
            }
        }
    }

    pub fn reparent_entity(&mut self, entity_id: Uuid, new_parent_id: Option<Uuid>) {
        self.remove_entity(entity_id);
        self.add_entity(entity_id, new_parent_id);
    }

    pub fn get_children(&self, entity_id: Option<Uuid>) -> Option<&HashSet<Uuid>> {
        self.parent_to_children.get(&entity_id)
    }

    pub fn get_parent(&self, entity_id: Uuid) -> Option<Option<Uuid>> {
        self.child_to_parent.get(&entity_id).copied()
    }
}
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use crate::editor_core::scene::entity::Entity;
use crate::editor_core::scene::command::Command;
use crate::editor_core::scene::hierarchy::HierarchyGraph;

pub struct SceneManager {
    pub entities: HashMap<Uuid, Entity>,
    pub hierarchy: HierarchyGraph,
    undo_stack: VecDeque<Box<dyn Command>>,
    redo_stack: VecDeque<Box<dyn Command>>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            entities: HashMap::new(),
            hierarchy: HierarchyGraph::new(),
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn get_entity(&self, id: Uuid) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: Uuid) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: Uuid) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn execute_command(&mut self, command: Box<dyn Command>) {
        command.execute(self);
        self.undo_stack.push_back(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(command) = self.undo_stack.pop_back() {
            command.undo(self);
            self.redo_stack.push_back(command);
        }
    }

    pub fn redo(&mut self) {
        if let Some(command) = self.redo_stack.pop_back() {
            command.execute(self);
            self.undo_stack.push_back(command);
        }
    }
}